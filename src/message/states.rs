// Enumeración que define los mensajes(acciones) que la aplicación puede recibir
#[derive(Debug, Clone, Copy)]
pub enum Buttons {
    Play,
    Exit,
}

#[derive(Debug, Clone, Copy)]
pub enum AppState {
    MainMenu,
    Game,
    // Settings,
}
