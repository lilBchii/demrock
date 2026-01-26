use avian2d::PhysicsPlugins;
use bevy::{prelude::*, window::CursorOptions};
use bevy_ecs_tiled::{
    prelude::{TiledPhysicsAvianBackend, TiledPhysicsPlugin},
    tiled::TiledPlugin,
};
use bevy_enhanced_input::EnhancedInputPlugin;
use camera::*;
use tilemap::*;

use crate::{
    car::CarPlugin,
    common::{AppState, MultiplayerMode, GAME_NAME},
    font::FontPlugin,
    menu::MenuPlugin,
};

mod animation;
mod camera;
mod car;
mod common;
mod font;
mod menu;
mod tilemap;
mod ui;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: GAME_NAME.into(),
                        ..Default::default()
                    }),
                    primary_cursor_options: Some(CursorOptions {
                        visible: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins((
            TiledPlugin::default(),
            TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default(),
        ))
        .add_plugins(EnhancedInputPlugin)
        .add_plugins(PhysicsPlugins::default().with_length_unit(5.0))
        .add_plugins((FontPlugin, MenuPlugin, CarPlugin))
        .insert_state(AppState::StartMenu)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(MultiplayerMode::SinglePlayer)
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(AppState::Playing), spawn_demcity_level)
        // .add_systems(start_game.in_set(OnUpdate(AppState::StartMenu)))
        // .add_systems(switch_multiplayer_mode.in_set(OnUpdate(AppState::StartMenu)))
        // .add_systems(despawn_screen::<OnStartMenuScreen>.in_schedule(OnExit(AppState::StartMenu)))
        .add_systems(
            PostUpdate,
            camera_follows_player.run_if(in_state(AppState::Playing)),
        )
        .run();
}
