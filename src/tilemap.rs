use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::car::{Car, IsGrounded, Progression, State};
use crate::common::AppState;
use crate::font::FontAssets;

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
                (handle_trigger_zone_collision, update_turn_display)
                    .run_if(in_state(AppState::Playing)),
            )
            .add_observer(detect_car_out);
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
struct CurrentTurnDisplay;

#[derive(Component)]
struct TimeDisplay;

pub fn handle_trigger_zone_collision(
    mut message_reader: MessageReader<CollisionStart>,
    zone_query: Query<&TriggerZone>,
    collider_query: Query<&TiledColliderOf>,
    mut car_query: Query<(&mut Transform, &mut Progression), With<Car>>,
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
        let Ok((mut transform, mut progression)) = car_query.get_mut(actor_entity) else {
            return;
        };
        match zone {
            TriggerZone::Startingline => {
                if progression.last_checkpoint == 3 || progression.current_turn == 0 {
                    progression.current_turn += 1;
                    progression.last_checkpoint = 0;
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
        children![(
            CurrentTurnDisplay,
            Text::new("turn: 0/3"),
            TextFont::from_font_size(20.0).with_font(fonts.default.clone()),
        )],
        DespawnOnExit(AppState::Playing),
    ));
}

fn update_turn_display(
    progression_query: Query<&Progression, With<Car>>,
    mut text_query: Query<&mut Text, With<CurrentTurnDisplay>>,
) {
    if let Ok(mut text) = text_query.single_mut() {
        if let Ok(progression) = progression_query.single() {
            *text = format!("turn: {}/3", progression.current_turn).into();
        }
    }
}

// Spawn Demcity level
pub fn spawn_demcity_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            TiledMap(asset_server.load("levels/demcity/map.tmx")),
            TilemapAnchor::Center,
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
            DespawnOnExit(AppState::Playing),
        ))
        .observe(insert_road_colliders);
}
