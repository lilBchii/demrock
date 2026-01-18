use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

#[derive(Component, Clone, PartialEq, Eq, Debug, Default)]
pub enum LevelItem {
    #[default]
    None,
    Road,
    Sky,
    StartingLine,
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
        );
}
