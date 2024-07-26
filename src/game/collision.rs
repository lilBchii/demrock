use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

use macroquad::{
    color::RED,
    math::{vec2, Rect, Vec2},
    shapes::{draw_circle, draw_line},
};

use super::{Player, Rotation, Tile, TileType, TILE_DIAG_SIZE, TILE_SIZE};

pub trait Collider<T> {
    fn collides(&self, other: T) -> bool;
}

#[derive(Debug)]
pub struct LineBorder {
    pub start: Vec2,
    pub lengh: f32,
    // rotation around start
    pub rotation: f32,
}

impl LineBorder {
    pub fn new(start: Vec2, lengh: f32, rotation: f32) -> Self {
        Self {
            start,
            lengh,
            rotation,
        }
    }

    pub fn rotated_start(&self) -> Vec2 {
        vec2(
            self.start.x * self.rotation.cos() + self.start.y * self.rotation.sin(),
            -self.start.x * self.rotation.sin() + self.start.y * self.rotation.cos(),
        )
    }

    pub fn end(&self) -> Vec2 {
        vec2(
            -(self.start.y - self.lengh) * self.rotation.sin() + self.start.x,
            (self.start.y - self.lengh) * self.rotation.cos() + self.start.y,
        )
    }

    pub fn can_from(tile: &Tile) -> Option<Self> {
        match tile.tile_type {
            TileType::DiagBorder => match tile.rotation {
                Rotation::PiFois2 => Some(LineBorder::new(
                    vec2(
                        (tile.position[0] + 1) as f32 * TILE_SIZE,
                        tile.position[1] as f32 * TILE_SIZE,
                    ),
                    TILE_DIAG_SIZE,
                    -3.0 * FRAC_PI_4,
                )),
                Rotation::PiSur2 => Some(LineBorder::new(
                    vec2(
                        (tile.position[0] + 1) as f32 * TILE_SIZE,
                        (tile.position[1] + 1) as f32 * TILE_SIZE,
                    ),
                    TILE_DIAG_SIZE,
                    -FRAC_PI_4,
                )),
                Rotation::Pi => Some(LineBorder::new(
                    vec2(
                        tile.position[0] as f32 * TILE_SIZE,
                        (tile.position[1] + 1) as f32 * TILE_SIZE,
                    ),
                    TILE_DIAG_SIZE,
                    FRAC_PI_4,
                )),
                Rotation::PiFois3Sur2 => Some(LineBorder::new(
                    vec2(
                        tile.position[0] as f32 * TILE_SIZE,
                        tile.position[1] as f32 * TILE_SIZE,
                    ),
                    TILE_DIAG_SIZE,
                    3.0 * FRAC_PI_4,
                )),
            },
            TileType::SoftTurnExterior => match tile.rotation {
                Rotation::PiSur2 => Some(LineBorder::new(
                    vec2(
                        (tile.position[0] + 1) as f32 * TILE_SIZE,
                        tile.position[1] as f32 * TILE_SIZE,
                    ),
                    TILE_DIAG_SIZE,
                    -3.0 * FRAC_PI_4,
                )),
                Rotation::Pi => Some(LineBorder::new(
                    vec2(
                        (tile.position[0] + 1) as f32 * TILE_SIZE,
                        (tile.position[1] + 1) as f32 * TILE_SIZE,
                    ),
                    TILE_DIAG_SIZE,
                    -FRAC_PI_4,
                )),
                Rotation::PiFois3Sur2 => Some(LineBorder::new(
                    vec2(
                        tile.position[0] as f32 * TILE_SIZE,
                        (tile.position[1] + 1) as f32 * TILE_SIZE,
                    ),
                    TILE_DIAG_SIZE,
                    FRAC_PI_4,
                )),
                Rotation::PiFois2 => Some(LineBorder::new(
                    vec2(
                        tile.position[0] as f32 * TILE_SIZE,
                        tile.position[1] as f32 * TILE_SIZE,
                    ),
                    TILE_DIAG_SIZE,
                    3.0 * FRAC_PI_4,
                )),
            },
            TileType::SoftTurnExterior2 => match tile.rotation {
                Rotation::Pi => Some(LineBorder::new(
                    vec2(
                        (tile.position[0] + 1) as f32 * TILE_SIZE,
                        tile.position[1] as f32 * TILE_SIZE,
                    ),
                    TILE_DIAG_SIZE,
                    -3.0 * FRAC_PI_4,
                )),
                Rotation::PiFois3Sur2 => Some(LineBorder::new(
                    vec2(
                        (tile.position[0] + 1) as f32 * TILE_SIZE,
                        (tile.position[1] + 1) as f32 * TILE_SIZE,
                    ),
                    TILE_DIAG_SIZE,
                    -FRAC_PI_4,
                )),
                Rotation::PiFois2 => Some(LineBorder::new(
                    vec2(
                        tile.position[0] as f32 * TILE_SIZE,
                        (tile.position[1] + 1) as f32 * TILE_SIZE,
                    ),
                    TILE_DIAG_SIZE,
                    FRAC_PI_4,
                )),
                Rotation::PiSur2 => Some(LineBorder::new(
                    vec2(
                        tile.position[0] as f32 * TILE_SIZE,
                        tile.position[1] as f32 * TILE_SIZE,
                    ),
                    TILE_DIAG_SIZE,
                    3.0 * FRAC_PI_4,
                )),
            },
            _ => None,
        }
    }

