use bevy::app::{App, Plugin, Startup};
use bevy::ecs::component::Component;
use bevy::ecs::system::Commands;
use rand::seq::IndexedRandom;

use crate::tilemap::{Level, ALL_LEVELS};

pub const ARCADE_NUM_RACES: usize = 3;

pub struct GameModePlugin;

impl Plugin for GameModePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_mode);
    }
}

#[derive(Component)]
pub enum GameMode {
    Arcade,
    Free,
}

#[derive(Component)]
pub struct SelectedLevel(pub Level);

#[derive(Component)]
pub struct ArcadeLevels {
    pub levels: [Level; ARCADE_NUM_RACES],
    index: usize,
}

impl Default for ArcadeLevels {
    fn default() -> Self {
        ArcadeLevels {
            levels: [Level::Playground, Level::Demcity, Level::Galabusa],
            index: 0,
        }
    }
}

impl ArcadeLevels {
    pub fn init() -> Self {
        let levels: Option<[Level; ARCADE_NUM_RACES]> = ALL_LEVELS.sample_array(&mut rand::rng());
        match levels {
            Some(levels) => {
                levels.iter().for_each(|l| println!("{}", l.name()));
                ArcadeLevels { levels, index: 0 }
            }
            None => ArcadeLevels::default(),
        }
    }

    // goes up to ARCADAE_NUM_RACES such that we can know when the serie
    // is finished
    #[must_use]
    pub fn increment_index(&mut self) -> bool {
        let index = self.index + 1;
        if index > ARCADE_NUM_RACES {
            false
        } else {
            self.index = index;
            true
        }
    }

    pub fn is_finished(&self) -> bool {
        self.index >= ARCADE_NUM_RACES
    }

    pub fn get_current_level(&self) -> Level {
        self.levels[self.index].clone()
    }
}

// TODO: modify with component resource with bevy 19
// and spawn when needed
pub fn spawn_game_mode(mut commands: Commands) {
    commands.spawn((
        GameMode::Arcade,
        SelectedLevel(Level::Playground),
        ArcadeLevels::default(),
    ));
}
