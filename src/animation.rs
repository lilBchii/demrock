use bevy::prelude::*;

use crate::common::AppState;

#[derive(Component)]
pub struct AnimationIndices<const N: usize> {
    // The index of the current animation frame
    pub index: usize,
    // The number of frames for each animation
    pub indices: [u32; N],
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn custom_layout<const N: usize>(tile_size: UVec2, indices: [u32; N]) -> TextureAtlasLayout {
    let size = UVec2::new(indices.into_iter().max().unwrap_or(1), N as u32) * tile_size.x;
    let mut textures = Vec::new();
    for i in 0..N {
        let top_left_y = i as u32 * tile_size.y;
        for j in 0..indices[i] {
            let top_left = UVec2::new(j as u32 * tile_size.x, top_left_y);
            textures.push(URect {
                min: top_left,
                max: top_left + tile_size
            });
        }
    }
    TextureAtlasLayout {
        size,
        textures,
    }
}

// Computes atlas index
pub fn compute_atlas_index(
    animation_line: usize,
    animation_index: usize,
    animation_indices: &[u32]
) -> usize {
    let mut start_index = 0;
    for line in 0..animation_line {
        start_index += animation_indices[line];
    }
    let end_index = start_index + animation_indices[animation_line] - 1;
    println!("start {} || end {}", start_index, end_index);
    let atlas_index = if (animation_index as u32) >= end_index
        || (animation_index as u32) < start_index
    {
        start_index as usize
    } else {
        animation_index + 1
    };
    println!("actual index {}", atlas_index);
    atlas_index
}
