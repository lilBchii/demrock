use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{car::Car, common::CAMERA_SCALE};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_systems(Startup, setup_camera)
            .add_systems(PostUpdate, camera_follows_player);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn camera_follows_player(
    q_car: Query<(&Transform, &LinearVelocity), With<Car>>,
    mut q_camera: Query<&mut Transform, (With<Camera>, Without<Car>)>,
    time: Res<Time>,
) {
    let Ok((car_transform, car_velocity)) = q_car.single() else {
        return;
    };
    let mut camera_transform = q_camera.single_mut().unwrap();
    // Gets the camera follow the player and show more in front as he goes faster
    let translation_target = Vec3::new(
        car_transform.translation.x,
        car_transform.translation.y,
        camera_transform.translation.z,
    ) + car_velocity.extend(0.0) * 0.5;

    camera_transform
        .translation
        .smooth_nudge(&translation_target, 5.0, time.delta_secs());

    // Zoom out as the player goes faster
    let scale_target = Vec2::splat(CAMERA_SCALE)
        + (car_velocity.0.length() * 0.0005).clamp(0.0, CAMERA_SCALE * 2.0);

    camera_transform
        .scale
        .smooth_nudge(&scale_target.extend(CAMERA_SCALE), 3.0, time.delta_secs());
}
