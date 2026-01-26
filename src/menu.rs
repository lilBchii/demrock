use bevy::{
    input_focus::{
        directional_navigation::DirectionalNavigationPlugin, InputDispatchPlugin, InputFocus,
    },
    prelude::*,
};
use bevy_enhanced_input::prelude::*;

use crate::{
    common::AppState,
    ui::{button_style, nav_interaction, UI},
};

pub mod gameover;
pub mod main;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, SubStates)]
#[source(AppState = AppState::StartMenu)]
#[states(scoped_entities)]
pub enum Menu {
    #[default]
    Main,
    Settings,
    Credits,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<Menu>()
            .add_input_context::<UI>()
            .add_plugins((InputDispatchPlugin, DirectionalNavigationPlugin))
            .add_systems(
                Update,
                (button_style, nav_interaction).run_if(not(in_state(AppState::Playing))),
            )
            .add_systems(OnEnter(Menu::Main), main::spawn_main_menu)
            .add_systems(OnEnter(AppState::GameOver), gameover::spawn_main_menu);
    }
}
