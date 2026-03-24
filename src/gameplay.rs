use bevy::app::Update;
use bevy::asset::{AssetServer, Assets};
use bevy::ecs::component::Component;
use bevy::ecs::lifecycle::Add;
use bevy::ecs::observer::On;
use bevy::ecs::query::{Has, With};
use bevy::ecs::resource::Resource;
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::ecs::system::{Commands, Query, Res, ResMut};
use bevy::image::{TextureAtlas, TextureAtlasLayout};
use bevy::math::{UVec2, Vec3};
use bevy::prelude::{Plugin, Transform};
use bevy::sprite::Sprite;
use bevy::state::app::AppExtStates;
use bevy::state::condition::in_state;
use bevy::state::state::NextState;
use bevy::state::state_scoped::DespawnOnExit;
use bevy::time::{Stopwatch, Time, Timer, TimerMode};
use bevy_ecs_tiled::prelude::{MapCreated, TiledEvent, TiledMap};

use crate::animation::AnimationTimer;
use crate::car::{Car, CarSpawnPoint, HasFinished, Grounded, Progression};
use crate::states::{Menu, Pause, PlayingState};
use crate::tilemap::NumberOfLaps;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_sub_state::<PlayingState>()
            .insert_state(Pause(false))
            .insert_resource(TimeSinceStart(Stopwatch::new()))
            .add_observer(spawn_countdown)
            .add_observer(reset_stopwatch)
            // .add_observer(game_over)
            .add_systems(
                Update,
                (
                    play_countdown.run_if(in_state(PlayingState::Countdown)),
                    (update_stopwatch, game_over).run_if(in_state(PlayingState::Racing)),
                ),
            );
    }
}

#[derive(Component)]
pub struct StartCountdown(pub Timer);

impl Default for StartCountdown {
    fn default() -> Self {
        Self(Timer::from_seconds(5.0, TimerMode::Once))
    }
}

#[derive(Resource)]
pub struct TimeSinceStart(pub Stopwatch);

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
            }
        }
    }
}

fn update_stopwatch(mut stopwatch_res: ResMut<TimeSinceStart>, time: Res<Time>) {
    stopwatch_res.0.tick(time.delta());
}

fn reset_stopwatch(_: On<TiledEvent<MapCreated>>, mut stopwatch_res: ResMut<TimeSinceStart>) {
    stopwatch_res.0.reset();
}

// #[derive(EntityEvent)]
// pub struct CrossTheLine {
//     pub entity: Entity,
// }

// fn game_over_evt(
//     car_query: Query<(&Progression, &IsGrounded, &HasFinished), With<Car>>,
//     level_query: Query<&NumberOfLaps, With<TiledMap>>,
//     mut next_state: ResMut<NextState<PlayingState>>,
//     mut next_menu: ResMut<NextState<Menu>>,
// ) {
//     for progression in &car_query {
//         if level_query
//             .single()
//             .is_ok_and(|n_laps| progression.current_turn == n_laps.0 as u8 + 1)
//         {
//             // TODO: check if all the players have finished
//             next_state.set(PlayingState::End);
//             next_menu.set(Menu::GameOver);
//         }
//     }
// }

fn game_over(
    car_query: Query<(&Progression, Has<Grounded>, &HasFinished), With<Car>>,
    level_query: Query<&NumberOfLaps, With<TiledMap>>,
    mut next_state: ResMut<NextState<PlayingState>>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    // TODO: check if all the players have finished
    for (progression, is_grounded, has_finished) in &car_query {
        // player ended all laps
        if level_query
            .single()
            .is_ok_and(|n_laps| progression.current_turn == n_laps.0 as u8 + 1) 
            // player has fallen
            || !is_grounded
            // player has finished
            || has_finished.0
        {
            next_state.set(PlayingState::End);
            next_menu.set(Menu::GameOver);
        }
    }
}
