use macroquad::math::Rect;

use super::{Tile, TILE_SIZE};

pub trait Collider {
    //fn collides<T: Collider>(&self, other: T) -> bool;
}

pub struct StraightHitbox {
    rect: Rect,
}

impl Collider for StraightHitbox {
    // fn collides<CarHitbox>(&self, other: CarHitbox) -> bool {
    //     true
    // }
}

pub struct CarHitbox {
    rect: Rect,
    rotation: f32,
}

// impl CarHitbox {

// }

// impl Collider for CarHitbox {
//     fn collides(&self, other: StraightHitbox) -> bool {
//         true
//     }
// }

// impl StraightHitbox {
//     pub fn new(rect: Rect) -> Self {
//         Self { rect }
//     }

//     pub fn from_tile(tile: Tile) -> Option<Self> {
//         match tile.mapatlas_source {
//             [1, 0] => match tile.rotation {
//                 // on the bottom right
//                 0 => Some(Self {
//                     rect: Rect {
//                         x: tile.position[0] as f32 * TILE_SIZE + 20.0,
//                         y: tile.position[1] as f32 * TILE_SIZE + 20.0,
//                         w: 4.0,
//                         h: 4.0,
//                     },
//                 }),
//                 // on the bottom left
//                 1 => Some(Self {
//                     rect: Rect {
//                         x: tile.position[0] as f32 * TILE_SIZE,
//                         y: tile.position[1] as f32 * TILE_SIZE + 20.0,
//                         w: 4.0,
//                         h: 4.0,
//                     },
//                 }), // on the top left
//                 2 => Some(Self {
//                     rect: Rect {
//                         x: tile.position[0] as f32 * TILE_SIZE,
//                         y: tile.position[1] as f32 * TILE_SIZE,
//                         w: 4.0,
//                         h: 4.0,
//                     },
//                 }),
//                 // on the top right
//                 3 => Some(Self {
//                     rect: Rect {
//                         x: tile.position[0] as f32 * TILE_SIZE + 20.0,
//                         y: tile.position[1] as f32 * TILE_SIZE,
//                         w: 4.0,
//                         h: 4.0,
//                     },
//                 }),
//                 _ => None,
//             },
//             [4, 0] => match tile.rotation {
//                 // bottom
//                 0 => Some(Self {
//                     rect: Rect {
//                         x: tile.position[0] as f32 * TILE_SIZE,
//                         y: tile.position[1] as f32 * TILE_SIZE + 20.0,
//                         w: TILE_SIZE,
//                         h: 4.0,
//                     },
//                 }),
//                 // left
//                 1 => Some(Self {
//                     rect: Rect {
//                         x: tile.position[0] as f32 * TILE_SIZE,
//                         y: tile.position[1] as f32 * TILE_SIZE,
//                         w: 4.0,
//                         h: TILE_SIZE,
//                     },
//                 }),
//                 // top
//                 2 => Some(Self {
//                     rect: Rect {
//                         x: tile.position[0] as f32 * TILE_SIZE,
//                         y: tile.position[1] as f32 * TILE_SIZE,
//                         w: TILE_SIZE,
//                         h: 4.0,
//                     },
//                 }),
//                 // right
//                 3 => Some(Self {
//                     rect: Rect {
//                         x: tile.position[0] as f32 * TILE_SIZE + 20.0,
//                         y: tile.position[1] as f32 * TILE_SIZE,
//                         w: 4.0,
//                         h: TILE_SIZE,
//                     },
//                 }),
//                 _ => None,
//             },
//             _ => None,
//         }
//     }
// }
