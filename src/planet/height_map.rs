use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use bytemuck::cast_slice;

#[derive(Debug, TypeUuid)]
#[uuid = "f8a947d6-7b52-4707-bb6c-9c295c9ef3dd"]
pub struct HeightMap {
    width: usize,
    height: usize,
    data: Vec<i16>,
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
            let data = cast_slice(bytes).iter().copied().collect();
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
