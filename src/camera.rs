use bevy::prelude::*;

const ROTATION_X_SPEED: f32 = 2.0;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
        ..default()
    });
}

pub fn camera_control(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    let transform = &mut camera.single_mut();
    if keyboard.pressed(KeyCode::Left) {
        rotate_camera_x(transform, -ROTATION_X_SPEED * time.delta_seconds())
    }
    if keyboard.pressed(KeyCode::Right) {
        rotate_camera_x(transform, ROTATION_X_SPEED * time.delta_seconds())
    }
}

fn rotate_camera_x(transform: &mut Transform, angle: f32) {
    transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(angle))
}
