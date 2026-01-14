use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::{
    animation::{custom_layout, compute_atlas_index, AnimationIndices, AnimationTimer},
    common::{
        AppState, CAR_ACCELERATION, CAR_ANIMATION_INDICES, CAR_BRAKE, CAR_NUM_ANIMATION,
        CAR_ROTATION, CAR_SPRITE_SIZE,
    },
};

// ---- Plugin ---- //
pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                apply_movement,
                animate_neutral,
                animate_acceleration,
                animate_brake,
            )
                .run_if(in_state(AppState::Playing)),
        )
        .add_input_context::<Car>()
        .add_observer(input_acceleration)
        .add_observer(input_cancel_acceleration)
        .add_observer(input_rotation)
        .add_observer(input_brake)
        .add_observer(input_cancel_brake);
    }
}

// ---- Components ---- //

// Describes velocity along x and y axis
#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
struct RotationFactor(f32);

#[derive(Component)]
struct BrakeFactor(f32);

// Marker for a car
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
enum State {
    Accelerating,
    Falling,
    Braking,
    Neutral,
}

#[derive(Bundle)]
struct CarBundle {
    marker: Car,
    config: Config,
    // inputs
    velocity: Velocity,
    rotation_factor: RotationFactor,
    brake_factor: BrakeFactor,
    state: State,
    // physics
    transform: Transform,
    collider: Collider,
    body: RigidBody,
    // appearence
    sprite: Sprite,
    animation_indices: AnimationIndices<CAR_NUM_ANIMATION>,
    animation_timer: AnimationTimer,
}

// ---- Sytems ---- //

pub fn spawn_car(
    commands: &mut Commands,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: &Res<AssetServer>,
    posx: f32,
    posy: f32,
) {
    let tile_size = UVec2::from(CAR_SPRITE_SIZE);
    let atlas_layout_handle = atlas_layouts.add(custom_layout::<CAR_NUM_ANIMATION>(
        tile_size,
        CAR_ANIMATION_INDICES,
    ));

    commands.spawn((
        CarBundle {
            marker: Car,
            config: Config {
                rotation_speed: CAR_ROTATION,
                acceleration: CAR_ACCELERATION,
                brake: CAR_BRAKE,
            },
            velocity: Velocity(Vec2::ZERO),
            rotation_factor: RotationFactor(0.0),
            brake_factor: BrakeFactor(0.0),
            state: State::Neutral,
            transform: Transform::from_translation(Vec3::new(posx, posy, 1.0)),
            collider: Collider::rectangle(10.0, 32.0),
            body: RigidBody::Dynamic,
            sprite: Sprite::from_atlas_image(
                asset_server.load("anim_test.png"),
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
    ));
}

fn apply_movement(
    mut query: Query<
        (
            &mut Transform,
            &mut Velocity,
            &RotationFactor,
            &BrakeFactor,
            &Config,
        ),
        With<Car>,
    >,
    time: Res<Time>,
) {
    for (mut transform, mut velocity, rotation_factor, brake_factor, car_config) in query.iter_mut()
    {
        // Apply rotation
        transform.rotate_z(rotation_factor.0 * car_config.rotation_speed * time.delta_secs());
        // get the direction in which the front of the car is looking at
        let movement_direction = transform.rotation * Vec3::Y;

        // Translate the car
        transform.translation += (
            velocity.0.extend(0.0) * car_config.acceleration // Apply acceleration
                * (1.0 - brake_factor.0 * car_config.brake)
            // Apply brake
        ) * movement_direction
            * time.delta_secs();

        // Apply deceleration
        velocity.0 *= 0.98;
    }
}

// ---- Animation Systems --- //

fn animate_neutral(
    mut q_car: Query<
        (
            &State,
            &mut AnimationTimer,
            &mut AnimationIndices<CAR_NUM_ANIMATION>,
            &mut Sprite,
        ),
        With<Car>,
    >,
    time: Res<Time>,
) {
    for (state, mut timer, mut indices, mut sprite) in &mut q_car {
        if matches!(state, State::Neutral) {
            timer.0.tick(time.delta());
            if timer.0.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    let animation_line = 0;
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.index = compute_atlas_index(animation_line, indices.index, &indices.indices);
                        indices.index = atlas.index;
                    }
                }
            }
        }
    }
}

fn animate_acceleration(
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
    for (rotation_factor, state, mut timer, mut indices, mut sprite) in &mut q_car {
        if matches!(state, State::Accelerating) {
            timer.0.tick(time.delta());
            if timer.0.just_finished() {
                // Set right animation index according to car rotation
                let animation_line = if rotation_factor.0 > 0.2 {
                    // car turns on the left
                    sprite.flip_x = false;
                    2
                } else if rotation_factor.0 < -0.2 {
                    // car turns on the right
                    sprite.flip_x = true;
                    2
                } else {
                    // car doesn't turn
                    1
                };
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = compute_atlas_index(animation_line, indices.index, &indices.indices);
                    indices.index = atlas.index;
                }
            }
        }
    }
}

fn animate_die() {}

fn animate_brake(
    mut q_car: Query<
        (
            &State,
            &mut AnimationTimer,
            &mut AnimationIndices<CAR_NUM_ANIMATION>,
            &mut Sprite,
        ),
        With<Car>,
    >,
    time: Res<Time>,
) {
    for (state, mut timer, mut indices, mut sprite) in &mut q_car {
        if matches!(state, State::Braking) {
            timer.0.tick(time.delta());
            if timer.0.just_finished() {
                let animation_line = 3;
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = compute_atlas_index(animation_line, indices.index, &indices.indices);
                    indices.index = atlas.index;
                }
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

fn input_acceleration(
    acceleration: On<Fire<Accelerate>>,
    mut query: Query<(&mut Velocity, &mut State), With<Car>>,
) {
    let (mut velocity, mut state) = query.get_mut(acceleration.context).unwrap();
    velocity.0 = Vec2::splat(acceleration.value);
    *state = State::Accelerating;
}

fn input_rotation(
    rotation: On<Fire<Rotate>>,
    mut q_rotation: Query<&mut RotationFactor, With<Car>>,
) {
    let mut rotation_factor = q_rotation.get_mut(rotation.context).unwrap();
    rotation_factor.0 = -rotation.value;
}

fn input_brake(
    brake: On<Fire<Brake>>,
    mut query: Query<(&mut BrakeFactor, &mut State), With<Car>>,
) {
    let (mut brake_factor, mut state) = query.get_mut(brake.context).unwrap();
    brake_factor.0 = brake.value;
    *state = State::Braking;
}

fn input_cancel_acceleration(
    acceleration: On<Complete<Accelerate>>,
    mut query: Query<&mut State, With<Car>>,
) {
    let mut state = query.get_mut(acceleration.context).unwrap();
    *state = State::Neutral;
}

fn input_cancel_brake(brake: On<Complete<Brake>>, mut query: Query<&mut State, With<Car>>) {
    let mut state = query.get_mut(brake.context).unwrap();
    *state = State::Neutral;
}
