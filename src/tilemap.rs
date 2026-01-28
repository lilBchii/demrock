use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::car::{Car, IsGrounded, State};
use crate::common::AppState;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TriggerZone>()
            .add_systems(OnEnter(AppState::Playing), spawn_demcity_level)
            .add_systems(
                Update,
                handle_trigger_zone_collision.run_if(in_state(AppState::Playing)),
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

pub fn handle_trigger_zone_collision(
    mut message_reader: MessageReader<CollisionStart>,
    mut commands: Commands,
    zone_query: Query<&TriggerZone>,
    collider_query: Query<&TiledColliderOf>,
    mut car_query: Query<(Entity, &mut Transform), With<Car>>,
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
        let Ok((actor, mut transform)) = car_query.get_mut(actor_entity) else {
            return;
        };
        match zone {
            TriggerZone::Startingline => {
                println!("starting line");
            }
            TriggerZone::CheckPoint(n) => {
                println!("checkpoint {n}");
            }
            _ => {
                println!("else");
            }
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
