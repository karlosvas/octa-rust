use std::time::Instant;

// Mensaje principal que engloba todos
#[derive(Debug, Clone)]
pub enum AppMessage {
    MainMenu(MainMenuMessage),
    Game(GameMessage),
    Settings(SettingsMessage),
}

#[derive(Debug, Clone, Copy)]
pub enum AppState {
    MainMenu,
    Game,
    Settings,
}

// Enumeración que define los mensajes(acciones) que la aplicación puede recibir
#[derive(Debug, Clone, Copy)]
pub enum MainMenuMessage {
    Play,
    Exit,
    OpenSettings,
}

#[derive(Debug, Clone)]
pub enum GameMessage {
    // PauseGame,
    // RestartGame,
    // PlayNote(u8),
    Tick(Instant),
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    // ChangeTheme,
    BackToMenu,
}
