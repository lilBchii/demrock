use macroquad::{
    miniquad::FilterMode,
    text::{load_ttf_font, Font},
    texture::{load_texture, Texture2D},
};

pub struct GuiResources {
    pub button_texture: Texture2D,
    pub font: Font,
}

impl GuiResources {
    pub async fn new() -> GuiResources {
        let button_texture = load_texture("assets/ui/btn_pix.png")
            .await
            .expect("failed to load buttons texture");
        button_texture.set_filter(FilterMode::Nearest);
        let font = load_ttf_font("assets/ui/PressStart2P.ttf")
            .await
            .expect("failed to load font");
        GuiResources {
            button_texture,
            font,
        }
    }
}
