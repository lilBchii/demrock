use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub carstat: CarStat,
}

#[derive(Deserialize, Clone, Copy)]
pub struct CarStat {
    pub max_velocity: f32,
    pub rotation_speed: f32,
    pub acceleration: f32,
    pub brake: f32,
}
