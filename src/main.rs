mod camera;
mod planet;

use bevy::{prelude::*, window::exit_on_window_close_system};

use camera::*;
use planet::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(AmbientLight {
            brightness: 1000.0,
            color: Color::WHITE,
        })
        .add_system(exit_on_window_close_system)
        .add_system(planet_added_system)
        .add_system(camera_control)
        .add_startup_system(setup)
        .add_startup_system(setup_camera)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn().insert(Planet::new(1.0));
}
