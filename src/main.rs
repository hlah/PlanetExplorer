mod planet;
mod player;

use bevy::{prelude::*, window::exit_on_window_close_system};

use planet::{height_map::*, *};
use player::*;

const MARS_RADIUS: f32 = 3396000.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_asset::<HeightMap>()
        .init_asset_loader::<HeightMapAssetLoder>()
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
        .add_startup_system(setup)
        .add_startup_system(setup_player)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let height_map_handler = asset_server.load("megt90n000fb.img");

    commands
        .spawn()
        .insert(Planet::new(MARS_RADIUS, height_map_handler));
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::ONE * 4.0 * MARS_RADIUS)
            .looking_at(Vec3::ZERO, Vec3::Y),
        point_light: PointLight {
            color: Color::WHITE,
            range: MARS_RADIUS * 10.0,
            intensity: 10000000000000000.0,
            ..default()
        },
        ..default()
    });
}
