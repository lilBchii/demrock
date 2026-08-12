use bevy::input_focus::{AutoFocus, InputFocus};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::Complete;

use crate::camera::{FocusOnPlayer, InGameCamera};
use crate::gameplay::PlayerFinished;
use crate::{
    car::{Car, GameProgression},
    states::{GameState, Menu, PlayingState},
    ui::{
        font::FontAssets, header, nav_button, navigate, ui_root, ClickUI, NavButton, H2_SIZE,
        MENU_BUTTON_SIZE,
    },
};

#[derive(Component)]
pub(crate) enum MenuAction {
    Continue,
    Exit,
}

pub fn spawn_race_over_menu(
    mut commands: Commands,
    score_query: Query<&GameProgression, With<Car>>,
    fonts: Res<FontAssets>,
    asset_server: Res<AssetServer>,
) {
    let border_image = asset_server.load("button.png");

    commands
        .spawn((ui_root("game over"), DespawnOnExit(Menu::RaceOver)))
        .observe(navigate)
        .observe(update_state)
        .with_children(|screen| {
            // Spawn the header
            screen.spawn(header(
                "RACE FINISHED",
                Val::Percent(15.0),
                H2_SIZE,
                fonts.default.clone(),
            ));

            // Spawn the body
            screen
                .spawn(Node {
                    width: percent(100),
                    height: percent(85),
                    position_type: PositionType::Absolute,
                    top: Val::Percent(15.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Percent(5.0),
                    ..default()
                })
                .with_children(|body| {
                    // Spawn the scores
                    body.spawn(Node {
                        width: percent(100),
                        height: percent(50),
                        position_type: PositionType::Relative,
                        top: Val::ZERO,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Row,
                        row_gap: Val::Percent(5.0),
                        ..default()
                    })
                    .with_children(|scores| {
                        for score in score_query {
                            scores
                                .spawn(Node {
                                    width: percent(100),
                                    height: percent(100),
                                    position_type: PositionType::Relative,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Percent(5.0),
                                    ..default()
                                })
                                .with_children(|by_player_score| {
                                    by_player_score.spawn((
                                        Text::new(score.current_race_times().to_string()),
                                        TextFont::from_font_size(15.0)
                                            .with_font(fonts.default.clone()),
                                    ));
                                });
                        }
                    });
                    // Spawn the nav buttons in column
                    body.spawn(Node {
                        width: percent(100),
                        height: percent(50),
                        position_type: PositionType::Relative,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Percent(5.0),
                        ..default()
                    })
                    .with_children(|buttons| {
                        buttons.spawn((
                            nav_button(
                                "Continue",
                                px(MENU_BUTTON_SIZE.0),
                                px(MENU_BUTTON_SIZE.1),
                                fonts.default.clone(),
                                border_image.clone(),
                                MenuAction::Continue,
                            ),
                            AutoFocus,
                        ));
                        buttons.spawn(nav_button(
                            "Exit",
                            px(MENU_BUTTON_SIZE.0),
                            px(MENU_BUTTON_SIZE.1),
                            fonts.default.clone(),
                            border_image.clone(),
                            MenuAction::Exit,
                        ));
                    });
                });
        });
}

pub fn spawn_individual_race_over_menu(
    player_finished: On<PlayerFinished>,
    mut commands: Commands,
    score_query: Query<&GameProgression, With<Car>>,
    camera_query: Query<(Entity, &FocusOnPlayer), With<InGameCamera>>,
    fonts: Res<FontAssets>,
) {
    for (index, (camera, focused_player)) in camera_query.iter().enumerate() {
        if focused_player.0 == player_finished.0 {
            let Ok(game_progression) = score_query.get(player_finished.0) else {
                return;
            };
            println!("spawn race over");
            let laps_time = game_progression.current_race_times();
            commands
                .spawn((
                    ui_root("game over"),
                    UiTargetCamera(camera),
                    DespawnOnEnter(PlayingState::End),
                ))
                .with_children(|screen| {
                    // Spawn the header
                    screen.spawn(header(
                        "RACE FINISHED",
                        Val::Percent(15.0),
                        H2_SIZE,
                        fonts.default.clone(),
                    ));

                    // Spawn the body
                    screen
                        .spawn(Node {
                            top: Val::Px((index / 2) as f32),
                            left: Val::Px((index % 2) as f32),
                            width: percent(100),
                            height: percent(85),
                            position_type: PositionType::Absolute,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Percent(5.0),
                            ..default()
                        })
                        .with_children(|body| {
                            // Spawn the scores
                            body.spawn((
                                Node {
                                    width: percent(100),
                                    height: percent(100),
                                    position_type: PositionType::Relative,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Percent(5.0),
                                    ..default()
                                },
                                children![(
                                    Text::new(laps_time.to_string()),
                                    TextFont::from_font_size(15.0).with_font(fonts.default.clone()),
                                )],
                            ));
                        });
                });
        }
    }
}

pub fn update_state(
    _click: On<Complete<ClickUI>>,
    input_focus: Res<InputFocus>,
    action: Query<&MenuAction, With<NavButton>>,
    mut game_progression: Query<&mut GameProgression, With<Car>>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut next_state: ResMut<NextState<PlayingState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Some(input_focus) = input_focus.get() {
        if let Ok(action) = action.get(input_focus) {
            match action {
                MenuAction::Continue => {
                    for mut progression in &mut game_progression {
                        progression.next_race();
                    }
                    next_menu.set(Menu::None);
                    next_state.set(PlayingState::Countdown);
                }
                MenuAction::Exit => {
                    next_menu.set(Menu::Main);
                    next_game_state.set(GameState::Init);
                }
            };
        }
    }
}
