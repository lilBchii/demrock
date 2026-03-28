use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::car::{Car, Grounded, LapsTime, Progression, State};
use crate::gamemodes::SelectedLevel;
use crate::gameplay::TimeSinceStart;
use crate::states::PlayingState;
use crate::ui::in_game::{setup_gameplay_ui, TotalTimeDisplay};
use crate::ui::RedrawRequested;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TriggerZone>()
            .add_systems(
                OnEnter(PlayingState::Countdown),
                (spawn_level, setup_gameplay_ui).chain(),
            )
            .add_systems(
                Update,
                (handle_trigger_zone_collision, detect_car_out)
                    .run_if(in_state(PlayingState::Racing)),
            );
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
pub struct NumberOfLaps(pub usize);

pub fn handle_trigger_zone_collision(
    mut commands: Commands,
    mut message_reader: MessageReader<CollisionStart>,
    zone_query: Query<&TriggerZone>,
    collider_query: Query<&TiledColliderOf>,
    mut car_query: Query<(Entity, &mut Progression, &mut LapsTime), With<Car>>,
    display_query: Query<Entity, (With<Text>, Without<TotalTimeDisplay>)>,
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
        let Ok((car_entity, mut progression, mut laps_time)) = car_query.get_mut(actor_entity)
        else {
            return;
        };
        match zone {
            TriggerZone::Startingline => {
                if progression.last_checkpoint == 3 || progression.current_turn == 0 {
                    progression.current_turn += 1;
                    progression.last_checkpoint = 0;
                    laps_time.0.push(time_since_start.0.elapsed_secs());
                    for display_entity in display_query {
                        commands.entity(display_entity).insert(RedrawRequested);
                    }
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
    // commands
    //     .entity(evt.origin)
    //     .insert((Sensor, CollisionEventsEnabled));
    // // If it comes from tile layer then it is the road
    // if evt.event.source == TiledColliderSource::TilesLayer {
    //     commands.entity(evt.origin).insert(Road);
    // }
    match evt.event.source {
        TiledColliderSource::TilesLayer => commands.entity(evt.origin).insert(Road),
        TiledColliderSource::Object => commands
            .entity(evt.origin)
            .insert((Sensor, CollisionEventsEnabled)),
    };
}

fn detect_car_out(
    mut commands: Commands,
    car_query: Query<(Entity, &CollidingEntities, &mut State), (With<Car>, With<Grounded>)>,
    road_query: Query<Entity, (With<Road>, Without<TriggerZone>)>,
) {
    for (entity, colliding_entities, mut state) in car_query {
        for road_entity in road_query {
            if colliding_entities.contains(&road_entity) {
                *state = State::Falling;
                commands
                    .entity(entity)
                    .insert((ColliderDisabled, RigidBodyDisabled))
                    .remove::<Grounded>();
            }
        }
    }
}

#[derive(Clone)]
pub enum Level {
    Playground,
    Demcity,
    Galabusa,
}

impl Level {
    pub fn name(&self) -> &str {
        match self {
            Self::Playground => "Playground",
            Self::Demcity => "Demcity",
            Self::Galabusa => "Galabusa",
        }
    }

    pub fn spawn_level(&self, commands: Commands, asset_server: Res<AssetServer>) {
        match self {
            Self::Playground => spawn_playground_level(commands, asset_server),
            Self::Demcity => spawn_demcity_level(commands, asset_server),
            Self::Galabusa => spawn_galabusa_level(commands, asset_server),
        }
    }
}

pub static ALL_LEVELS: [Level; 3] = [Level::Playground, Level::Demcity, Level::Galabusa];

// Main level spawner system
fn spawn_level(
    commands: Commands,
    level_query: Query<&SelectedLevel>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(level) = level_query.single() {
        level.0.spawn_level(commands, asset_server);
    }
}

// Spawn Demcity level
fn spawn_demcity_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            TiledMap(asset_server.load("levels/demcity/map.tmx")),
            TilemapAnchor::Center,
            NumberOfLaps(3),
            DespawnOnExit(PlayingState::Racing),
        ))
        .observe(insert_road_colliders);
}

// Spawn Demcity playground level
pub fn spawn_playground_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            TiledMap(asset_server.load("levels/the_playground/map.tmx")),
            TilemapAnchor::Center,
            NumberOfLaps(2),
            DespawnOnExit(PlayingState::Racing),
        ))
        .observe(insert_road_colliders);
}

// Spawn Galabusa level
//
// Credits
// Design: Grégoire Genouville
// Tiles: Grégoire Genouville
pub fn spawn_galabusa_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            TiledMap(asset_server.load("levels/galabusa/map.tmx")),
            TilemapAnchor::Center,
            NumberOfLaps(4),
            DespawnOnExit(PlayingState::Racing),
        ))
        .observe(insert_road_colliders);
}