    pub fn draw(&self) {
        draw_line(
            self.start.x,
            self.start.y,
            self.end().x,
            self.end().y,
            1.0,
            RED,
        );
        draw_circle(self.start.x, self.start.y, 1.5, RED);
    }
}

pub trait RectHitbox {
    // (x,y) the center of the hitbox
    fn rect(&self) -> Rect;
    fn rotation(&self) -> f32;

    fn points(&self) -> [Vec2; 4] {
        let cos_rot = self.rotation().cos();
        let sin_rot = self.rotation().sin();
        [
            // TL
            vec2(
                (-self.rect().w * cos_rot + self.rect().h * sin_rot) * 0.5 + self.rect().x,
                (-self.rect().w * sin_rot - self.rect().h * cos_rot) * 0.5 + self.rect().y,
            ),
            // TR
            vec2(
                (self.rect().w * cos_rot + self.rect().h * sin_rot) * 0.5 + self.rect().x,
                (self.rect().w * sin_rot - self.rect().h * cos_rot) * 0.5 + self.rect().y,
            ),
            // BR
            vec2(
                (self.rect().w * cos_rot - self.rect().h * sin_rot) * 0.5 + self.rect().x,
                (self.rect().w * sin_rot + self.rect().h * cos_rot) * 0.5 + self.rect().y,
            ),
            // BL
            vec2(
                (-self.rect().w * cos_rot - self.rect().h * sin_rot) * 0.5 + self.rect().x,
                (-self.rect().w * sin_rot + self.rect().h * cos_rot) * 0.5 + self.rect().y,
            ),
        ]
    }

    fn rotated_points(&self, rotation: f32) -> [Vec2; 4] {
        let cos_rot = rotation.cos();
        let sin_rot = -rotation.sin();
        let points = self.points();
        [
            vec2(
                points[0].x * cos_rot - points[0].y * sin_rot,
                points[0].x * sin_rot + points[0].y * cos_rot,
            ),
            vec2(
                points[1].x * cos_rot - points[1].y * sin_rot,
                points[1].x * sin_rot + points[1].y * cos_rot,
            ),
            vec2(
                points[2].x * cos_rot - points[2].y * sin_rot,
                points[2].x * sin_rot + points[2].y * cos_rot,
            ),
            vec2(
                points[3].x * cos_rot - points[3].y * sin_rot,
                points[3].x * sin_rot + points[3].y * cos_rot,
            ),
        ]
    }

    fn draw_hitbox(&self) {
        let points = self.points();
        draw_line(points[0].x, points[0].y, points[1].x, points[1].y, 1.0, RED);
        draw_line(points[1].x, points[1].y, points[2].x, points[2].y, 1.0, RED);
        draw_line(points[2].x, points[2].y, points[3].x, points[3].y, 1.0, RED);
        draw_line(points[3].x, points[3].y, points[0].x, points[0].y, 1.0, RED);
        draw_circle(self.rect().x, self.rect().y, 1.0, RED)
    }
}

impl Collider<LineBorder> for Vec2 {
    fn collides(&self, other: LineBorder) -> bool {
        let point = vec2(
            self.x * other.rotation.cos() + self.y * other.rotation.sin(),
            -self.x * other.rotation.sin() + self.y * other.rotation.cos(),
        );
        let start = other.rotated_start();
        point.x > start.x && point.y < start.y && point.y > (start.y - other.lengh)
    }
}
