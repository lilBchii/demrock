use std::f32::consts::PI;

use macroquad::{
    color::RED,
    math::{vec2, Rect, Vec2},
    shapes::{draw_circle, draw_line},
};

use super::{Player, Tile, TileType, TILE_SIZE};

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

    pub fn draw(&self) {
        draw_line(
            self.start.x,
            self.start.y,
            self.end().x,
            self.end().y,
            1.0,
            RED,
        );
    }
}

#[derive(Clone, Copy)]
pub struct PlayerHitbox {
    // (x,y) the center of the hitbox
    rect: Rect,
    rotation: f32,
}

impl From<&mut Player> for PlayerHitbox {
    fn from(player: &mut Player) -> Self {
        Self {
            rect: Rect {
                x: player.position.x,
                y: player.position.y,
                w: 4.0,
                h: 4.0,
            },
            rotation: player.rotation,
        }
    }
}

impl PlayerHitbox {
    pub fn new(pos: Vec2, rot: f32) -> Self {
        Self {
            rect: Rect {
                x: pos.x,
                y: pos.y,
                w: 5.0,
                h: 10.0,
            },
            rotation: rot,
        }
    }

    pub fn points(self) -> [Vec2; 4] {
        let cos_rot = self.rotation.cos();
        let sin_rot = self.rotation.sin();
        [
            // TL
            vec2(
                (-self.rect.w * cos_rot + self.rect.h * sin_rot) * 0.5 + self.rect.x,
                (-self.rect.w * sin_rot - self.rect.h * cos_rot) * 0.5 + self.rect.y,
            ),
            // TR
            vec2(
                (self.rect.w * cos_rot + self.rect.h * sin_rot) * 0.5 + self.rect.x,
                (self.rect.w * sin_rot - self.rect.h * cos_rot) * 0.5 + self.rect.y,
            ),
            // BR
            vec2(
                (self.rect.w * cos_rot - self.rect.h * sin_rot) * 0.5 + self.rect.x,
                (self.rect.w * sin_rot + self.rect.h * cos_rot) * 0.5 + self.rect.y,
            ),
            // BL
            vec2(
                (-self.rect.w * cos_rot - self.rect.h * sin_rot) * 0.5 + self.rect.x,
                (-self.rect.w * sin_rot + self.rect.h * cos_rot) * 0.5 + self.rect.y,
            ),
        ]
    }

    pub fn rotated_points(self, rotation: f32) -> [Vec2; 4] {
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

    pub fn draw(&self) {
        let points = self.points();
        draw_line(points[0].x, points[0].y, points[1].x, points[1].y, 1.0, RED);
        draw_line(points[1].x, points[1].y, points[2].x, points[2].y, 1.0, RED);
        draw_line(points[2].x, points[2].y, points[3].x, points[3].y, 1.0, RED);
        draw_line(points[3].x, points[3].y, points[0].x, points[0].y, 1.0, RED);
        draw_circle(self.rect.x, self.rect.y, 1.0, RED)
    }
}

impl Collider<&LineBorder> for PlayerHitbox {
    fn collides(&self, other: &LineBorder) -> bool {
        let start = other.rotated_start();
        // check if player hitbox has a point at the right of the line
        self.rotated_points(other.rotation).iter().any(|point| {
            point.x > start.x && point.y < start.y && point.y > (start.y - other.lengh)
        })
    }
}
