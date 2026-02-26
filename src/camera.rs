use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::TiledParallaxCamera;

use crate::{car::Car, common::CAMERA_SCALE};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_systems(Startup, setup_camera)
            .add_systems(PostUpdate, (camera_follows_player, camera_zoom_on_player));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, TiledParallaxCamera));
}

fn camera_follows_player(
    q_car: Query<(&Transform, &LinearVelocity), With<Car>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Car>)>,
    time: Res<Time>,
) {
    let Ok((car_transform, car_velocity)) = q_car.single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };
    // Gets the camera follow the player and show more in front as he goes faster
    let translation_target = Vec3::new(
        car_transform.translation.x,
        car_transform.translation.y,
        camera_transform.translation.z,
    ) + car_velocity.extend(0.0) * 0.5;

    camera_transform
        .translation
        .smooth_nudge(&translation_target, 5.0, time.delta_secs());
}

fn camera_zoom_on_player(
    car_query: Query<&LinearVelocity, With<Car>>,
    mut camera_query: Query<&mut Projection, With<Camera>>,
    time: Res<Time>,
) {
    let Ok(camera_projection) = camera_query.single_mut() else {
        return;
    };
    let Ok(car_velocity) = car_query.single() else {
        return;
    };

    if let Projection::Orthographic(ref mut projection) = *camera_projection.into_inner() {
        let scale_target =
            CAMERA_SCALE + (car_velocity.0.length() * 0.0005).clamp(0.0, CAMERA_SCALE * 2.0);

        projection
            .scale
            .smooth_nudge(&scale_target, 3.0, time.delta_secs());
    }
}
