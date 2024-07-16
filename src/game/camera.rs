use macroquad::{
    camera::{set_camera, Camera2D},
    math::vec2,
    miniquad::window::screen_size,
    text::draw_text,
};

use super::{Player, MAP_SIZE, TILE_SIZE, ZOOM};

pub const SCREEN_RATIO: f32 = 1920.0 / 1080.0;

pub type Viewport = (i32, i32, i32, i32);

/*#[derive(Debug)]
pub struct Camera {
    pub pos: Vec2,
    pub zoom: f32,
    pub dest: Rect,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: vec2(0.0, 0.0),
            zoom: 2.0,
            dest: Rect {
                x: 0.0,
                y: 0.0,
                w: 0.0,
                h: 0.0,
            },
        }
    }

    pub fn update(&mut self, player: &Player) {
        let (sw, sh) = screen_size();
        let full_screen_ratio = sw / sh;
        if full_screen_ratio >= SCREEN_RATIO {
            let asw = SCREEN_RATIO * sh;
            self.dest = Rect {
                x: (sw - asw) / 2.0,
                y: 0.0,
                w: asw,
                h: sh,
            }
        } else {
            let ash = sw / SCREEN_RATIO;
            self.dest = Rect {
                x: 0.0,
                y: (sh - ash) / 2.0,
                w: sw,
                h: ash,
            }
        }
        // player is center on the screen
        self.pos.x = player.position.x - self.dest.w / 2.0;
        self.pos.y = player.position.y - self.dest.h / 2.0;
    }
}*/

/*pub fn viewport_rect() -> Rect {
    let (sw, sh) = screen_size();
    let full_screen_ratio = sw / sh;
    if full_screen_ratio >= SCREEN_RATIO {
        let asw = SCREEN_RATIO * sh;
        Rect {
            x: (sw - asw) / 2.0,
            y: 0.0,
            w: asw,
            h: sh,
        }
    } else {
        let ash = sw / SCREEN_RATIO;
        Rect {
            x: 0.0,
            y: (sh - ash) / 2.0,
            w: sw,
            h: ash,
        }
    }
}*/

pub fn update_viewport() -> Viewport {
    let (sw, sh) = screen_size();
    let full_screen_ratio = sw / sh;
    if full_screen_ratio >= SCREEN_RATIO {
        let asw = SCREEN_RATIO * sh;
        ((sw - asw) as i32 / 2, 0, asw as i32, sh as i32)
    } else {
        let ash = sw / SCREEN_RATIO;
        (0, (sh - ash) as i32 / 2, sw as i32, ash as i32)
    }
}

pub fn set_player_cam(zoom: f32, player: &Player, viewport: Viewport) {
    set_camera(&Camera2D {
        zoom: vec2(
            1.0 / TILE_SIZE * zoom,
            1.0 / TILE_SIZE * SCREEN_RATIO * zoom,
        ),
        target: vec2(
            player.position.x.clamp(
                TILE_SIZE / zoom,
                TILE_SIZE * MAP_SIZE.0 - (ZOOM * TILE_SIZE / zoom),
            ),
            player.position.y.clamp(
                TILE_SIZE / SCREEN_RATIO / zoom,
                TILE_SIZE * MAP_SIZE.1 - ((ZOOM / SCREEN_RATIO * TILE_SIZE) / zoom * SCREEN_RATIO),
            ),
        ),
        viewport: Some(viewport),
        ..Default::default()
    });
}

pub fn set_background_cam(player: &Player, viewport: Viewport) {
    let mut cam = Camera2D {
        zoom: vec2(
            1.0 / TILE_SIZE * ZOOM * 2.0,
            1.0 / TILE_SIZE * SCREEN_RATIO * ZOOM * 2.0,
        ),
        viewport: Some(viewport),
        ..Default::default()
    };
    // to clamp to screen we need the cam to go from 120 to 360 so 240px and 240/500 = 0.48
    // for y axis it's from 67.5 (120/SCREEN_RATIO) to 202.5 so 135px and 135/500 = 0.27
    cam.target = vec2(
        player.position.x / TILE_SIZE * 0.48 + 120.0,
        player.position.y / TILE_SIZE * 0.27 + 67.5,
    );
    set_camera(&cam);
}

/*pub fn set_player_cam(zoom: f32, player: &Player) {
    let vp = viewport_rect();
    let init_zoom = 65.0;
    let mut cam = Camera2D::from_display_rect(vp);
    cam.zoom = vec2(cam.zoom.x * zoom * SCREEN_RATIO, -cam.zoom.y * zoom);
    cam.target = vec2(
        player.position.x.clamp(
            cam.zoom.x / zoom + vp.w * 0.5,
            TILE_SIZE * MAP_SIZE.0 - (init_zoom * TILE_SIZE / zoom),
        ),
        player.position.y.clamp(
            cam.zoom.y / zoom + vp.h * 0.5,
            TILE_SIZE * MAP_SIZE.1 - ((init_zoom / SCREEN_RATIO * TILE_SIZE) / zoom),
        ),
    );
    set_camera(&cam);
    unsafe {
        window::get_internal_gl().quad_gl.scissor(Some((
            vp.x as i32,
            vp.y as i32,
            vp.w as i32,
            vp.h as i32,
        )));
    }
}*/
