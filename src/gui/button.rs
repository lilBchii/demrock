use gilrs::Button as GPButton;
use gilrs::{Axis, Event, EventType, Gilrs};
use macroquad::input::is_key_pressed;
use macroquad::{
    color::WHITE,
    input::KeyCode,
    math::{vec2, Rect},
    text::{draw_text_ex, get_text_center, TextParams},
    texture::{draw_texture_ex, DrawTextureParams},
    window::screen_height,
};

use crate::input::MenuInput;

use super::GuiResources;
use super::BUTTON_SIZE;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElementState {
    Normal,
    Selected,
    Clicked,
}

#[derive(Clone)]
pub struct Button {
    pub state: ElementState,
    pub name: String,
    pub dest: Rect,
}

impl Button {
    pub fn new(dest: Rect, name: String) -> Self {
        Self {
            state: ElementState::Normal,
            name,
            dest,
        }
    }

    pub fn draw(&self, resources: &GuiResources) {
        let source = match self.state {
            ElementState::Normal => Some(Rect::new(0.0, 0.0, BUTTON_SIZE.0, BUTTON_SIZE.1)),
            ElementState::Selected => {
                Some(Rect::new(0.0, BUTTON_SIZE.1, BUTTON_SIZE.0, BUTTON_SIZE.1))
            }
            ElementState::Clicked => Some(Rect::new(
                0.0,
                2.0 * BUTTON_SIZE.1,
                BUTTON_SIZE.0,
                BUTTON_SIZE.1,
            )),
        };
        draw_texture_ex(
            &resources.button_texture,
            self.dest.x,
            self.dest.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.dest.w, self.dest.h)),
                source,
                ..Default::default()
            },
        );
        let font_size = self.dest.h * 0.4;
        let text_center = get_text_center(
            &self.name,
            Some(&resources.font),
            font_size as u16,
            1.0,
            0.0,
        );
        draw_text_ex(
            &self.name,
            self.dest.x + self.dest.w * 0.5 - text_center.x,
            self.dest.y + self.dest.h * 0.5 - text_center.y,
            TextParams {
                font: Some(&resources.font),
                font_size: font_size as u16,
                color: WHITE,
                ..Default::default()
            },
        );
    }

    pub fn is_clicked(&self) -> bool {
        self.state == ElementState::Clicked
    }

    pub fn back_button() -> Self {
        Self {
            state: ElementState::Normal,
            name: "Back".into(),
            dest: Rect::new(10.0, screen_height() - 50.0, 180.0, 40.0),
        }
    }
}

#[derive(Default)]
pub struct Ui {
    pub cursor: usize,
    pub widgets: Vec<Button>,
}

impl From<Vec<Button>> for Ui {
    fn from(widgets: Vec<Button>) -> Self {
        Self {
            widgets,
            ..Default::default()
        }
    }
}

impl Ui {
    pub fn build(&mut self, buttons: Vec<Button>) {
        self.widgets = buttons;
    }

    pub fn update(&mut self, input: MenuInput) {
        // update cursor
        if input.up {
            self.cursor = if self.cursor == 0 {
                self.widgets.len() - 1
            } else {
                self.cursor - 1
            };
        } else if input.down {
            self.cursor = if self.cursor == self.widgets.len() - 1 {
                0
            } else {
                self.cursor + 1
            };
        }

        // update widgets
        self.widgets
            .iter_mut()
            .for_each(|button| button.state = ElementState::Normal);
        self.widgets[self.cursor].state = ElementState::Selected;

        if input.click {
            self.widgets[self.cursor].state = ElementState::Clicked;
        }
    }

    pub fn draw(&self, resources: &GuiResources) {
        self.widgets.iter().for_each(|w| {
            w.draw(resources);
        });
    }
}
