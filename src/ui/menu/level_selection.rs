use bevy::{
    asset::AssetServer,
    ecs::{
        component::Component,
        observer::On,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    input_focus::InputFocus,
    prelude::default,
    state::{state::NextState, state_scoped::DespawnOnExit},
};
use bevy_enhanced_input::prelude::Complete;
use bevy_ui::{percent, AlignItems, FlexDirection, JustifyContent, Node, PositionType, Val};

use crate::{
    gamemodes::SelectedLevel,
    states::{GameState, Menu},
    tilemap::{Level, ALL_LEVELS},
    ui::{font::FontAssets, header, nav_button, navigate, ui_root, Back, ClickUI, NavButton},
};

#[derive(Component)]
pub(crate) enum MenuAction {
    SelectLevel(Level),
    PrevPage,
    NextPage,
}

pub fn spawn_level_selection_menu(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    asset_server: Res<AssetServer>,
    mut input_focus: ResMut<InputFocus>,
) {
    let (button_width, button_height) = (Val::Px(400.0), Val::Px(80.0));
    let border_image = asset_server.load("button.png");

    commands
        .spawn((ui_root("main menu"), DespawnOnExit(Menu::LevelSelection)))
        .observe(navigate)
        .observe(update_state)
        .observe(go_back)
        .with_children(|parent| {
            parent.spawn(header(
                "Select Level",
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
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Percent(5.0),
                    ..default()
                })
                .with_children(|parent| {
                    input_focus.set(
                        parent
                            .spawn(nav_button(
                                Level::Playground.name(),
                                button_width,
                                button_height,
                                fonts.default.clone(),
                                border_image.clone(),
                                MenuAction::SelectLevel(Level::Playground),
                            ))
                            .id(),
                    );
                    for level in ALL_LEVELS.iter().skip(1) {
                        parent.spawn(nav_button(
                            level.name(),
                            button_width,
                            button_height,
                            fonts.default.clone(),
                            border_image.clone(),
                            MenuAction::SelectLevel(level.clone()),
                        ));
                    }
                });
        });
}

pub fn update_state(
    _click: On<Complete<ClickUI>>,
    mut commands: Commands,
    input_focus: Res<InputFocus>,
    action: Query<&MenuAction, With<NavButton>>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(input_focus) = input_focus.0 {
        if let Ok(action) = action.get(input_focus) {
            match action {
                MenuAction::SelectLevel(level) => {
                    commands.spawn((
                        SelectedLevel(level.clone()),
                        DespawnOnExit(GameState::Playing),
                    ));
                    next_menu.set(Menu::None);
                    next_state.set(GameState::Playing);
                }
                MenuAction::PrevPage => {}
                MenuAction::NextPage => {}
            };
        }
    }
}

pub fn go_back(_click: On<Complete<Back>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}
