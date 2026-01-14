use bevy::prelude::*;

pub const CAR_SPEED: f32 = 200.0;
pub const CAR_ROTATION: f32 = 4.0;
pub const CAR_ACCELERATION: f32 = 650.0;
pub const CAR_BRAKE: f32 = 0.05;

pub const CAMERA_SCALE: f32 = 0.5;

pub const CAR_SPRITE_SIZE: (u32, u32) = (32, 32);
pub const CAR_ANIMATION_INDICES: [u32; 4] = [2,5,1,3];
pub const CAR_NUM_ANIMATION: usize = 4;

pub const TILE_SIZE: i32 = 24;
pub const LEVEL_COUNT: i32 = 4;
pub const LEVEL_HEIGHT: f32 = 2400.0;
pub const LEVEL_WIDTH: f32 = 3600.0;
pub const LVL0_P1_START: (f32, f32) = (1670., 702.);
pub const LVL1_P1_START: (f32, f32) = (2004., 770.);
pub const LVL2_P1_START: (f32, f32) = (2004., 680.);
pub const LVL3_P1_START: (f32, f32) = (1935., 655.);

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    StartMenu,
    Playing,
    GameOver,
}

#[derive(Resource, Debug, PartialEq, Eq)]
pub enum MultiplayerMode {
    SinglePlayer,
    TwoPlayers,
}
