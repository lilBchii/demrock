use bevy::input_focus::{AutoFocus, InputFocus};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::Complete;

use crate::{
    gamemodes::{ArcadeLevels, GameMode, SelectedLevel},
    states::{GameState, Menu},
    ui::{
        font::FontAssets, header, nav_button, navigate, ui_root, Back, ClickUI, NavButton, H2_SIZE,
    },
};

#[derive(Component)]
pub(crate) enum MenuAction {
    Arcade,
    Custom,
}

pub fn spawn_mode_selection_menu(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    asset_server: Res<AssetServer>,
) {
    let (button_width, button_height) = (Val::Px(400.0), Val::Px(80.0));
    let border_image = asset_server.load("button.png");

    commands
        .spawn((ui_root("main menu"), DespawnOnExit(Menu::ModeSelection)))
        .observe(navigate)
        .observe(update_state)
        .observe(go_back)
        .with_children(|parent| {
            parent.spawn(header(
                "Game mode",
                Val::Percent(15.0),
                H2_SIZE,
                fonts.default.clone(),
            ));
            parent
                .spawn(Node {
                    width: percent(100),
                    height: percent(85),
                    position_type: PositionType::Absolute,
                    top: Val::Percent(30.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Percent(5.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        nav_button(
                            "Arcade",
                            button_width,
                            button_height,
                            fonts.default.clone(),
                            border_image.clone(),
                            MenuAction::Arcade,
                        ),
                        AutoFocus,
                    ));
                    parent.spawn(nav_button(
                        "Custom",
                        button_width,
                        button_height,
                        fonts.default.clone(),
                        border_image.clone(),
                        MenuAction::Custom,
                    ));
                });
        });
}

pub fn update_state(
    _click: On<Complete<ClickUI>>,
    input_focus: Res<InputFocus>,
    action: Query<&MenuAction, With<NavButton>>,
    mut game_mode_query: Single<(&mut SelectedLevel, &mut ArcadeLevels, &mut GameMode)>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(input_focus) = input_focus.get() {
        if let Ok(action) = action.get(input_focus) {
            match action {
                MenuAction::Arcade => {
                    let arcade_levels = ArcadeLevels::init();
                    game_mode_query.0 .0 = arcade_levels.levels[0].clone();
                    *game_mode_query.1 = arcade_levels;
                    *game_mode_query.2 = GameMode::Arcade;

                    next_menu.set(Menu::None);
                    next_state.set(GameState::Playing);
                }
                MenuAction::Custom => {
                    *game_mode_query.2 = GameMode::Free;
                    next_menu.set(Menu::LevelSelection);
                }
            };
        }
    }
}

pub fn go_back(_click: On<Complete<Back>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}
