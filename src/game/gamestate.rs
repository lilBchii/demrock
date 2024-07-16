#[derive(Debug)]
pub enum GameMode {
    Arcade,
    Perso,
}

#[derive(Debug)]
pub enum GameState {
    Menu,
    Playing(GameMode),
    SelectLevel,
    GameOver,
    Credits,
    Options,
    Quit,
}

impl Default for GameState {
    fn default() -> Self {
        Self::Menu
    }
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_game_mode(gm: GameMode) -> Self {
        GameState::Playing(gm)
    }
}
