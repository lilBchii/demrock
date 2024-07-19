use std::f32::consts::LN_2;

use gilrs::*;
use macroquad::experimental::animation::*;
use macroquad::prelude::*;

use crate::game::ZOOM;

use crate::config::CarStat;

use super::TILE_SIZE;

pub const SPRITE_SIZE: (f32, f32) = (32.0, 56.0);

#[derive(Debug)]
pub struct Input {
    accelerate: Option<f32>,
    turn: Option<f32>,
    brake: Option<f32>,
    boost: bool,
    deadzone: f32,
}

pub struct Player {
    pub sprite: AnimatedSprite,
    pub texture: Texture2D,

    pub position: Vec2,
    pub rotation: f32,
    pub velocity: f32,

    stat: CarStat,

    input: Input,
}

impl Player {
    pub async fn new(stat: &CarStat) -> Self {
        let texture = load_texture("assets/rb_ship.png")
            .await
            .expect("car sprite file");
        texture.set_filter(FilterMode::Nearest);

        Self {
            sprite: AnimatedSprite::new(
                SPRITE_SIZE.0 as u32,
                SPRITE_SIZE.1 as u32,
                &[
                    Animation {
                        name: "stop".to_string(),
                        row: 0,
                        frames: 1,
                        fps: 1,
                    },
                    Animation {
                        name: "rainbow".to_string(),
                        row: 1,
                        frames: 6,
                        fps: 6,
                    },
                ],
                true,
            ),
            texture,
            position: Vec2::new(0.0, 0.0),
            rotation: 0.0,
            velocity: 0.0,
            stat: *stat,
            input: Input {
                accelerate: None,
                turn: None,
                brake: None,
                boost: false,
                deadzone: 0.1,
            },
        }
    }

    pub fn update(&mut self, gilrs: &mut Gilrs) {
        let delta_time = get_frame_time();

        // gamepad controls
        while let Some(Event { event, .. }) = gilrs.next_event() {
            match event {
                EventType::ButtonChanged(Button::RightTrigger2, value, _) => {
                    if value > self.input.deadzone || value < -self.input.deadzone {
                        self.input.accelerate = Some(value);
                    } else {
                        self.input.accelerate = None;
                    }
                }

                EventType::ButtonChanged(Button::LeftTrigger2, value, _) => {
                    if value > self.input.deadzone || value < -self.input.deadzone {
                        self.input.brake = Some(value);
                    } else {
                        self.input.brake = None;
                    }
                }
                EventType::AxisChanged(Axis::LeftStickX, value, _) => {
                    if value > self.input.deadzone || value < -self.input.deadzone {
                        self.input.turn = Some(value);
                    } else {
                        self.input.turn = None;
                    }
                }
                _ => {}
            }
        }

        if self.input.accelerate.is_some() {
            self.velocity += self.stat.acceleration * self.input.accelerate.unwrap() * delta_time;
            self.sprite.set_animation(1);
        }
        if self.input.brake.is_some() {
            self.velocity -= self.input.brake.unwrap() * self.stat.brake * delta_time;
            self.sprite.set_animation(0);
        }
        if self.input.turn.is_some() {
            self.rotation += self.input.turn.unwrap() * self.stat.rotation_speed * delta_time;
        }

        // keyboard controls
        if is_key_down(KeyCode::Right) | is_key_down(KeyCode::D) {
            self.rotation += self.stat.rotation_speed * delta_time;
        }
        if is_key_down(KeyCode::Left) | is_key_down(KeyCode::Q) | is_key_down(KeyCode::A) {
            self.rotation -= self.stat.rotation_speed * delta_time;
        }
        if is_key_down(KeyCode::Down) | is_key_down(KeyCode::S) {
            self.velocity -= self.stat.brake * delta_time;
            self.sprite.set_animation(0);
        }
        if is_key_down(KeyCode::Up) | is_key_down(KeyCode::Z) | is_key_down(KeyCode::W) {
            self.velocity += self.stat.acceleration * delta_time;
            self.sprite.set_animation(1);
        }

        // Avoid velocity to get higher than max speed
        self.velocity = self.velocity.clamp(0.0, self.stat.max_velocity);
        // Move car
        self.position.x += self.rotation.sin() * self.velocity;
        self.position.y += -self.rotation.cos() * self.velocity;
        // Decelerate car
        self.velocity *= 0.98;
    }

    pub fn draw(&mut self) {
        draw_texture_ex(
            &self.texture,
            self.position.x - SPRITE_SIZE.0 * 0.5,
            self.position.y - SPRITE_SIZE.1 * 0.5,
            WHITE,
            DrawTextureParams {
                source: Some(self.sprite.frame().source_rect),
                dest_size: Some(vec2(SPRITE_SIZE.0, SPRITE_SIZE.1)),
                rotation: self.rotation,
                ..Default::default()
            },
        )
    }

    // allow to dezoom when the car is fast
    pub fn zoom_speed(&self) -> f32 {
        //ZOOM * (1.0 / self.velocity * 6.0).clamp(0.75, 1.0)
        //ZOOM * (1.0 - (0.25 * self.velocity) / self.stat.max_velocity)
        ZOOM * ((-LN_2 / (self.stat.max_velocity * self.stat.max_velocity))
            * self.velocity
            * self.velocity)
            .exp()
    }

    pub fn init(&mut self, pos: [usize; 2]) {
        self.position = vec2(
            pos[0] as f32 * TILE_SIZE + TILE_SIZE * 0.5,
            pos[1] as f32 * TILE_SIZE + TILE_SIZE * 0.5,
        );
        self.rotation = 0.0;
        self.velocity = 0.0;
    }
}
