use avian2d::prelude::LinearVelocity;
use bevy::{
    asset::RenderAssetUsages,
    camera::{RenderTarget, Viewport},
    prelude::*,
    render::render_resource::{TextureDimension, TextureFormat, TextureUsages},
    window::WindowResized,
};
use bevy_ecs_tiled::prelude::TiledParallaxCamera;

use crate::{
    car::Car,
    common::{MultiplayerMode, CAMERA_SCALE},
    gameplay::PlayerFinished,
    states::{GameState, Menu, PlayingState},
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_systems(OnEnter(GameState::Init), setup_menu_camera)
            .add_systems(OnEnter(PlayingState::End), setup_menu_camera)
            .add_systems(
                Update,
                (
                    camera_follows_player,
                    camera_zoom_on_player,
                    set_camera_viewports,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_observer(set_camera_to_race_over);
    }
}

#[derive(Component)]
pub struct InGameCamera;

#[derive(Component)]
pub struct MenuCamera;

#[derive(Component)]
pub struct CameraPosition {
    pub pos: UVec2,
}

#[derive(Component)]
pub struct FocusOnPlayer(pub Entity);

fn setup_menu_camera(mut commands: Commands) {
    commands.spawn((MenuCamera, Camera2d, DespawnOnEnter(Menu::None)));
}

pub fn setup_game_camera(
    mut commands: Commands,
    car_query: Query<Entity, With<Car>>,
    window: Single<&Window>,
    n_players: Res<MultiplayerMode>,
) {
    let size = compute_camera_viewports(&window, &n_players);
    for (index, car) in car_query.iter().enumerate() {
        let pos = UVec2::new(index as u32 % 2, index as u32 / 2);
        commands.spawn((
            InGameCamera,
            Camera {
                viewport: Some(Viewport {
                    physical_position: pos * size,
                    physical_size: size,
                    ..Default::default()
                }),
                order: index as isize,
                ..Default::default()
            },
            Camera2d,
            CameraPosition { pos },
            TiledParallaxCamera,
            FocusOnPlayer(car),
            DespawnOnExit(PlayingState::Racing),
        ));
    }
}

fn camera_follows_player(
    q_car: Query<(&Transform, &LinearVelocity), With<Car>>,
    camera_query: Query<(&mut Transform, &FocusOnPlayer), (With<InGameCamera>, Without<Car>)>,
    time: Res<Time>,
) {
    for (mut camera_transform, focused_player) in camera_query {
        let Ok((car_transform, car_velocity)) = q_car.get(focused_player.0) else {
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
}

fn camera_zoom_on_player(
    car_query: Query<&LinearVelocity, With<Car>>,
    camera_query: Query<(&mut Projection, &FocusOnPlayer), With<InGameCamera>>,
    time: Res<Time>,
) {
    for (camera_projection, focused_player) in camera_query {
        let Ok(car_velocity) = car_query.get(focused_player.0) else {
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
}

fn set_camera_viewports(
    windows: Query<&Window>,
    mut window_resized_reader: MessageReader<WindowResized>,
    n_players: Res<MultiplayerMode>,
    mut query: Query<(&CameraPosition, &mut Camera), (With<InGameCamera>, Without<MenuCamera>)>,
) {
    for window_resized in window_resized_reader.read() {
        let window = windows.get(window_resized.window).unwrap();
        let size = compute_camera_viewports(window, &n_players);

        for (camera_position, mut camera) in &mut query {
            camera.viewport = Some(Viewport {
                physical_position: camera_position.pos * size,
                physical_size: size,
                ..default()
            });
        }
    }
}

fn set_camera_to_race_over(
    player_finished: On<PlayerFinished>,
    camera_query: Query<
        (&mut RenderTarget, &FocusOnPlayer),
        (With<InGameCamera>, Without<MenuCamera>),
    >,
    mut images: ResMut<Assets<Image>>,
) {
    for (mut render_target, focused_player) in camera_query {
        if focused_player.0 == player_finished.0 {
            let mut image = Image::new_uninit(
                default(),
                TextureDimension::D2,
                TextureFormat::Bgra8UnormSrgb,
                RenderAssetUsages::all(),
            );
            image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT;
            let image_handle = images.add(image);

            *render_target = RenderTarget::Image(image_handle.clone().into());
        }
    }
}

fn compute_camera_viewports(window: &Window, n_players: &MultiplayerMode) -> UVec2 {
    match n_players {
        MultiplayerMode::Single => window.physical_size(),
        MultiplayerMode::Two => UVec2::new(window.physical_width() / 2, window.physical_height()),
        MultiplayerMode::Three | MultiplayerMode::Four => window.physical_size() / 2,
    }
}
