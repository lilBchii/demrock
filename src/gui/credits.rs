use gilrs::Gilrs;
use macroquad::{
    color::{BLACK, WHITE},
    miniquad::window::screen_size,
    text::{draw_text_ex, get_text_center, TextParams},
    window::{clear_background, next_frame},
};

use crate::gui::style::GuiResources;
use crate::{game::GameState, input::MenuInput};

use super::{
    button::{Button, Ui},
    TITLE_FONT_SIZE,
};

pub async fn credits(resources: &GuiResources, gilrs: &mut Gilrs) -> GameState {
    let mut ui = Ui::default();
    let mut input = MenuInput::new();

    loop {
        clear_background(BLACK);

        input.update(gilrs);

        let (sw, sh) = screen_size();

        let title = "Credits";
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

        ui.build(vec![Button::back_button()]);
        ui.update(input);
        ui.draw(resources);

        if ui.widgets[0].is_clicked() || input.back {
            return GameState::Menu;
        }

        next_frame().await;
    }
}
