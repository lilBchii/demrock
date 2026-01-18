use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::car::spawn_car;

#[derive(Component, Clone, PartialEq, Eq, Debug, Default)]
pub enum LevelItem {
    #[default]
    None,
    Road,
    Sky,
    StartingLine,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[require(Transform)]
#[reflect(Component)]
struct SpawnPoint;

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
        );
    // TODO: wait for map spawn to spawn player
    spawn_car(&mut commands, atlas_layouts, &asset_server, 0.0, 0.0);
}
