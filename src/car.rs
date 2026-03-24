use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::{
    animation::{compute_atlas_index, custom_layout, AnimationIndices, AnimationTimer},
    common::{
        CAR_ACCELERATION, CAR_ANIMATION_INDICES, CAR_BRAKE, CAR_NUM_ANIMATION, CAR_ROTATION,
        CAR_SPRITE_SIZE,
    },
    states::{GameState, Menu, PlayingState},
};

// ---- Plugin ---- //
pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CarSpawnPoint>();
        app.add_systems(
            Update,
            (deccelerate, animate_car, animate_falling).run_if(in_state(PlayingState::Racing)),
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

// TODO: change to sparseset component instead of bool
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Component)]
pub struct HasFinished(pub bool);

#[derive(Component)]
struct FallingTimer(Timer);

#[derive(Component)]
pub struct Progression {
    pub last_checkpoint: u8,
    pub current_turn: u8,
}

// Component storing time elapsed since begining of the race and the end of a lap
// Lap 0 ends LapsTime.0[0] seconds after the begining of the race,
// Lap 1 LapsTime.0[1] ...
#[derive(Component)]
pub struct LapsTime(pub Vec<f32>);

impl ToString for LapsTime {
    fn to_string(&self) -> String {
        let mut string_buffer = String::new();
        for (lap, time) in self
            .0
            .iter()
            .scan(0.0, |state, time| {
                let lap_time = time - *state;
                *state = *time;
                Some(lap_time)
            })
            .skip(1)
            .enumerate()
        {
            string_buffer.push_str(&format!("lap {}: {:.3}\n", lap + 1, time));
        }
        string_buffer
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
    has_finished: HasFinished,
    state: State,
}

#[derive(Bundle)]
struct AppearanceBundle {
    visibility: Visibility,
    sprite: Sprite,
    animation_indices: AnimationIndices<CAR_NUM_ANIMATION>,
    animation_timer: AnimationTimer,
}

#[derive(Bundle)]
struct ScoreBundle {
    lap_times: LapsTime,
    progression: Progression,
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
            has_finished: HasFinished(false),
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
            animation_indices: AnimationIndices::<CAR_NUM_ANIMATION> {
                index: 0,
                indices: CAR_ANIMATION_INDICES,
            },
            animation_timer: AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        },
        ScoreBundle {
            lap_times: LapsTime(Vec::new()),
            progression: Progression {
                last_checkpoint: 0,
                current_turn: 0,
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
            &mut Progression,
            &mut LapsTime,
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
        mut progression,
        mut laps_time,
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
        *progression = Progression {
            last_checkpoint: 0,
            current_turn: 0,
        };
        *laps_time = LapsTime(Vec::new());
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

fn animate_car(
    mut q_car: Query<
        (
            &RotationFactor,
            &State,
            &mut AnimationTimer,
            &mut AnimationIndices<CAR_NUM_ANIMATION>,
            &mut Sprite,
        ),
        With<Car>,
    >,
    time: Res<Time>,
) {
    for (rotation, state, mut timer, mut indices, mut sprite) in &mut q_car {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            let animation_line = match state {
                State::Neutral => 0,
                State::Accelerating => {
                    if rotation.0 > 0.2 {
                        // car turns on the left
                        sprite.flip_x = false;
                        2
                    } else if rotation.0 < -0.2 {
                        // car turns on the right
                        sprite.flip_x = true;
                        2
                    } else {
                        // car doesn't turn
                        1
                    }
                }
                State::Braking => 3,
                State::Falling => 0,
            };
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = compute_atlas_index(animation_line, indices.index, &indices.indices);
                indices.index = atlas.index;
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
