use std::f32::consts::FRAC_PI_2;

use macroquad::audio::{load_sound, Sound};
use macroquad::color::WHITE;
use macroquad::math::{vec2, Rect};
use macroquad::miniquad::FilterMode;
use macroquad::texture::{draw_texture_ex, load_texture, DrawTextureParams, Texture2D};
use serde::Deserialize;

pub const TILE_SIZE: f32 = 24.0;
pub const MAP_SIZE: (f32, f32) = (500.0, 250.0);

#[derive(Clone, Deserialize)]
pub struct Levels {
    pub levels: Vec<LevelConfig>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename(deserialize = "level"))]
pub struct LevelConfig {
    name: String,
    background_path: String,
    tiles_texture_path: String,
    music_path: String,
    starting_position: [usize; 2],
    parallax: f32,
    tiles: Vec<Tile>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Tile {
    pub position: [usize; 2],
    pub mapatlas_source: [usize; 2],
    pub rotation: usize,
}

#[derive(Debug, Clone, Copy, Deserialize)]
enum TileType {
    StartingLine,
    Base,
    BaseDecorated1,
    BaseDecorated2,
    BaseDecorated3,
    BaseDecorated4,
    BaseDecorated5,
    HardTurnInterior,
    HardTurnExterior,
    SoftTurnInterior,
    SoftTurnInterior2,
    SoftTurnExterior,
    SoftTurnExterior2,
    StraightBorder,
    DiagBorder,
}

#[derive(Clone, Copy)]
pub enum Rotation {
    PiSur2 = 1,
    Pi = 2,
    PiFois3Sur2 = 3,
    PiFois2 = 0,
}

pub struct Level {
    name: String,
    pub background: Texture2D,
    pub tile_texture: Texture2D,
    music: Sound,
    pub starting_position: [usize; 2],
    pub parallax: f32,
    tiles: Vec<Tile>,
}

impl Level {
    pub async fn load(conf: &LevelConfig) -> Self {
        let background: Texture2D = load_texture(&conf.background_path).await.expect("file bg");
        let music: Sound = load_sound(&conf.music_path).await.expect("file sound");
        let tile_texture: Texture2D = load_texture(&conf.tiles_texture_path)
            .await
            .expect("file tile");

        background.set_filter(FilterMode::Nearest);
        tile_texture.set_filter(FilterMode::Nearest);

        Self {
            name: conf.name.clone(),
            background,
            tile_texture,
            music,
            starting_position: conf.starting_position,
            parallax: conf.parallax,
            tiles: conf.tiles.clone(),
        }
    }

    pub fn draw_background(&self) {
        draw_texture_ex(
            &self.background,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                ..Default::default()
            },
        );
    }

    pub fn draw_circuit(&self) {
        self.tiles.iter().for_each(|tile| {
            draw_texture_ex(
                &self.tile_texture,
                TILE_SIZE * tile.position[0] as f32,
                TILE_SIZE * tile.position[1] as f32,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                    source: Some(Rect::new(
                        tile.mapatlas_source[0] as f32 * TILE_SIZE,
                        tile.mapatlas_source[1] as f32 * TILE_SIZE,
                        TILE_SIZE,
                        TILE_SIZE,
                    )),
                    rotation: tile.rotation as f32 * FRAC_PI_2,
                    ..Default::default()
                },
            )
        });
    }
}
