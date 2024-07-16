use macroquad::{
    color::Color,
    miniquad::FilterMode,
    text::{load_ttf_font, load_ttf_font_from_bytes, Font, TextParams},
    texture::{load_texture, Image, Texture2D},
    ui::{root_ui, Skin},
};

/*pub struct GuiResources {
    pub main_skin: Skin,
    pub menu_skin: Skin,
}

impl GuiResources {
    pub fn new() -> Self {
        let main_skin = {
            let label_style = root_ui()
                .style_builder()
                .font(include_bytes!("../../assets/ui/PressStart2P.ttf"))
                .expect("font file")
                .text_color(Color::from_rgba(255, 255, 255, 255))
                .font_size(130)
                .build();

            let button_style = root_ui()
                .style_builder()
                .background(
                    Image::from_file_with_format(include_bytes!("../../assets/ui/btn1.png"), None)
                        .expect("assets load error"),
                )
                .background_hovered(
                    Image::from_file_with_format(
                        include_bytes!("../../assets/ui/btn1_hovered.png"),
                        None,
                    )
                    .expect("assets load error"),
                )
                .background_clicked(
                    Image::from_file_with_format(
                        include_bytes!("../../assets/ui/btn1_clicked.png"),
                        None,
                    )
                    .expect("assets load error"),
                )
                .font(include_bytes!("../../assets/ui/PressStart2P.ttf"))
                .unwrap()
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size(25)
                .build();

            Skin {
                label_style,
                button_style,
                ..root_ui().default_skin()
            }
        };

        let menu_skin = {
            let label_style = root_ui()
                .style_builder()
                .font(include_bytes!("../../assets/ui/PressStart2P.ttf"))
                .unwrap()
                .text_color(Color::from_rgba(255, 255, 255, 255))
                .font_size(50)
                .build();

            let button_style = root_ui()
                .style_builder()
                .background(
                    Image::from_file_with_format(include_bytes!("../../assets/ui/btn1.png"), None)
                        .expect("assets load error"),
                )
                .background_hovered(
                    Image::from_file_with_format(
                        include_bytes!("../../assets/ui/btn1_hovered.png"),
                        None,
                    )
                    .expect("assets load error"),
                )
                .background_clicked(
                    Image::from_file_with_format(
                        include_bytes!("../../assets/ui/btn1_clicked.png"),
                        None,
                    )
                    .expect("assets load error"),
                )
                .font(include_bytes!("../../assets/ui/PressStart2P.ttf"))
                .unwrap()
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size(15)
                .build();

            Skin {
                label_style,
                button_style,
                ..root_ui().default_skin()
            }
        };

        Self {
            main_skin,
            menu_skin,
        }
    }
}*/

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
