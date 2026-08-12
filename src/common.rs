use bevy::prelude::*;

pub const GAME_NAME: &str = "Demrock";

pub const CAR_ROTATION: f32 = 15.5;
pub const CAR_ACCELERATION: f32 = 350.0;
pub const CAR_BRAKE: f32 = 0.05;

pub const CAMERA_SCALE: f32 = 0.5;

pub const CAR_SPRITE_SIZE: (u32, u32) = (32, 32);
pub const CAR_ANIMATION_INDICES: [u32; 4] = [2, 8, 6, 4];
pub const CAR_NUM_ANIMATION: usize = 4;

pub const TILE_SIZE: i32 = 24;
pub const LEVEL_COUNT: i32 = 4;

#[derive(Resource, Debug, PartialEq, Eq, Clone, Copy)]
pub enum MultiplayerMode {
    Single = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}
