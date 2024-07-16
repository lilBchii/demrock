use gilrs::Gilrs;
use macroquad::{
    color::BLACK,
    math::Rect,
    miniquad::window::screen_size,
    window::{clear_background, next_frame},
};

use crate::game::{GameMode, GameState};
use crate::gui::style::GuiResources;

use super::button::{Button, Ui, UserInput};

pub async fn main_menu(resources: &GuiResources, gilrs: &mut Gilrs) -> GameState {
    let mut ui = Ui::default();
    let prev = 0.0;
    let mut input = UserInput::new();

    loop {
        clear_background(BLACK);

        let (sw, sh) = screen_size();

        input.update(gilrs, prev);

        //let button_gap = sh / 32.0;
        let button_h = (3.0 * sh) / 32.0; // button is 3/4 of 1/2 screen and there are 4 buttons so sh/2/4 * 3/4
        let button_w = 6.0 * button_h;
        let button_align_x = (sw - button_w) / 2.0;

        ui.build(vec![
            Button::new(
                Rect::new(button_align_x, sh / 2.0, button_w, button_h),
                "Play".into(),
            ),
            Button::new(
                Rect::new(button_align_x, sh / 2.0 + (sh / 8.0), button_w, button_h),
                "Options".into(),
            ),
            Button::new(
                Rect::new(
                    button_align_x,
                    sh / 2.0 + (2.0 * (sh / 8.0)),
                    button_w,
                    button_h,
                ),
                "Credits".into(),
            ),
            Button::new(
                Rect::new(
                    button_align_x,
                    sh / 2.0 + (3.0 * (sh / 8.0)),
                    button_w,
                    button_h,
                ),
                "Quit".into(),
            ),
        ]);
        ui.update(input);
        ui.draw(resources);

        if ui.widgets[0].is_clicked() {
            return GameState::Playing(GameMode::Arcade);
        }

        if ui.widgets[1].is_clicked() {
            return GameState::Options;
        }

        if ui.widgets[2].is_clicked() {
            return GameState::Credits;
        }

        if ui.widgets[3].is_clicked() {
            return GameState::Quit;
        }

        next_frame().await;
    }
    //GameState::Menu
}
