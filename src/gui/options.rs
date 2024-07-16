use gilrs::Gilrs;
use macroquad::{
    color::{BLACK, WHITE},
    math::Rect,
    miniquad::window::screen_size,
    text::{draw_text_ex, get_text_center, TextParams},
    window::{clear_background, next_frame},
};

use crate::game::{GameState, MusicParams};
use crate::gui::style::GuiResources;

use super::{
    button::{Button, Ui, UserInput},
    TITLE_FONT_SIZE,
};

pub async fn options(
    resources: &GuiResources,
    gilrs: &mut Gilrs,
    music_params: &mut MusicParams,
) -> GameState {
    let mut ui = Ui::default();
    let mut input = UserInput::new();
    let prev = 0.0;

    loop {
        clear_background(BLACK);

        input.update(gilrs, prev);

        let (sw, sh) = screen_size();

        // draw title
        let title = "Options";
        let text_size = get_text_center(title, Some(&resources.font), TITLE_FONT_SIZE, 1.0, 0.0);

        draw_text_ex(
            title,
            sw * 0.5 - text_size.x,
            sh * 0.2 + text_size.y,
            TextParams {
                font: Some(&resources.font),
                font_size: TITLE_FONT_SIZE,
                color: WHITE,
                ..Default::default()
            },
        );

        // buttons
        let button_h = 3.0 * sh / 40.0;
        let button_w = 6.0 * button_h;
        let button_align_x = (sw - button_w) * 0.5;
        let button_align_y = sh * 0.25;

        ui.build(vec![
            Button::new(
                Rect::new(button_align_x, button_align_y, button_w, button_h),
                if music_params.is_activated {
                    "turn music off".into()
                } else {
                    "turn music on".into()
                },
            ),
            Button::back_button(),
        ]);
        ui.update(input);
        ui.draw(resources);

        if ui.widgets[0].is_clicked() {
            music_params.is_activated ^= true;
            return GameState::Options;
        }

        // Back
        if ui.widgets[1].is_clicked() || input.back {
            return GameState::Menu;
        }

        next_frame().await;
    }
}
