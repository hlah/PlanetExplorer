mod planet;
mod player;

use bevy::{prelude::*, window::exit_on_window_close_system};
use planet::{
    graphics::PlanetMaterial,
    height_map::*,
    view::{update_color_mode, ColorMode},
    *,
};
use player::*;

const MARS_RADIUS: f32 = 3396000.0;
const MARS_MIN_ALTITUDE: f32 = -8206.0;
const MARS_MAX_ALTITUDE: f32 = 21181.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<PlanetMaterial>::default())
        .add_asset::<HeightMap>()
        .init_asset_loader::<HeightMapAssetLoder>()
        .insert_resource(ColorMode::Altitude)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(AmbientLight {
            brightness: 0.05,
            color: Color::WHITE,
            ..default()
        })
        .add_system(exit_on_window_close_system)
        .add_system(planet_loading_system)
        .add_system(planet_added_system)
        .add_system(player_control)
        .add_system(update_color_mode)
        .add_startup_system(setup)
        .add_startup_system(setup_player)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let height_map_handler = asset_server.load("mars/megt90n000fb.img");
    commands.spawn().insert(Planet::new(
        MARS_RADIUS,
        MARS_MIN_ALTITUDE,
        MARS_MAX_ALTITUDE,
        height_map_handler,
    ));
}
