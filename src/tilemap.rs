use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::car::{Car, IsGrounded, LapsTime, Progression, State};
use crate::common::AppState;
use crate::font::FontAssets;
use crate::gameplay::{CrossTheLine, TimeSinceStart};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TriggerZone>()
            .add_systems(
                OnEnter(AppState::Playing),
                (spawn_playground_level, setup_gameplay_ui),
            )
            .add_systems(
                Update,
                (
                    handle_trigger_zone_collision,
                    update_lap_display,
                    update_total_time_display,
                )
                    .run_if(in_state(AppState::Playing)),
            )
            .add_observer(detect_car_out)
            .add_observer(update_time_by_lap_display);
    }
}

#[derive(Component)]
pub struct Road;

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component, Default)]
pub enum TriggerZone {
    #[default]
    Default,
    Startingline,
    CheckPoint(u8),
}

#[derive(Component)]
pub struct NumberOfLaps(usize);

#[derive(Component)]
struct CurrentTurnDisplay;

#[derive(Component)]
struct TotalTimeDisplay;

#[derive(Component)]
struct LapTimeDisplay;

pub fn handle_trigger_zone_collision(
    mut commands: Commands,
    mut message_reader: MessageReader<CollisionStart>,
    zone_query: Query<&TriggerZone>,
    collider_query: Query<&TiledColliderOf>,
    mut car_query: Query<(Entity, &mut Transform, &mut Progression, &mut LapsTime), With<Car>>,
    time_since_start: Res<TimeSinceStart>,
) {
    for evt in message_reader.read() {
        let Ok(zone) = collider_query
            .get(evt.collider1)
            .and_then(|&collider_of| zone_query.get(*collider_of))
        else {
            return;
        };
        let Some(actor_entity) = evt.body2 else {
            return;
        };
        let Ok((entity, mut transform, mut progression, mut laps_time)) =
            car_query.get_mut(actor_entity)
        else {
            return;
        };
        match zone {
            TriggerZone::Startingline => {
                if progression.last_checkpoint == 3 || progression.current_turn == 0 {
                    progression.current_turn += 1;
                    progression.last_checkpoint = 0;
                    laps_time.0.push(time_since_start.0.elapsed_secs());
                    commands.trigger(CrossTheLine { entity });
                }
            }
            TriggerZone::CheckPoint(n) => {
                if *n == progression.last_checkpoint + 1 {
                    progression.last_checkpoint += 1;
                }
            }
            _ => {}
        }
    }
}

fn insert_road_colliders(
    collider_created: On<TiledEvent<ColliderCreated>>,
    mut commands: Commands,
) {
    let evt = collider_created.event();
    commands
        .entity(evt.origin)
        .insert((Sensor, CollisionEventsEnabled));
    // If it comes from tile layer then it is the road
    if evt.event.source == TiledColliderSource::TilesLayer {
        commands.entity(evt.origin).insert(Road);
    }
}

pub fn detect_car_out(
    collision_event: On<CollisionStart>,
    mut car_query: Query<(&mut IsGrounded, &mut State), With<Car>>,
    road_query: Query<&Road, Without<TriggerZone>>,
) {
    let road_entity = collision_event.collider1;
    let car_entity = collision_event.collider2;

    if road_query.contains(road_entity) {
        if let Ok((mut is_grounded, mut state)) = car_query.get_mut(car_entity) {
            is_grounded.0 = false;
            *state = State::Falling;
        }
    }
}

fn setup_gameplay_ui(mut commands: Commands, fonts: Res<FontAssets>) {
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
                Text::new("lap: 0/3"),
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
        DespawnOnExit(AppState::Playing),
    ));
}

fn update_lap_display(
    progression_query: Query<&Progression, With<Car>>,
    n_lap_query: Query<&NumberOfLaps>,
    mut lap_query: Query<&mut Text, With<CurrentTurnDisplay>>,
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
    _: On<CrossTheLine>,
    progression_query: Query<&LapsTime, With<Car>>,
    mut lap_time_query: Query<&mut Text, With<LapTimeDisplay>>,
) {
    if let Ok(laps_time) = progression_query.single() {
        if let Ok(mut text) = lap_time_query.single_mut() {
            let mut string_buffer = String::new();
            for (lap, time) in laps_time
                .0
                .iter()
                .scan(0.0, |state, time| {
                    let lap_time = time - *state;
                    *state = *time;
                    Some(lap_time)
                })
                .skip(1)
                .enumerate()
            {
                string_buffer.push_str(&format!("lap {}: {:.3}\n", lap + 1, time));
            }
            *text = string_buffer.into();
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

// Spawn Demcity level
pub fn spawn_demcity_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            TiledMap(asset_server.load("levels/demcity/map.tmx")),
            TilemapAnchor::Center,
            NumberOfLaps(3),
            DespawnOnExit(AppState::Playing),
        ))
        .observe(insert_road_colliders);
}

// Spawn Demcity playground level
pub fn spawn_playground_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            TiledMap(asset_server.load("levels/the_playground/map.tmx")),
            TilemapAnchor::Center,
            NumberOfLaps(3),
            DespawnOnExit(AppState::Playing),
        ))
        .observe(insert_road_colliders);
}

// Spawn Galabusa level
//
// Credits:
// Design: Grégoire Genouville
// Tiles: Grégoire Genouville
pub fn spawn_galabusa_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            TiledMap(asset_server.load("levels/galabusa/map.tmx")),
            TilemapAnchor::Center,
            NumberOfLaps(4),
            DespawnOnExit(AppState::Playing),
        ))
        .observe(insert_road_colliders);
}
