use bevy::prelude::States;
use bevy::state::state::SubStates;

#[derive(Debug, PartialEq, Eq, Clone, Hash, States)]
pub enum GameState {
    Init,
    Configuring,
    Playing,
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Hash, SubStates)]
#[source(GameState = GameState::Playing)]
#[states(scoped_entities)]
pub enum PlayingState {
    #[default]
    Countdown,
    Racing,
    End,
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Hash, States)]
pub enum Pause {
    #[default]
    Running,
    Paused,
}

impl Pause {
    pub fn get_toggled(&self) -> Self {
        match self {
            Self::Running => Self::Paused,
            Self::Paused => Self::Running,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum Menu {
    None,
    // Start menus
    Main,
    Settings,
    Credits,
    // Game configuration menus
    PlayerMenu,
    ModeSelection,
    LevelSelection,
    CarConfig,
    // In game menu
    Pause,
    // End menus
    RaceOver,
    GameOver,
}
