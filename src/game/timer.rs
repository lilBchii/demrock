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
