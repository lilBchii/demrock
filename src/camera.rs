use bevy::prelude::*;

use crate::{
    car::{Car, Velocity},
    common::{CAMERA_SCALE, LEVEL_HEIGHT, LEVEL_WIDTH},
};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn camera_follows_player(
    q_car: Query<(&Transform, &Velocity), With<Car>>,
    mut q_camera: Query<&mut Transform, (With<Camera>, Without<Car>)>,
    time: Res<Time>,
) {
    let (car_transform, car_velocity) = q_car.single().unwrap();
    let mut camera_transform = q_camera.single_mut().unwrap();
    let translation_target = Vec3::new(
        car_transform.translation.x,
        car_transform.translation.y,
        camera_transform.translation.z,
    );

    camera_transform
        .translation
        .smooth_nudge(&translation_target, 10.0, time.delta_secs());

    let scale_target = Vec3::new(
        CAMERA_SCALE + car_velocity.0.x * 0.1,
        CAMERA_SCALE + car_velocity.0.y * 0.1,
        CAMERA_SCALE,
    );

    camera_transform
        .scale
        .smooth_nudge(&scale_target, 3.0, time.delta_secs());
}
