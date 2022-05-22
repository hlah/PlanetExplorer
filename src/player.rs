use crate::MARS_RADIUS;
use bevy::{input::mouse::MouseMotion, prelude::*};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCamera;

const SPEED_FACTOR: f32 = 0.2;
const ZOOM_FACTOR: f32 = 0.1;
const ROTATION_SPEED: f32 = 0.1;

pub fn setup_player(mut commands: Commands) {
    commands
        .spawn()
        .insert(Player)
        .insert_bundle(TransformBundle::from_transform(
            Transform::from_translation(Vec3::new(0.0, 0.0, 4.0 * MARS_RADIUS)),
        ))
        .with_children(|children| {
            children
                .spawn_bundle(PerspectiveCameraBundle::default())
                .insert(PlayerCamera);
        });
}

pub fn player_control(
    keyboard: Res<Input<KeyCode>>,
    mouse_button: Res<Input<MouseButton>>,
    time: Res<Time>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut player: Query<&mut Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
) {
    let mut forward = 0.0;
    let mut side = 0.0;
    let mut zoom = 0.0;
    if keyboard.pressed(KeyCode::Comma) || keyboard.pressed(KeyCode::Up) {
        forward += 1.0;
    }
    if keyboard.pressed(KeyCode::O) || keyboard.pressed(KeyCode::Down) {
        forward -= 1.0;
    }
    if keyboard.pressed(KeyCode::E) || keyboard.pressed(KeyCode::Right) {
        side += 1.0;
    }
    if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left) {
        side -= 1.0;
    }
    if keyboard.pressed(KeyCode::Period) || keyboard.pressed(KeyCode::Space) {
        zoom += 1.0;
    }
    if keyboard.pressed(KeyCode::Semicolon) || keyboard.pressed(KeyCode::LControl) {
        zoom -= 1.0;
    }

    let mouse_delta: Vec2 = mouse_motion.iter().map(|motion| &motion.delta).sum();

    let dt = time.delta_seconds();
    let transform = &mut player.single_mut();
    let surface_distance = transform.translation.length() - MARS_RADIUS;

    if forward != 0.0 {
        let direction = transform.rotation * Vec3::Y;
        let displacement = forward * dt * SPEED_FACTOR * surface_distance * direction;
        transform.translation += displacement;
        transform.look_at(Vec3::ZERO, direction);
    }

    if side != 0.0 {
        let direction = transform.rotation * Vec3::X;
        let up = transform.rotation * Vec3::Y;
        let displacement = side * dt * SPEED_FACTOR * surface_distance * direction;
        transform.translation += displacement;
        transform.look_at(Vec3::ZERO, up);
    }

    if zoom != 0.0 {
        let direction = transform.rotation * Vec3::Z;
        let displacement = zoom * ZOOM_FACTOR * surface_distance * dt * direction;
        transform.translation += displacement;
    }

    if mouse_button.pressed(MouseButton::Right) {
        let mut camera_transform = camera.single_mut();
        camera_transform.rotate(Quat::from_axis_angle(
            Vec3::X,
            dt * mouse_delta.y * ROTATION_SPEED,
        ));

        let axis = transform.rotation * Vec3::Z;
        transform.rotate(Quat::from_axis_angle(
            axis,
            dt * mouse_delta.x * ROTATION_SPEED,
        ));
    }
}
