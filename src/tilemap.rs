use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

#[derive(Component)]
pub struct Road;

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component, Default)]
pub struct StartingLine;

fn create_starting_line(
    object_created: On<TiledEvent<ObjectCreated>>,
    mut commands: Commands,
    zone_query: Query<Entity, With<StartingLine>>,
) {
    let evt = object_created.event();
    if zone_query.get(evt.origin).is_ok() {
        commands
            .entity(evt.origin)
            .insert((Sensor, CollisionEventsEnabled));
    }
}

fn insert_road_colliders(
    collider_created: On<TiledEvent<ColliderCreated>>,
    mut commands: Commands,
) {
    commands
        .entity(collider_created.event().origin)
        .insert((RigidBody::Static, Road));
}

// Spawn Demcity level
pub fn spawn_demcity_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            TiledMap(asset_server.load("levels/demcity/map.tmx")),
            TilemapAnchor::Center,
        ))
        .observe(insert_road_colliders)
        .observe(create_starting_line);
}
