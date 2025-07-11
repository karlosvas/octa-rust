mod app;
mod message;
mod styles;
mod utils;
mod widgets;
use app::MyApp;
use iced::{
    Result, Size, application,
    window::{Icon, Position, Settings, icon::from_file_data},
};

// Punto de inicio de la aplicación
fn main() -> Result {
    // Incluir un icono para la ventana de la aplicación
    let icon: Option<Icon> = from_file_data(include_bytes!("../assets/octarust.png"), None).ok();

    // Configuración de la ventana de la aplicación
    let settings: Settings = Settings {
        // Configuración de la ventana
        icon: icon,                              // Icono de la ventana
        position: Position::Centered,            // Centrar la ventana
        min_size: Some(Size::new(800.0, 600.0)), // Tamaño mínimo de la ventana
        resizable: true,                         // Permitir redimensionar la ventana
        decorations: true,                       // Mostrar bordes y barra de título
        transparent: false,                      // No transparente
        ..Settings::default()
    };

    // Configurar y ejecutar la aplicación con Iced
    application("OcoRust", MyApp::update, MyApp::view)
        .window(settings)
        .settings(iced::Settings::default())
        .run()
}
