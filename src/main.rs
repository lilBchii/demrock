use avian2d::PhysicsPlugins;

// use avian2d::diagnostics::{PhysicsDiagnosticsPlugin, PhysicsEntityDiagnosticsPlugin};
// use avian2d::prelude::{PhysicsDebugPlugin, PhysicsDiagnosticsUiPlugin};
use bevy::app::{App, PluginGroup};
use bevy::camera::ClearColor;
use bevy::color::Color;
use bevy::image::ImagePlugin;
use bevy::prelude::AppExtStates;
use bevy::window::{CursorOptions, Window, WindowPlugin};
use bevy::DefaultPlugins;

use bevy_ecs_tiled::prelude::{TiledPhysicsAvianBackend, TiledPhysicsPlugin};
use bevy_ecs_tiled::tiled::TiledPlugin;

use bevy_enhanced_input::EnhancedInputPlugin;
// use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use camera::*;
use tilemap::*;

use crate::{
    animation::AnimationPlugin,
    car::CarPlugin,
    common::{MultiplayerMode, GAME_NAME},
    gamemodes::GameModePlugin,
    gameplay::GameplayPlugin,
    states::GameState,
    ui::{font::FontPlugin, in_game::InGameUiPlugin, menu::MenuPlugin},
};

mod animation;
mod camera;
mod car;
mod common;
mod gamemodes;
mod gameplay;
mod states;
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
                        decorations: false,
                        skip_taskbar: false,
                        movable_by_window_background: false,
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
        // .add_plugins((
        //     PhysicsDebugPlugin::default(),
        //     PhysicsDiagnosticsPlugin,
        //     PhysicsDiagnosticsUiPlugin,
        //     PhysicsEntityDiagnosticsPlugin,
        // ))
        // .add_plugins((EguiPlugin::default(), WorldInspectorPlugin::new()))
        .insert_state(GameState::Menu)
        .add_plugins((
            FontPlugin,
            CameraPlugin,
            MenuPlugin,
            CarPlugin,
            LevelPlugin,
            GameplayPlugin,
            InGameUiPlugin,
            GameModePlugin,
            AnimationPlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(MultiplayerMode::SinglePlayer)
        .run();
}
