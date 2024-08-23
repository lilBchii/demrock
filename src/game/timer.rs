use macroquad::{
    color::WHITE,
    text::{draw_text_ex, Font, TextParams},
    texture::Texture2D,
    time::get_frame_time,
    window::{clear_background, screen_height, screen_width},
};

#[derive(Debug, Clone)]
pub struct Timer {
    elapsed: f64,
    wait: f64,
}

impl Timer {
    pub fn new(wait: f64) -> Self {
        Self { elapsed: 0.0, wait }
    }

    pub fn update(&mut self, elapsed_time: f64) -> bool {
        self.elapsed += elapsed_time;
        self.is_done()
    }

    pub fn is_done(&self) -> bool {
        self.wait <= self.elapsed
    }

    pub fn reset(&mut self) {
        while self.is_done() {
            self.elapsed -= self.wait;
        }
    }

    pub fn elapsed(&self) -> f64 {
        self.elapsed
    }

    pub fn time_left(&self) -> f64 {
        self.wait - self.elapsed
    }
}

pub struct Countdown {
    count: f64,
    start: f64,
}

impl Countdown {
    pub fn new(start: f64) -> Self {
        Self { count: 0.0, start }
    }

    pub fn finished(&self) -> bool {
        self.count >= self.start
    }

    pub fn update(&mut self, delta: f64) {
        if !self.finished() {
            self.count += delta;
        }
    }

    pub fn time_left(&self) -> f64 {
        (self.start - self.count).max(0.0)
    }

    pub fn draw(&self, font: &Font) {
        if !self.finished() {
            draw_text_ex(
                format!("{}", self.time_left() as usize).as_str(),
                screen_width() * 0.5 - 35.0,
                screen_height() * 0.5 - 100.0,
                TextParams {
                    font: Some(font),
                    font_size: 75,
                    color: WHITE,
                    ..Default::default()
                },
            );
        }
    }
}
