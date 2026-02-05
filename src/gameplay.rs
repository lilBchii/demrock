use bevy::app::Update;
use bevy::asset::{AssetServer, Assets};
use bevy::ecs::component::Component;
use bevy::ecs::lifecycle::Add;
use bevy::ecs::observer::On;
use bevy::ecs::query::With;
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::ecs::system::{Commands, Query, Res, ResMut};
use bevy::image::{TextureAtlas, TextureAtlasLayout};
use bevy::math::{UVec2, Vec3};
use bevy::prelude::{Plugin, StateSet, Transform};
use bevy::sprite::Sprite;
use bevy::state::app::AppExtStates;
use bevy::state::condition::in_state;
use bevy::state::state::{NextState, SubStates};
use bevy::state::state_scoped::DespawnOnExit;
use bevy::time::{Time, Timer, TimerMode};

use crate::animation::AnimationTimer;
use crate::car::CarSpawnPoint;
use crate::common::AppState;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_sub_state::<PlayingState>()
            .add_observer(spawn_countdown)
            .add_systems(
                Update,
                play_countdown.run_if(in_state(PlayingState::Countdown)),
            );
    }
}

#[derive(Clone, PartialEq, Eq, Default, Hash, Debug, SubStates)]
#[source(AppState = AppState::Playing)]
pub enum PlayingState {
    #[default]
    Countdown,
    Racing,
}

#[derive(Component)]
pub struct StartCountdown(pub Timer);

impl Default for StartCountdown {
    fn default() -> Self {
        Self(Timer::from_seconds(5.0, TimerMode::Once))
    }
}

fn spawn_countdown(
    add_spawn: On<Add, CarSpawnPoint>,
    mut commands: Commands,
    countdown_query: Query<&Transform, With<CarSpawnPoint>>,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(12),
        5,
        1,
        None,
        None,
    ));

    // TODO: manage error
    let Ok(spawn_pos) = countdown_query.get(add_spawn.event().entity) else {
        return;
    };

    commands.spawn((
        StartCountdown::default(),
        Sprite::from_atlas_image(
            asset_server.load("countdown.png"),
            TextureAtlas { layout, index: 0 },
        ),
        AnimationTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
        DespawnOnExit(PlayingState::Countdown),
        Transform::from_translation(Vec3::new(
            spawn_pos.translation.x,
            spawn_pos.translation.y + 80.0,
            1.0,
        ))
        .with_scale(Vec3::splat(2.0)),
    ));
}

fn play_countdown(
    mut countdown: Query<(&mut StartCountdown, &mut AnimationTimer, &mut Sprite)>,
    mut next_state: ResMut<NextState<PlayingState>>,
    time: Res<Time>,
) {
    for (mut countdown_timer, mut animation_timer, mut sprite) in countdown.iter_mut() {
        countdown_timer.0.tick(time.delta());
        animation_timer.0.tick(time.delta());
        if countdown_timer.0.just_finished() {
            next_state.set(PlayingState::Racing);
        }
        if animation_timer.0.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index >= 4 {
                    atlas.index = 0;
                } else {
                    atlas.index += 1;
                }
                println!(
                    countdown_timer.0.elapsed_secs(),
                    atlas.index
                );
            }
        }
    }
}
