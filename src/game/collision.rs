use std::f32::consts::PI;

use macroquad::math::{vec2, Rect, Vec2};

use super::{Player, Tile, TILE_SIZE};

pub trait Collider {
    fn collides<T: Collider>(&self, other: T) -> bool;
    fn top(&self) -> Vec2;
    fn bottom(&self) -> Vec2;
    fn left(&self) -> Vec2;
    fn right(&self) -> Vec2;
    fn rotation(&self) -> f32;
}

pub struct LineBorder {
    start: Vec2,
    lengh: f32,
    // rotation around start
    rotation: f32,
}

impl Collider for LineBorder {
    fn collides<T: Collider>(&self, other: T) -> bool {
        todo!()
    }
    fn top(&self) -> Vec2 {
        if self.rotation <= PI {
            self.start
        } else {
            vec2(
                self.start.y * self.start.x + self.start.y * self.start.y,
                self.lengh,
            )
        }
    }
    fn bottom(&self) -> Vec2 {
        todo!()
    }
    fn left(&self) -> Vec2 {
        todo!()
    }
    fn right(&self) -> Vec2 {
        todo!()
    }
    fn rotation(&self) -> f32 {
        self.rotation
    }
}

// impl LineBorder {
//     pub fn from_tile(tile: Tile) -> Option<LineBorder> {
//         match tile.mapatlas_source {
//             [1, 0] => {
//                 Some(LineBorder { points: vec![] })
//             }
//             _ => None,
//         }
//     }
// }

// pub struct TileHitbox {
//     rect: Rect,
//     dir: Direction,
// }

// impl Collider for TileHitbox {
//     fn collides<T: Collider>(&self, other: T) -> bool {
//         todo!()
//     }

//     fn top(&self) -> f32 {
//         match self.dir {
//             Direction::Straight => self.rect.y - self.rect.h * 0.5,
//             Direction::Diag => (self.rect.y),
//         }
//     }

//     fn bottom(&self) -> f32 {
//         todo!()
//     }

//     fn left(&self) -> f32 {
//         todo!()
//     }

//     fn right(&self) -> f32 {
//         todo!()
//     }

//     fn rect(&self) -> Rect {
//         self.rect
//     }
// }

// impl TileHitbox {
//     // pub fn from_tile(tile: Tile) -> Option<Self> {
//     //     match tile.mapatlas_source {
//     //         [1, 0] => match tile.rotation {
//     //             // on the bottom right
//     //             0 => Some(Self {
//     //                 rect: Rect {
//     //                     x: tile.position[0] as f32 * TILE_SIZE + 20.0,
//     //                     y: tile.position[1] as f32 * TILE_SIZE + 20.0,
//     //                     w: 4.0,
//     //                     h: 4.0,
//     //                 },
//     //             }),
//     //             // on the bottom left
//     //             1 => Some(Self {
//     //                 rect: Rect {
//     //                     x: tile.position[0] as f32 * TILE_SIZE,
//     //                     y: tile.position[1] as f32 * TILE_SIZE + 20.0,
//     //                     w: 4.0,
//     //                     h: 4.0,
//     //                 },
//     //             }), // on the top left
//     //             2 => Some(Self {
//     //                 rect: Rect {
//     //                     x: tile.position[0] as f32 * TILE_SIZE,
//     //                     y: tile.position[1] as f32 * TILE_SIZE,
//     //                     w: 4.0,
//     //                     h: 4.0,
//     //                 },
//     //             }),
//     //             // on the top right
//     //             3 => Some(Self {
//     //                 rect: Rect {
//     //                     x: tile.position[0] as f32 * TILE_SIZE + 20.0,
//     //                     y: tile.position[1] as f32 * TILE_SIZE,
//     //                     w: 4.0,
//     //                     h: 4.0,
//     //                 },
//     //             }),
//     //             _ => None,
//     //         },
//     //         [4, 0] => match tile.rotation {
//     //             // bottom
//     //             0 => Some(Self {
//     //                 rect: Rect {
//     //                     x: tile.position[0] as f32 * TILE_SIZE,
//     //                     y: tile.position[1] as f32 * TILE_SIZE + 20.0,
//     //                     w: TILE_SIZE,
//     //                     h: 4.0,
//     //                 },
//     //             }),
//     //             // left
//     //             1 => Some(Self {
//     //                 rect: Rect {
//     //                     x: tile.position[0] as f32 * TILE_SIZE,
//     //                     y: tile.position[1] as f32 * TILE_SIZE,
//     //                     w: 4.0,
//     //                     h: TILE_SIZE,
//     //                 },
//     //             }),
//     //             // top
//     //             2 => Some(Self {
//     //                 rect: Rect {
//     //                     x: tile.position[0] as f32 * TILE_SIZE,
//     //                     y: tile.position[1] as f32 * TILE_SIZE,
//     //                     w: TILE_SIZE,
//     //                     h: 4.0,
//     //                 },
//     //             }),
//     //             // right
//     //             3 => Some(Self {
//     //                 rect: Rect {
//     //                     x: tile.position[0] as f32 * TILE_SIZE + 20.0,
//     //                     y: tile.position[1] as f32 * TILE_SIZE,
//     //                     w: 4.0,
//     //                     h: TILE_SIZE,
//     //                 },
//     //             }),
//     //             _ => None,
//     //         },
//     //         _ => None,
//     //     }
//     // }
// }

pub struct PlayerHitbox {
    // (x,y) the center of the hitbox
    rect: Rect,
    rotation: f32,
}

impl From<&Player> for PlayerHitbox {
    fn from(player: &Player) -> Self {
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

impl Collider for PlayerHitbox {
    fn collides<T: Collider>(&self, other: T) -> bool {
        todo!()
    }

    fn top(&self) -> Vec2 {
        todo!()
    }

    fn bottom(&self) -> Vec2 {
        todo!()
    }

    fn left(&self) -> Vec2 {
        todo!()
    }

    fn right(&self) -> Vec2 {
        todo!()
    }

    fn rotation(&self) -> f32 {
        self.rotation
    }
}
