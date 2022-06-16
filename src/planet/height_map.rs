use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use std::f32::consts::{FRAC_PI_2, PI};

const SAMPLE_DIST: f32 = 0.001;
pub const HEIGHT_SCALLING: f32 = 10.0;

#[derive(Debug, TypeUuid)]
#[uuid = "f8a947d6-7b52-4707-bb6c-9c295c9ef3dd"]
pub struct HeightMap {
    width: usize,
    height: usize,
    data: Vec<i16>,
}

struct Neighboors {
    bottom_left: f32,
    bottom_right: f32,
    top_left: f32,
    top_right: f32,
    relative_lat: f32,
    relative_lon: f32,
}

impl Neighboors {
    fn bilinear_interpolation(&self) -> f32 {
        let bottom_interpolation =
            Self::linear_interpolation(self.bottom_left, self.bottom_right, self.relative_lon);
        let top_interpolation =
            Self::linear_interpolation(self.top_left, self.top_right, self.relative_lon);

        Self::linear_interpolation(bottom_interpolation, top_interpolation, self.relative_lat)
    }

    fn linear_interpolation(v1: f32, v2: f32, t: f32) -> f32 {
        v1 * t + (1.0 - t) * v2
    }

    fn nearest(&self) -> f32 {
        if self.relative_lat < 0.5 {
            self.nearest_horizontal(self.bottom_left, self.bottom_right)
        } else {
            self.nearest_horizontal(self.top_left, self.top_right)
        }
    }

    fn nearest_horizontal(&self, left: f32, right: f32) -> f32 {
        if self.relative_lon < 0.5 {
            left
        } else {
            right
        }
    }
}

impl HeightMap {
    pub fn fetch_relief_at(&self, normalized_position: Vec3, radius: f32) -> (f32, Vec3) {
        let height = self.get_height_at(normalized_position);
        let normal = self.get_norm((radius + height) * normalized_position, radius);
        (height, normal)
    }

    pub fn get_height_at(&self, normalized_position: Vec3) -> f32 {
        let (longitude, latitude) = self.get_spherical_coord(normalized_position);
        let neghboors = self.get_neighboors(longitude, latitude);

        HEIGHT_SCALLING * neghboors.nearest()
    }

    fn get_neighboors(&self, longitude: f32, latitude: f32) -> Neighboors {
        let lo_f = longitude.floor();
        let lo_c = longitude.ceil();
        let la_f = latitude.floor();
        let la_c = latitude.ceil();

        Neighboors {
            bottom_left: self.height_at(la_f as usize, lo_f as usize),
            bottom_right: self.height_at(la_f as usize, lo_c as usize),
            top_left: self.height_at(la_c as usize, lo_f as usize),
            top_right: self.height_at(la_c as usize, lo_c as usize),
            relative_lat: latitude - la_f,
            relative_lon: longitude - lo_f,
        }
    }

    fn get_spherical_coord(&self, normalized_position: Vec3) -> (f32, f32) {
        let longitude = normalized_position.x.atan2(normalized_position.z);
        let latitude = -normalized_position.y.asin();
        let longitude = ((longitude + PI) / (2.0 * PI)) * self.width as f32;
        let latitude = ((latitude + FRAC_PI_2) / PI) * self.height as f32;
        (longitude, latitude)
    }

    fn height_at(&self, row: usize, col: usize) -> f32 {
        let row = row % self.height;
        let col = col.clamp(0, self.width - 1);
        self.data[row * self.width + col] as f32
    }

    fn get_norm(&self, pos: Vec3, radius: f32) -> Vec3 {
        let cross = Vec3::Z.cross(pos.normalize());
        let rotation = if cross.length() > 0.0 {
            let angle = Vec3::Z.dot(pos.normalize()).acos();
            Quat::from_axis_angle(cross.normalize(), angle)
        } else {
            Quat::IDENTITY
        };

        let samples: Vec<Vec3> = vec![Vec3::X, Vec3::Y]
            .iter()
            .map(|v| rotation * *v)
            .map(|v| (pos + v * radius * SAMPLE_DIST).normalize())
            .map(|v| (radius + self.get_height_at(v)) * v)
            .map(|v| v - pos)
            .collect();

        let norm = samples[0].cross(samples[1]).normalize();
        norm
    }
}

#[derive(Default)]
pub struct HeightMapAssetLoder;

impl AssetLoader for HeightMapAssetLoder {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let width = (bytes.len() as f64).sqrt() as usize;
            let height = width / 2;
            let data = bytes
                .chunks(2)
                .map(|chunk| ((chunk[0] as i16) << 8) + chunk[1] as i16)
                .collect();
            info!("Loaded {} x {} height map", width, height);
            let height_map_asset = HeightMap {
                width,
                height,
                data,
            };
            load_context.set_default_asset(LoadedAsset::new(height_map_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["img"]
    }
}
