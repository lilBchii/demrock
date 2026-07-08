use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate);
    }
}

#[derive(Component)]
pub struct AnimationIndices {
    // The index of the current animation frame
    frame_index: usize,
    // The number of frames for each animation
    indices: Vec<u32>,
}

impl AnimationIndices {
    pub fn with_indices(indices: &[u32]) -> Self {
        Self {
            frame_index: 0,
            indices: indices.into(),
        }
    }

    pub fn single_with_frames(n_frames: u32) -> Self {
        Self {
            frame_index: 0,
            indices: Vec::from([n_frames]),
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationIndex(pub usize);

pub fn animate(
    mut animation_query: Query<(
        &mut AnimationTimer,
        &AnimationIndex,
        &mut AnimationIndices,
        &mut Sprite,
    )>,
    time: Res<Time>,
) {
    for (mut timer, index, mut indices, mut sprite) in &mut animation_query {
        timer.0.tick(time.delta());
        if timer.0.is_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = compute_atlas_index(index.0, indices.frame_index, &indices.indices);
                indices.frame_index = atlas.index;
            }
        }
    }
}

// ---- Helper functions ---- //

pub fn custom_layout<const N: usize>(tile_size: UVec2, indices: [u32; N]) -> TextureAtlasLayout {
    let size = UVec2::new(indices.into_iter().max().unwrap_or(1), N as u32) * tile_size.x;
    let mut textures = Vec::new();
    for i in 0..N {
        let top_left_y = i as u32 * tile_size.y;
        for j in 0..indices[i] {
            let top_left = UVec2::new(j as u32 * tile_size.x, top_left_y);
            textures.push(URect {
                min: top_left,
                max: top_left + tile_size,
            });
        }
    }
    TextureAtlasLayout { size, textures }
}

// Computes atlas index
fn compute_atlas_index(
    animation_line: usize,
    animation_index: usize,
    animation_indices: &[u32],
) -> usize {
    let mut start_index = 0;
    for line in 0..animation_line {
        start_index += animation_indices[line];
    }
    let end_index = (start_index + animation_indices[animation_line]) as usize - 1;
    let atlas_index = if animation_index >= end_index || (animation_index as u32) < start_index {
        start_index as usize
    } else {
        animation_index + 1
    };
    atlas_index
}
