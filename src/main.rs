mod app;
mod message;
mod styles;
use iced::{window::Position, Result, Sandbox, Settings};
use app::MyApp;

fn main() -> Result {
    MyApp::run(Settings {
        window: iced::window::Settings {
            position: Position::Centered, // Centra la ventana al inicio
            min_size: Some((400, 300)), // Tamaño mínimo (opcional)
            max_size: None, // Tamaño máximo (opcional)
            ..Default::default() // Otras configuraciones por defecto
        },
        ..Default::default() // Otras configuraciones por defecto
    })
}