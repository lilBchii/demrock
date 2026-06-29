use std::fmt::Display;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::{
    animation::{custom_layout, AnimationIndex, AnimationIndices, AnimationTimer},
    common::{
        CAR_ACCELERATION, CAR_ANIMATION_INDICES, CAR_BRAKE, CAR_NUM_ANIMATION, CAR_ROTATION,
        CAR_SPRITE_SIZE,
    },
    gamemodes::ARCADE_NUM_RACES,
    states::{GameState, Menu, PlayingState},
};

// ---- Plugin ---- //
pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CarSpawnPoint>();
        app.add_systems(
            Update,
            (deccelerate, update_state, animate_falling).run_if(in_state(PlayingState::Racing)),
        )
        .add_systems(OnEnter(PlayingState::Racing), enable_input)
        .add_systems(OnEnter(GameState::Playing), spawn_car)
        .add_systems(OnExit(Menu::None), disable_input)
        .add_input_context::<Car>()
        .add_observer(setup_car)
        .add_observer(accelerate)
        .add_observer(rotate)
        .add_observer(input_cancel_acceleration)
        .add_observer(input_brake)
        .add_observer(input_cancel_brake);
    }
}

// ---- Components ---- //
#[derive(Component)]
struct RotationFactor(f32);

#[derive(Component)]
pub struct Car;

// All statistics of a car, these can be set by the player before a race and
// should not change during the race
#[derive(Component)]
struct Config {
    rotation_speed: f32,
    acceleration: f32,
    brake: f32,
}

#[derive(Component)]
pub enum State {
    Accelerating,
    Falling,
    Braking,
    Neutral,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Component)]
struct FallingTimer(Timer);

// Actual progression of the player during a race
#[derive(Component)]
pub struct RaceProgression {
    pub last_checkpoint: u8,
    pub current_lap: u8,
}

#[derive(Component, Debug)]
pub struct GameProgression {
    n_race_finished: usize,
    races_time: Vec<LapsTime>,
    current_race: usize,
}

impl GameProgression {
    pub fn new() -> Self {
        let mut races_time = Vec::with_capacity(ARCADE_NUM_RACES);
        races_time.push(LapsTime::new());
        Self {
            n_race_finished: 0,
            current_race: 0,
            races_time,
        }
    }

    pub fn push_laps_times(&mut self, laps_time: LapsTime) {
        self.races_time.push(laps_time);
        // self.current_race += 1;
    }

    pub fn incr_n_race(&mut self) {
        self.n_race_finished += 1;
    }

    pub fn next_race(&mut self) {
        self.current_race += 1;
    }

    pub fn current_race_times(&self) -> &LapsTime {
        &self.races_time[self.current_race]
    }

    pub fn current_race_times_mut(&mut self) -> &mut LapsTime {
        &mut self.races_time[self.current_race]
    }

    pub fn races_times(&self) -> &Vec<LapsTime> {
        &self.races_time
    }
}

// Time for each lap of a race
#[derive(Clone, Debug)]
pub struct LapsTime(Vec<f32>);

impl LapsTime {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add_time_from_full_timer(&mut self, timer: f32) {
        let prev_laps_total_t: f32 = self.0.iter().sum();
        self.0.push(timer - prev_laps_total_t);
    }
}

impl Display for LapsTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (lap, t) in self.0.iter().enumerate() {
            writeln!(f, "lap {}: {:.3}", lap + 1, t)?;
        }
        Ok(())
    }
}

#[derive(Bundle)]
struct PhysicsBundle {
    transform: Transform,
    rotation_factor: RotationFactor,
    linear_velocity: LinearVelocity,
    collider: Collider,
    body: RigidBody,
    colliding_entities: CollidingEntities,
    interpolation: TransformInterpolation,
}

#[derive(Bundle)]
struct StateBundle {
    state: State,
}

#[derive(Bundle)]
struct AppearanceBundle {
    visibility: Visibility,
    sprite: Sprite,
    animation_indices: AnimationIndices,
    animation_line: AnimationIndex,
    animation_timer: AnimationTimer,
}

