use {
    iced::{
        Font, Result, Size,
        advanced::graphics::core::window,
        application,
        font::{Family, Stretch, Style, Weight},
        window::{Icon, Position, icon::from_file_data},
    },
    octarust::{MyApp, asset_path},
    std::{borrow::Cow, fs::read},
};

/// Punto de inicio de la aplicación
fn main() -> Result {
    env_logger::init();

    // Incluir un icono para la ventana de la aplicación
    let icon_bytes: Vec<u8> = read(asset_path!("octarust.png")).expect("Icon file not found");
    let icon: Option<Icon> = from_file_data(&icon_bytes, None).ok();

    // Configuración de la ventana de la aplicación
    let window_settings: window::Settings = window::Settings {
        icon: icon,                              // Icono de la ventana
        position: Position::Centered,            // Centrar la ventana
        min_size: Some(Size::new(600.0, 600.0)), // Tamaño mínimo de la ventana
        resizable: true,                         // Permitir redimensionar la ventana
        decorations: true,                       // Mostrar bordes y barra de título
        transparent: false,                      // No transparente
        ..window::Settings::default()
    };

    // Configuración de la fuente para la aplicación
    let rustica_font_bytes: Vec<u8> =
        read(asset_path!("PlayfairDisplay-Medium.ttf")).expect("Font file not found");
    let settings: iced::Settings = iced::Settings {
        id: Some("OctaRust".to_string()),
        fonts: vec![Cow::Owned(rustica_font_bytes)],
        default_font: Font {
            family: Family::Name("PlayfairDisplay-Medium".into()),
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
        .theme(MyApp::theme)
        .run()
}
