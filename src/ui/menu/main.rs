use bevy::input_focus::InputFocus;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::Complete;

use crate::{
    common::GAME_NAME,
    states::{GameState, Menu},
    ui::font::FontAssets,
    ui::{header, nav_button, navigate, ui_root, ClickUI, NavButton},
};

#[derive(Component)]
pub(crate) enum MenuAction {
    Play,
    Settings,
    Credits,
    Quit,
}

pub fn spawn_main_menu(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    asset_server: Res<AssetServer>,
    mut input_focus: ResMut<InputFocus>,
) {
    let (button_width, button_height) = (Val::Px(400.0), Val::Px(80.0));
    let border_image = asset_server.load("button.png");

    commands
        .spawn((ui_root("main menu"), DespawnOnExit(Menu::Main)))
        .observe(navigate)
        .observe(update_state)
        .with_children(|parent| {
            parent.spawn(header(
                GAME_NAME,
                Val::Percent(30.0),
                80.0,
                fonts.default.clone(),
            ));
            parent
                .spawn(Node {
                    width: percent(100),
                    height: percent(70),
                    position_type: PositionType::Absolute,
                    top: Val::Percent(30.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Percent(5.0),
                    ..default()
                })
                .with_children(|parent| {
                    input_focus.set(
                        parent
                            .spawn(nav_button(
                                "Play",
                                button_width,
                                button_height,
                                fonts.default.clone(),
                                border_image.clone(),
                                MenuAction::Play,
                            ))
                            .id(),
                    );
                    parent.spawn(nav_button(
                        "Settings",
                        button_width,
                        button_height,
                        fonts.default.clone(),
                        border_image.clone(),
                        MenuAction::Settings,
                    ));
                    parent.spawn(nav_button(
                        "Credits",
                        button_width,
                        button_height,
                        fonts.default.clone(),
                        border_image.clone(),
                        MenuAction::Credits,
                    ));
                    parent.spawn(nav_button(
                        "Quit",
                        button_width,
                        button_height,
                        fonts.default.clone(),
                        border_image.clone(),
                        MenuAction::Quit,
                    ));
                });
        });
}

pub fn update_state(
    _click: On<Complete<ClickUI>>,
    input_focus: Res<InputFocus>,
    action: Query<&MenuAction, With<NavButton>>,
    mut next_menu: ResMut<NextState<Menu>>,
    // mut next_state: ResMut<NextState<GameState>>,
    mut app_exit: MessageWriter<AppExit>,
) {
    if let Some(input_focus) = input_focus.0 {
        if let Ok(action) = action.get(input_focus) {
            match action {
                MenuAction::Play => {
                    // TODO: set menu to PlayerMenu
                    next_menu.set(Menu::LevelSelection);
                }
                MenuAction::Settings => {
                    next_menu.set(Menu::Settings);
                }
                MenuAction::Credits => {
                    next_menu.set(Menu::Credits);
                }
                MenuAction::Quit => {
                    app_exit.write(AppExit::Success);
                }
            };
        }
    }
}