#[derive(Bundle)]
struct ScoreBundle {
    game_progression: GameProgression,
    race_progression: RaceProgression,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[require(Transform)]
#[reflect(Component)]
pub struct CarSpawnPoint;

// ---- Sytems ---- //

fn spawn_car(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let tile_size = UVec2::from(CAR_SPRITE_SIZE);
    let atlas_layout_handle = atlas_layouts.add(custom_layout::<CAR_NUM_ANIMATION>(
        tile_size,
        CAR_ANIMATION_INDICES,
    ));

    commands.spawn((
        Car,
        Config {
            rotation_speed: CAR_ROTATION,
            acceleration: CAR_ACCELERATION,
            brake: CAR_BRAKE,
        },
        StateBundle {
            state: State::Neutral,
        },
        AppearanceBundle {
            visibility: Visibility::Hidden,
            sprite: Sprite::from_atlas_image(
                asset_server.load("racer.png"),
                TextureAtlas {
                    layout: atlas_layout_handle,
                    index: 0,
                },
            ),
            animation_indices: AnimationIndices::with_indices(&CAR_ANIMATION_INDICES),
            animation_line: AnimationIndex(0),
            animation_timer: AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        },
        ScoreBundle {
            game_progression: GameProgression::new(),
            race_progression: RaceProgression {
                last_checkpoint: 0,
                current_lap: 0,
            },
        },
        PhysicsBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            rotation_factor: RotationFactor(0.0),
            linear_velocity: LinearVelocity::ZERO,
            collider: Collider::rectangle(2.0, 5.0),
            body: RigidBody::Kinematic,
            colliding_entities: CollidingEntities::default(),
            interpolation: TransformInterpolation,
        },
        FallingTimer(Timer::from_seconds(4.0, TimerMode::Once)),
        actions!(Car[
            (
                Action::<Accelerate>::new(),
                SmoothNudge::default(),
                bindings![KeyCode::ArrowUp, GamepadButton::RightTrigger2],
            ),
            (
                Action::<Brake>::new(),
                bindings![KeyCode::ArrowDown, GamepadButton::LeftTrigger2],
            ),
            (
                Action::<Rotate>::new(),
                DeadZone {
                    kind: DeadZoneKind::Radial,
                    lower_threshold: 0.3,
                    upper_threshold: 0.98,
                },
                SmoothNudge::default(),
                Bindings::spawn((
                    Bidirectional::new(KeyCode::ArrowRight, KeyCode::ArrowLeft),
                    Axial::left_stick(),
                )),
            ),
        ]),
        ContextActivity::<Car>::INACTIVE,
        DespawnOnExit(GameState::Playing),
    ));
}

// This spawns physics components of the car when a spawn point has spawned.
// It allows to reset the physics of the car when changing map.
fn setup_car(
    add_car_spawn: On<Add, CarSpawnPoint>,
    mut commands: Commands,
    spawn_point_query: Query<&Transform, (With<CarSpawnPoint>, Without<Car>)>,
    car_query: Query<
        (
            Entity,
            &mut Transform,
            &mut RotationFactor,
            &mut LinearVelocity,
            &mut State,
            &mut RaceProgression,
            &mut Visibility,
        ),
        With<Car>,
    >,
) {
    // TODO: consider multiple spawn point with multiplayer for instance
    let Ok(spawn_pos) = spawn_point_query.get(add_car_spawn.event().entity) else {
        return;
    };
    for (
        entity,
        mut transform,
        mut rotation_factor,
        mut linear_velocity,
        mut state,
        mut race_progression,
        mut visibility,
    ) in car_query
    {
        *transform = Transform::from_translation(Vec3::new(
            spawn_pos.translation.x,
            spawn_pos.translation.y,
            1.0,
        ));
        *rotation_factor = RotationFactor(0.0);
        *linear_velocity = LinearVelocity::ZERO;
        *state = State::Neutral;
        *race_progression = RaceProgression {
            last_checkpoint: 0,
            current_lap: 0,
        };
        *visibility = Visibility::Visible;
        commands
            .entity(entity)
            .remove::<(ColliderDisabled, RigidBodyDisabled)>()
            .insert(Grounded);
    }
}

fn enable_input(mut commands: Commands, player_query: Query<Entity, With<Car>>) {
    for entity in player_query {
        commands
            .entity(entity)
            .insert(ContextActivity::<Car>::ACTIVE);
    }
}

fn disable_input(mut commands: Commands, player_query: Query<Entity, With<Car>>) {
    for entity in player_query {
        commands
            .entity(entity)
            .insert(ContextActivity::<Car>::INACTIVE);
    }
}

fn hide_car(car_query: Query<&mut Visibility, With<Car>>) {
    for mut visibility in car_query {
        if matches!(*visibility, Visibility::Visible) {
            *visibility = Visibility::Hidden;
        }
    }
}

fn show_car(car_query: Query<&mut Visibility, With<Car>>) {
    for mut visibility in car_query {
        if matches!(*visibility, Visibility::Hidden) {
            *visibility = Visibility::Visible;
        }
    }
}

