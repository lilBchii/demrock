use bevy::{input_focus::InputFocus, prelude::*};
use bevy_enhanced_input::prelude::Complete;

use crate::{
    common::AppState,
    font::FontAssets,
    ui::{header, nav_button, navigate, ui_root, ClickUI, NavButton, MENU_BUTTON_SIZE},
};

#[derive(Component)]
pub(crate) enum MenuAction {
    Restart,
    Exit,
}

pub fn spawn_main_menu(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    asset_server: Res<AssetServer>,
    mut input_focus: ResMut<InputFocus>,
) {
    let border_image = asset_server.load("button.png");

    commands
        .spawn((ui_root("game over"), DespawnOnExit(AppState::GameOver)))
        .observe(navigate)
        .observe(update_state)
        .with_children(|parent| {
            parent.spawn(header(
                "GAME OVER",
                Val::Percent(20.0),
                80.0,
                fonts.default.clone(),
            ));
            parent
                .spawn(Node {
                    width: percent(100),
                    height: percent(80),
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
                                "Restart",
                                px(MENU_BUTTON_SIZE.0),
                                px(MENU_BUTTON_SIZE.1),
                                fonts.default.clone(),
                                border_image.clone(),
                                MenuAction::Restart,
                            ))
                            .id(),
                    );
                    parent.spawn(nav_button(
                        "Exit",
                        px(MENU_BUTTON_SIZE.0),
                        px(MENU_BUTTON_SIZE.1),
                        fonts.default.clone(),
                        border_image.clone(),
                        MenuAction::Exit,
                    ));
                });
        });
}

pub fn update_state(
    _click: On<Complete<ClickUI>>,
    input_focus: Res<InputFocus>,
    action: Query<&MenuAction, With<NavButton>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Some(input_focus) = input_focus.0 {
        if let Ok(action) = action.get(input_focus) {
            match action {
                MenuAction::Restart => {
                    next_state.set(AppState::Playing);
                }
                MenuAction::Exit => {
                    next_state.set(AppState::StartMenu);
                }
            };
        }
    }
}
