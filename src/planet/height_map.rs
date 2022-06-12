use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use itertools::Itertools;
use ordered_float::OrderedFloat;
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

impl HeightMap {
    pub fn fetch_relief_at(&self, normalized_position: Vec3, radius: f32) -> (f32, Vec3) {
        let height = self.get_height_at(normalized_position);
        let normal = self.get_norm((radius + height) * normalized_position, radius);
        (height, normal)
    }

    pub fn get_height_at(&self, normalized_position: Vec3) -> f32 {
        let (longitude, latitude) = self.get_spherical_coord(normalized_position);
        let neghboors = self.get_neighboors(longitude, latitude);

        HEIGHT_SCALLING * Self::bilinear_interpolation(longitude, latitude, neghboors)
    }

    fn bilinear_interpolation(lon: f32, lat: f32, neghboors: [(f32, f32, f32); 4]) -> f32 {
        let lat_interpolation_down =
            Self::linear_interpolation(neghboors[0].2, neghboors[1].2, lat - neghboors[0].1);
        let lat_interpolation_up =
            Self::linear_interpolation(neghboors[2].2, neghboors[3].2, lat - neghboors[2].1);

        Self::linear_interpolation(
            lat_interpolation_down,
            lat_interpolation_up,
            lon - neghboors[0].0,
        )
    }

    fn linear_interpolation(v1: f32, v2: f32, t: f32) -> f32 {
        v1 * t + (1.0 - t) * v2
    }

    fn get_neighboors(&self, longitude: f32, latitude: f32) -> [(f32, f32, f32); 4] {
        let lo_f = longitude.floor();
        let lo_c = longitude.ceil();
        let la_f = latitude.floor();
        let la_c = latitude.ceil();

        [
            (lo_f, la_f, self.height_at(la_f as usize, lo_f as usize)),
            (lo_f, la_c, self.height_at(la_c as usize, lo_f as usize)),
            (lo_c, la_f, self.height_at(la_f as usize, lo_c as usize)),
            (lo_c, la_c, self.height_at(la_c as usize, lo_c as usize)),
        ]
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
        let samples: Vec<Vec3> = vec![Vec3::X, Vec3::Y, Vec3::Z]
            .iter()
            .map(|v| (pos + *v * radius * SAMPLE_DIST).normalize())
            .sorted_by_key(|v| OrderedFloat::from(pos.angle_between(*v)))
            .skip(1)
            .map(|v| (radius + self.get_height_at(Vec3::new(v.x, v.y, v.z))) * v)
            .map(|v| v - pos)
            .collect();

        let mut norm = samples[0].cross(samples[1]).normalize();
        if norm.angle_between(pos) > std::f32::consts::PI / 2.0 {
            norm = -norm;
        }
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