fn deccelerate(
    mut query: Query<(&mut LinearVelocity, &mut AngularVelocity), (With<Car>, With<Grounded>)>,
) {
    for (mut velocity, mut rotation) in query.iter_mut() {
        rotation.0 *= 0.88;
        velocity.0 *= 0.98;
    }
}

fn update_state(
    mut appearance_query: Query<(
        &RotationFactor,
        &mut AnimationIndex,
        &mut Sprite,
        Ref<State>,
    )>,
) {
    for (rotation, mut animation_index, mut sprite, state) in &mut appearance_query {
        if state.is_changed() {
            match *state {
                State::Neutral => {
                    *animation_index = AnimationIndex(0);
                }
                State::Accelerating => {
                    if rotation.0 > 0.2 {
                        // car turns on the left
                        sprite.flip_x = false;
                        *animation_index = AnimationIndex(2);
                    } else if rotation.0 < -0.2 {
                        // car turns on the right
                        sprite.flip_x = true;
                        *animation_index = AnimationIndex(2);
                    } else {
                        // car doesn't turn
                        *animation_index = AnimationIndex(1);
                    }
                }
                State::Braking => {
                    *animation_index = AnimationIndex(3);
                }
                State::Falling => {
                    *animation_index = AnimationIndex(0);
                }
            }
        }
    }
}

fn animate_falling(
    mut car_query: Query<
        (
            &mut Transform,
            &mut LinearVelocity,
            &mut AngularVelocity,
            &mut FallingTimer,
            &State,
        ),
        With<Car>,
    >,
    mut next_menu: ResMut<NextState<Menu>>,
    mut next_state: ResMut<NextState<PlayingState>>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity, mut angular_velocity, mut timer, state) in &mut car_query {
        if matches!(state, State::Falling) {
            timer.0.tick(time.delta());
            if timer.0.is_finished() {
                next_state.set(PlayingState::End);
                // TODO: change this to RaceOver
                next_menu.set(Menu::GameOver);
            } else {
                transform.scale = (transform.scale - 0.25 * time.delta_secs()).max(Vec3::ZERO);
                transform.rotate_z(1.5 * time.delta_secs());
                velocity.0 *= 0.95;
                angular_velocity.0 = 0.0;
            }
        }
    }
}

// ---- Actions ---- //

#[derive(InputAction)]
#[action_output(f32)]
struct Accelerate;

#[derive(InputAction)]
#[action_output(f32)]
struct Brake;

#[derive(InputAction)]
#[action_output(f32)]
struct Rotate;

fn accelerate(
    acceleration: On<Fire<Accelerate>>,
    mut query: Query<
        (&mut LinearVelocity, &Transform, &mut State, &Config),
        (With<Car>, With<Grounded>),
    >,
    time: Res<Time>,
) {
    if let Ok((mut velocity, transform, mut state, config)) = query.get_mut(acceleration.context) {
        let dir = (transform.rotation * Vec3::Y).truncate();
        velocity.0 +=
            Vec2::splat(acceleration.value * config.acceleration * time.delta_secs()) * dir;
        *state = State::Accelerating;
    }
}

fn rotate(
    rotation: On<Fire<Rotate>>,
    mut q_rotation: Query<
        (&mut RotationFactor, &mut AngularVelocity, &Config),
        (With<Car>, With<Grounded>),
    >,
    time: Res<Time>,
) {
    if let Ok((mut rotation_factor, mut angular_velocity, config)) =
        q_rotation.get_mut(rotation.context)
    {
        angular_velocity.0 -= rotation.value * config.rotation_speed * time.delta_secs();
        rotation_factor.0 = -rotation.value;
    }
}

fn input_brake(
    brake: On<Fire<Brake>>,
    mut query: Query<(&mut LinearVelocity, &mut State, &Config), (With<Car>, With<Grounded>)>,
) {
    if let Ok((mut velocity, mut state, config)) = query.get_mut(brake.context) {
        velocity.0 *= 1.0 - brake.value * config.brake;
        *state = State::Braking;
    }
}

fn input_cancel_acceleration(
    acceleration: On<Complete<Accelerate>>,
    mut query: Query<&mut State, With<Car>>,
) {
    if let Ok(mut state) = query.get_mut(acceleration.context) {
        *state = State::Neutral;
    }
}

fn input_cancel_brake(brake: On<Complete<Brake>>, mut query: Query<&mut State, With<Car>>) {
    if let Ok(mut state) = query.get_mut(brake.context) {
        *state = State::Neutral;
    }
}
