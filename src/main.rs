mod app;
mod message;
mod styles;
mod utils;
mod widgets;
use std::borrow::Cow;

use app::MyApp;
use iced::{
    Font, Result, Size, application,
    font::{Family, Stretch, Style, Weight},
    window::{Icon, Position, Settings, icon::from_file_data},
};

// Punto de inicio de la aplicación
fn main() -> Result {
    // Incluir un icono para la ventana de la aplicación
    let icon: Option<Icon> = from_file_data(include_bytes!("../assets/octarust.png"), None).ok();

    // Configuración de la ventana de la aplicación
    let window_settings: Settings = Settings {
        // Configuración de la ventana
        icon: icon,                              // Icono de la ventana
        position: Position::Centered,            // Centrar la ventana
        min_size: Some(Size::new(800.0, 600.0)), // Tamaño mínimo de la ventana
        resizable: true,                         // Permitir redimensionar la ventana
        decorations: true,                       // Mostrar bordes y barra de título
        transparent: false,                      // No transparente
        ..Settings::default()
    };

    let settings = iced::Settings {
        id: Some("OctaRust".to_string()),
        fonts: vec![Cow::Borrowed(include_bytes!("../assets/Rustica.ttf"))],
        default_font: Font {
            family: Family::Name("Rustica".into()),
            weight: Weight::Normal,
            stretch: Stretch::Normal,
            style: Style::Normal,
        },
        ..iced::Settings::default()
    };

    // Configurar y ejecutar la aplicación con Iced
    application("OctaRust", MyApp::update, MyApp::view)
        .window(window_settings)
        .settings(settings)
        .subscription(MyApp::subscription)
        .run()
}

// impl Default for Settings {
//     fn default() -> Self {
//         Self {
//             id: None,
//             fonts: Vec::new(),
//             default_font: Font::default(),
//             default_text_size: Pixels(16.0),
//             antialiasing: false,
//         }
//     }
// }

// impl From<Settings> for iced_winit::Settings {
//     fn from(settings: Settings) -> iced_winit::Settings {
//         iced_winit::Settings {
//             id: settings.id,
//             fonts: settings.fonts,
//         }
//     }
// }
