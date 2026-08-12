use bevy::app::Plugin;
use bevy::app::Update;
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::ecs::{
    children,
    component::Component,
    entity::Entity,
    name::Name,
    query::With,
    system::{Commands, Query, Res},
};
use bevy::prelude::{
    default, AlignItems, FlexDirection, JustifyContent, Node, PositionType, Text, UiRect, Val,
};
use bevy::state::condition::in_state;
use bevy::state::state_scoped::DespawnOnExit;

use bevy::text::TextFont;
use bevy_ui::px;
use bevy_ui::UiTargetCamera;

use crate::camera::FocusOnPlayer;
use crate::camera::InGameCamera;
use crate::car::GameProgression;
use crate::{
    car::{Car, RaceProgression},
    gameplay::TimeSinceStart,
    states::PlayingState,
    tilemap::NumberOfLaps,
    ui::{font::FontAssets, RedrawRequested},
};

pub struct InGameUiPlugin;

impl Plugin for InGameUiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            Update,
            (
                update_lap_display,
                update_time_by_lap_display,
                update_total_time_display,
            )
                .run_if(in_state(PlayingState::Racing)),
        );
    }
}

#[derive(Component)]
struct CurrentTurnDisplay;

#[derive(Component)]
pub(crate) struct TotalTimeDisplay;

#[derive(Component)]
struct LapTimeDisplay;

pub fn setup_gameplay_ui(
    mut commands: Commands,
    n_lap_query: Query<&NumberOfLaps>,
    fonts: Res<FontAssets>,
    camera_query: Query<(Entity, &FocusOnPlayer), With<InGameCamera>>,
) {
    let n_lap = n_lap_query.single().map_or(0, |lap| lap.0);
    for (index, (camera, focused_player)) in camera_query.iter().enumerate() {
        commands.spawn((
            Name::new(format!("gameplay ui {}", camera)),
            UiTargetCamera(camera),
            Node {
                top: Val::Px((index / 2) as f32),
                left: Val::Px((index % 2) as f32),
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                padding: UiRect::all(px(10)),
                ..default()
            },
            children![
                (
                    CurrentTurnDisplay,
                    Text::new(format!("lap: 0/{}", n_lap)),
                    TextFont::from_font_size(20.0).with_font(fonts.default.clone()),
                    FocusOnPlayer(focused_player.0),
                ),
                (
                    TotalTimeDisplay,
                    Text::new(""),
                    TextFont::from_font_size(20.0).with_font(fonts.default.clone()),
                    FocusOnPlayer(focused_player.0),
                ),
                (
                    LapTimeDisplay,
                    Text::new(""),
                    TextFont::from_font_size(15.0).with_font(fonts.default.clone()),
                    FocusOnPlayer(focused_player.0),
                )
            ],
            FocusOnPlayer(focused_player.0),
            DespawnOnExit(PlayingState::Racing),
        ));
    }
}

fn update_lap_display(
    progression_query: Query<&RaceProgression, With<Car>>,
    n_lap_query: Query<&NumberOfLaps>,
    lap_query: Query<
        (&mut Text, &FocusOnPlayer),
        (With<CurrentTurnDisplay>, With<RedrawRequested>),
    >,
) {
    for (mut text, focused_player) in lap_query {
        let Ok(progression) = progression_query.get(focused_player.0) else {
            return;
        };
        if let Ok(number_of_laps) = n_lap_query.single() {
            text.0 = format!("lap: {}/{}", progression.current_lap, number_of_laps.0).into();
        }
    }
}

fn update_time_by_lap_display(
    progression_query: Query<&GameProgression, With<Car>>,
    lap_time_query: Query<
        (&mut Text, &FocusOnPlayer),
        (With<LapTimeDisplay>, With<RedrawRequested>),
    >,
) {
    for (mut text, focused_player) in lap_time_query {
        let Ok(progression) = progression_query.get(focused_player.0) else {
            return;
        };
        text.0 = progression.current_race_times().to_string().into();
    }
}

fn update_total_time_display(
    total_time_res: Res<TimeSinceStart>,
    total_time_text_query: Query<&mut Text, With<TotalTimeDisplay>>,
) {
    for mut text in total_time_text_query {
        text.0 = format!(
            "Total time: {:.3}",
            total_time_res.0.elapsed().as_secs_f32()
        )
        .into();
    }
}
