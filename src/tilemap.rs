use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component, Default)]
struct StartingLine;

fn create_starting_line(
    mut collider_created: On<TiledEvent<ColliderCreated>>,
    mut commands: Commands,
    zone_query: Query<Entity, With<StartingLine>>,
) {
    let evt = collider_created.event();
    if zone_query.get(evt.event.collider_of.0).is_ok() {
        commands
            .entity(evt.origin)
            .insert((Sensor, CollisionEventsEnabled));

    }
}

// Spawn Demcity level
pub fn spawn_demcity_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands
        .spawn((
            TiledMap(asset_server.load("levels/demcity/map.tmx")),
            TilemapAnchor::Center,
        ))
        .observe(
            |collider_created: On<TiledEvent<ColliderCreated>>, mut commands: Commands| {
                commands
                    .entity(collider_created.event().origin)
                    .insert(RigidBody::Static);
            },
        )
        .observe(create_starting_line);
}
