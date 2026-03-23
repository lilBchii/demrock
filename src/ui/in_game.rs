use bevy::app::Update;
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::prelude::{
    default, AlignItems, FlexDirection, JustifyContent, Node, PositionType, Text, UiRect, Val,
};
use bevy::state::condition::in_state;
use bevy::state::state_scoped::DespawnOnExit;
use bevy::text::TextFont;
use bevy::{
    app::Plugin,
    ecs::{
        children,
        component::Component,
        name::Name,
        query::With,
        system::{Commands, Query, Res},
    },
};
use bevy_ui::px;

use crate::{
    car::{Car, LapsTime, Progression},
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
) {
    let n_lap = n_lap_query.single().map_or(0, |lap| lap.0);
    commands.spawn((
        Name::new("gameplay ui"),
        Node {
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
            ),
            (
                TotalTimeDisplay,
                Text::new(""),
                TextFont::from_font_size(20.0).with_font(fonts.default.clone()),
            ),
            (
                LapTimeDisplay,
                Text::new(""),
                TextFont::from_font_size(15.0).with_font(fonts.default.clone()),
            )
        ],
        DespawnOnExit(PlayingState::Racing),
    ));
}

fn update_lap_display(
    progression_query: Query<&Progression, With<Car>>,
    n_lap_query: Query<&NumberOfLaps>,
    mut lap_query: Query<&mut Text, (With<CurrentTurnDisplay>, With<RedrawRequested>)>,
) {
    if let Ok(progression) = progression_query.single() {
        if let Ok(mut text) = lap_query.single_mut() {
            if let Ok(number_of_laps) = n_lap_query.single() {
                *text = format!("lap: {}/{}", progression.current_turn, number_of_laps.0).into();
            }
        }
    }
}

fn update_time_by_lap_display(
    progression_query: Query<&LapsTime, With<Car>>,
    mut lap_time_query: Query<&mut Text, (With<LapTimeDisplay>, With<RedrawRequested>)>,
) {
    if let Ok(laps_time) = progression_query.single() {
        if let Ok(mut text) = lap_time_query.single_mut() {
            *text = laps_time.to_string().into();
        }
    }
}

fn update_total_time_display(
    total_time_res: Res<TimeSinceStart>,
    mut total_time_text_query: Query<&mut Text, With<TotalTimeDisplay>>,
) {
    if let Ok(mut text) = total_time_text_query.single_mut() {
        *text = format!(
            "Total time: {:.3}",
            total_time_res.0.elapsed().as_secs_f32()
        )
        .into();
    }
}
