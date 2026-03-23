use bevy::{
    asset::AssetServer,
    ecs::{
        observer::On,
        system::{Commands, Query, Res, ResMut},
    },
    input_focus::InputFocus,
    state::{state::NextState, state_scoped::DespawnOnExit},
    text::TextFont,
};
use bevy_enhanced_input::prelude::Complete;
use bevy_ui::{widget::Text, Val};

use crate::{
    states::Menu,
    ui::font::FontAssets,
    ui::{header, navigate, ui_root, Back},
};

pub fn spawn_settings_menu(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    // asset_server: Res<AssetServer>,
    // mut input_focus: ResMut<InputFocus>,
) {
    commands
        .spawn((ui_root("settings menu"), DespawnOnExit(Menu::Settings)))
        .observe(navigate)
        .observe(go_back)
        .with_children(|parent| {
            parent.spawn(header(
                "Settings",
                Val::Percent(15.0),
                65.0,
                fonts.default.clone(),
            ));
            // TODO: actual settings ui
            parent.spawn((
                Text::new("WIP"),
                TextFont::from_font_size(20.0).with_font(fonts.default.clone()),
            ));
        });
}

pub fn go_back(_click: On<Complete<Back>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}
