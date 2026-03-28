use bevy::ecs::component::Component;

use crate::tilemap::Level;

#[derive(Component)]
enum GameMode {
    Arcade,
    Free,
}

#[derive(Component)]
pub struct SelectedLevel(pub Level);
