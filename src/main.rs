use game::{
    play_music, set_background_cam, set_player_cam, update_viewport, GameMode, GameState, Level,
    Levels, MusicParams, Player, TILE_SIZE,
};
use gilrs::*;
use macroquad::audio::{load_sound, set_sound_volume, stop_sound};
use macroquad::prelude::*;
use macroquad::window;

use std::error::Error;

mod config;
mod game;
mod gui;

use config::Config;

/*struct Resources {
    car: Texture2D,
}

impl Resources {
    async fn new() -> Result<Resources, macroquad::Error> {
        let car = load_texture("assets/sprite.png")
            .await
            .expect("car sprite file");
        car.set_filter(FilterMode::Nearest);
        Ok(Self { car })
    }
}*/

async fn play_level(
    player: &mut Player,
    level: &Level,
    start_time: f64,
    gilrs: &mut Gilrs,
    font: &Font,
) {
    player.init(level.starting_position);

    loop {
        let time = get_time() - start_time;

        clear_background(BLACK);

        let viewport = update_viewport();

        let zoom = player.zoom_speed();

        set_background_cam(player, viewport);

        level.draw_background();

        // clear viewport
        unsafe {
            window::get_internal_gl().quad_gl.viewport(None);
        }

        set_player_cam(zoom, player, viewport);

        level.draw_circuit();
        player.update(gilrs);
        player.sprite.update();
        player.draw();

        // clear viewport
        unsafe {
            window::get_internal_gl().quad_gl.viewport(None);
        }

        set_default_camera();

        draw_text_ex(
            format!("{:.2}s", time).as_str(),
            10.0,
            20.0,
            TextParams {
                font: Some(font),
                font_size: 20,
                color: WHITE,
                ..Default::default()
            },
        );
        draw_text(
            format!("FPS: {}", get_fps()).as_str(),
            10.0,
            40.0,
            20.0,
            WHITE,
        );
        draw_text(
            format!("speed: {:.3}", player.velocity).as_str(),
            10.,
            60.,
            20.,
            WHITE,
        );

        draw_text(
            format!("player: {:2} {:2}", player.position.x, player.position.y).as_str(),
            10.0,
            80.0,
            20.0,
            WHITE,
        );

        draw_text(
            format!("animation: {:?}", player.sprite.frame().source_rect).as_str(),
            10.0,
            100.0,
            20.0,
            WHITE,
        );

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        /*if is_key_pressed(KeyCode::Enter) {
            println!("{:?}", cam);
            println!(
                "screen w: {}, screen h: {}",
                screen_width(),
                screen_height()
            );
        }*/

        //player.sprite.update();

        next_frame().await;
    }
}

#[macroquad::main("BigRace")]
async fn main() -> Result<(), Box<dyn Error>> {
    // detect controller
    let mut gilrs = Gilrs::new().unwrap();

    //set_pc_assets_folder("assets");

    // load gui resources
    let gui_resources = gui::GuiResources::new().await;

    // load config file
    let config_str =
        std::fs::read_to_string("assets/config.toml").expect("failed to read config.toml file");
    let config: Config = toml::from_str(&config_str).expect("invalid config");

    // set the game state to begin at menu
    let mut game_state = GameState::Menu;

    // load maintheme sound
    let main_theme_sound = load_sound("assets/maintheme.wav")
        .await
        .expect("failed to load main_theme sound");
    let mut main_theme = MusicParams {
        sound: main_theme_sound,
        is_playing: false,
        is_activated: true,
        volume: 1.0,
    };

    loop {
        match game_state {
            GameState::Menu => {
                set_sound_volume(&main_theme.sound, main_theme.volume);
                play_music(&mut main_theme);
                game_state = gui::main_menu(&gui_resources, &mut gilrs).await;
            }
            GameState::Playing(GameMode::Arcade) => {
                stop_sound(&main_theme.sound);
                main_theme.is_playing = false;
                // load resources
                println!("[...] loading");

                let mut player = Player::new(&config.carstat).await;
                player.sprite.set_animation(0);

                let levels_str =
                    std::fs::read_to_string("assets/levels/levels.toml").expect("levels.toml file");
                let levels_config: Levels =
                    toml::from_str(&levels_str).expect("invalid config level");

                let mut current_level_index: usize = 1;

                let mut levels = Vec::<Level>::new();

                for level in levels_config.levels.clone().into_iter() {
                    let loaded = Level::load(&level).await;
                    levels.push(loaded);
                }

                println!("[OK] loaded.");

                let start_time = get_time();

                play_level(
                    &mut player,
                    &levels[current_level_index],
                    start_time,
                    &mut gilrs,
                    &gui_resources.font,
                )
                .await;

                game_state = GameState::Menu;
            }
            GameState::Credits => {
                game_state = gui::credits(&gui_resources, &mut gilrs).await;
            }
            GameState::Options => {
                play_music(&mut main_theme);
                game_state = gui::options(&gui_resources, &mut gilrs, &mut main_theme).await;
            }
            GameState::Quit => {
                break Ok(());
            }
            _ => {
                clear_background(BLACK);

                let font_size: u16 = 30;
                let center = get_text_center(
                    "Something went wrong",
                    Some(&gui_resources.font),
                    font_size,
                    1.0,
                    0.0,
                );

                let (x, y) = (
                    screen_width() * 0.5 - center.x,
                    screen_height() * 0.5 - center.y,
                );

                draw_text_ex(
                    "Something went wrong",
                    x,
                    y,
                    TextParams {
                        font: Some(&gui_resources.font),
                        font_size,
                        color: WHITE,
                        ..Default::default()
                    },
                );

                draw_text_ex(
                    "Try escape or force quit",
                    x,
                    y + font_size as f32 + 20.0,
                    TextParams {
                        font: Some(&gui_resources.font),
                        font_size,
                        color: WHITE,
                        ..Default::default()
                    },
                );

                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Menu;
                }
            }
        }
        next_frame().await;
    }
}
