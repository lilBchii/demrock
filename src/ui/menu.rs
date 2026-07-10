use bevy::{input_focus::directional_navigation::DirectionalNavigationPlugin, prelude::*};
use bevy_enhanced_input::prelude::*;

use crate::{
    states::Menu,
    ui::{button_style, nav_interaction, UI},
};

pub mod credits;
pub mod game_mode_selection;
pub mod gameover;
pub mod level_selection;
pub mod main;
pub mod race_over;
pub mod settings;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(Menu::Main)
            .add_input_context::<UI>()
            .add_plugins(DirectionalNavigationPlugin)
            .add_systems(
                Update,
                (button_style, nav_interaction).run_if(not(in_state(Menu::None))),
            )
            .add_systems(OnEnter(Menu::Main), main::spawn_main_menu)
            .add_systems(OnEnter(Menu::RaceOver), race_over::spawn_race_over_menu)
            .add_systems(OnEnter(Menu::GameOver), gameover::spawn_gameover_menu)
            .add_systems(OnEnter(Menu::Settings), settings::spawn_settings_menu)
            .add_systems(OnEnter(Menu::Credits), credits::spawn_credits_menu)
            .add_systems(
                OnEnter(Menu::ModeSelection),
                game_mode_selection::spawn_mode_selection_menu,
            )
            .add_systems(
                OnEnter(Menu::LevelSelection),
                level_selection::spawn_level_selection_menu,
            );
    }
}
