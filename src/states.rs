use bevy::prelude::{StateSet, States};
use bevy::state::state::SubStates;

#[derive(Debug, PartialEq, Eq, Clone, Hash, States)]
pub enum GameState {
    Menu,
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

#[derive(Debug, PartialEq, Eq, Clone, Hash, States)]
pub struct Pause(pub bool);

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
