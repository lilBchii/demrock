use gilrs::{Axis, Button, Event, EventType, Gilrs};
use macroquad::input::{is_key_pressed, KeyCode};

#[derive(Clone, Copy, Debug)]
pub struct MenuInput {
    pub click: bool,
    pub down: bool,
    pub up: bool,
    pub back: bool,
    deadzone: f32,
    prev: f32,
}

impl MenuInput {
    pub fn new() -> Self {
        Self {
            click: false,
            down: false,
            up: false,
            back: false,
            deadzone: 0.35,
            prev: 0.0,
        }
    }

    pub fn update(&mut self, gilrs: &mut Gilrs) {
        self.up =
            is_key_pressed(KeyCode::Z) || is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W);
        self.down = is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down);

        self.click = is_key_pressed(KeyCode::Enter);
        self.back = is_key_pressed(KeyCode::Backspace) || is_key_pressed(KeyCode::Escape);

        while let Some(Event { event, .. }) = gilrs.next_event() {
            match event {
                EventType::ButtonPressed(Button::South, _) => {
                    self.click = true;
                }
                EventType::ButtonReleased(Button::South, _) => {
                    self.click = false;
                }
                EventType::ButtonPressed(Button::East, _) => {
                    self.back = true;
                }
                EventType::ButtonReleased(Button::East, _) => {
                    self.back = false;
                }
                EventType::AxisChanged(Axis::LeftStickY, value, _) => {
                    if (value > self.deadzone && value > self.prev)
                        || (value < self.deadzone && value < self.prev)
                    {
                        self.prev = value;
                    }
                }
                _ => {}
            }
            self.down = self.prev < -0.985;
            self.up = self.prev > 0.985;
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PlayerInput {
    pub accelerate: Option<f32>,
    pub turn: Option<f32>,
    pub brake: Option<f32>,
    pub boost: bool,
    pub deadzone: f32,
}

impl Default for PlayerInput {
    fn default() -> Self {
        Self {
            accelerate: None,
            turn: None,
            brake: None,
            boost: false,
            deadzone: 0.32,
        }
    }
}
