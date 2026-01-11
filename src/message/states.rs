use {iced::Event, std::time::Instant};

/// Mensajes principales de la App
#[derive(Debug, Clone)]
pub enum AppMessage {
    MainMenu(MainMenuMessage),   // Menú
    Game(GameMessage),           // Juego
    Settings(SettingsMessage),   // Ajustes
    Selection(SelectionMessage), // Selecion de mensajes
    Event(Event),                // Eventos
}

/// Estados principales de la App
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppState {
    MainMenu,
    Game,
    Settings,
    SelectionPartiture,
    Paused,
}

// Mensajes específicos para la selección de partituras
#[derive(Debug, Clone)]
pub enum SelectionMessage {
    StartGame(&'static str),
    BackToMenu,
}

// Enumeración que define los mensajes(acciones) que la aplicación puede recibir
#[derive(Debug, Clone, Copy)]
pub enum MainMenuMessage {
    SelectPartiture,
    OpenSettings,
    Exit,
}

#[derive(Debug, Clone)]
pub enum GameMessage {
    Tick(Instant),
    ResumeGame,
    RestartGame,
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    ChangeTheme(iced::Theme),
    BackToMenu,
}
