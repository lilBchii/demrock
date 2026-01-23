use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::{
    animation::{compute_atlas_index, custom_layout, AnimationIndices, AnimationTimer},
    common::{
        AppState, CAR_ACCELERATION, CAR_ANIMATION_INDICES, CAR_BRAKE, CAR_NUM_ANIMATION,
        CAR_ROTATION, CAR_SPRITE_SIZE,
    },
    tilemap::{Road, StartingLine},
};

// ---- Plugin ---- //
pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CarSpawnPoint>();
        app.add_systems(
            Update,
            (deccelerate, animate_car, detect_out).run_if(in_state(AppState::Playing)),
        )
        .add_input_context::<Car>()
        .add_observer(spawn_car)
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
enum State {
    Accelerating,
    Falling,
    Braking,
    Neutral,
}

#[derive(Component)]
struct IsGrounded(pub bool);

#[derive(Bundle)]
struct CarBundle {
    marker: Car,
    config: Config,
    // inputs
    rotation_factor: RotationFactor,
    state: State,
    // physics
    transform: Transform,
    collider: Collider,
    body: RigidBody,
    is_grounded: IsGrounded,
    // appearence
    sprite: Sprite,
    animation_indices: AnimationIndices<CAR_NUM_ANIMATION>,
    animation_timer: AnimationTimer,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[require(Transform)]
#[reflect(Component)]
struct CarSpawnPoint;

// ---- Sytems ---- //

fn spawn_car(
    add_car_spawn: On<Add, CarSpawnPoint>,
    mut commands: Commands,
    car_spawn_query: Query<&Transform, With<CarSpawnPoint>>,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let tile_size = UVec2::from(CAR_SPRITE_SIZE);
    let atlas_layout_handle = atlas_layouts.add(custom_layout::<CAR_NUM_ANIMATION>(
        tile_size,
        CAR_ANIMATION_INDICES,
    ));

    let spawn_pos = *car_spawn_query.get(add_car_spawn.event().entity).unwrap();

    commands.spawn((
        CarBundle {
            marker: Car,
            config: Config {
                rotation_speed: CAR_ROTATION,
                acceleration: CAR_ACCELERATION,
                brake: CAR_BRAKE,
            },
            rotation_factor: RotationFactor(0.0),
            state: State::Neutral,
            transform: Transform::from_translation(Vec3::new(
                spawn_pos.translation.x,
                spawn_pos.translation.y,
                1.0,
            )),
            collider: Collider::rectangle(2.0, 5.0),
            body: RigidBody::Kinematic,
            is_grounded: IsGrounded(true),
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
        TransformInterpolation,
        Sensor,
        CollisionEventsEnabled,
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

fn detect_out(
    spatial_query: SpatialQuery,
    mut car_query: Query<(&Collider, &Transform, &mut IsGrounded, &mut State), With<Car>>,
    road_query: Query<&Road, Without<StartingLine>>,
) {
    for (collider, transform, mut is_grounded, mut state) in car_query.iter_mut() {
        let intersections = spatial_query.shape_intersections(
            collider,
            Vec2::new(transform.translation.x, transform.translation.y),
            transform.rotation.to_axis_angle().1,
            &SpatialQueryFilter::default(),
        );
        for entity in intersections.iter() {
            // intersects with road component
            if road_query.contains(*entity) {
                if is_grounded.0 {
                    is_grounded.0 = false;
                    *state = State::Falling;
                    break;
                }
            }
        }
    }
}

fn deccelerate(
    mut query: Query<(&IsGrounded, &mut LinearVelocity, &mut AngularVelocity), With<Car>>,
) {
    for (is_grounded, mut velocity, mut rotation) in query.iter_mut() {
        if is_grounded.0 {
            rotation.0 *= 0.95;
            velocity.0 *= 0.98;
        }
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
                State::Falling => 4,
            };
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = compute_atlas_index(animation_line, indices.index, &indices.indices);
                indices.index = atlas.index;
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
        (
            &mut LinearVelocity,
            &Transform,
            &IsGrounded,
            &mut State,
            &Config,
        ),
        With<Car>,
    >,
    time: Res<Time>,
) {
    let (mut velocity, transform, is_grounded, mut state, config) =
        query.get_mut(acceleration.context).unwrap();
    if is_grounded.0 {
        let dir = (transform.rotation * Vec3::Y).truncate();
        velocity.0 +=
            Vec2::splat(acceleration.value * config.acceleration * time.delta_secs()) * dir;
        println!("{}", velocity.length());
        *state = State::Accelerating;
    }
}

fn rotate(
    rotation: On<Fire<Rotate>>,
    mut q_rotation: Query<
        (
            &mut RotationFactor,
            &mut AngularVelocity,
            &IsGrounded,
            &Config,
        ),
        With<Car>,
    >,
    time: Res<Time>,
) {
    let (mut rotation_factor, mut angular_velocity, is_grounded, config) =
        q_rotation.get_mut(rotation.context).unwrap();
    if is_grounded.0 {
        angular_velocity.0 -= rotation.value * config.rotation_speed * time.delta_secs();
        rotation_factor.0 = -rotation.value;
    }
}

fn input_brake(
    brake: On<Fire<Brake>>,
    mut query: Query<(&mut LinearVelocity, &IsGrounded, &mut State, &Config), With<Car>>,
) {
    let (mut velocity, is_grounded, mut state, config) = query.get_mut(brake.context).unwrap();
    if is_grounded.0 {
        velocity.0 *= 1.0 - brake.value * config.brake;
        *state = State::Braking;
    }
}

fn input_cancel_acceleration(
    acceleration: On<Complete<Accelerate>>,
    mut query: Query<(&IsGrounded, &mut State), With<Car>>,
) {
    let (is_grounded, mut state) = query.get_mut(acceleration.context).unwrap();
    if is_grounded.0 {
        *state = State::Neutral;
    }
}

fn input_cancel_brake(
    brake: On<Complete<Brake>>,
    mut query: Query<(&IsGrounded, &mut State), With<Car>>,
) {
    let (is_grounded, mut state) = query.get_mut(brake.context).unwrap();
    if is_grounded.0 {
        *state = State::Neutral;
    }
}
