use crate::models::settings::{CustomTheme, Difficulty};
use std::time::Instant;

// Mensaje principal que engloba todos
#[derive(Debug, Clone)]
pub enum AppMessage {
    MainMenu(MainMenuMessage),
    Game(GameMessage),
    Settings(SettingsMessage),
    Selection(SelectionMessage),
}

// Estados de la aplicación
#[derive(Debug, Clone, Copy)]
pub enum AppState {
    MainMenu,
    SlectionPartiture,
    Game,
    Settings,
    Paused,
}

// Mensajes específicos para la selección de partituras
#[derive(Debug, Clone)]
pub enum SelectionMessage {
    StartGame(String),
    BackToMenu,
}

// Enumeración que define los mensajes(acciones) que la aplicación puede recibir
#[derive(Debug, Clone, Copy)]
pub enum MainMenuMessage {
    SelectPartiture,
    Exit,
    OpenSettings,
}

#[derive(Debug, Clone)]
pub enum GameMessage {
    PauseGame,
    RestartGame,
    ResumeGame,
    Finished,
    Tick(Instant),
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    ChangeDifficulty(Difficulty),
    ChangeTheme(CustomTheme),
    BackToMenu,
}
