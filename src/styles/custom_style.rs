use crate::models::settings::CustomTheme;
use iced::{
    Background, Border, Color, Shadow, Theme, Vector,
    widget::{button, container::Style},
};

// Estilo personalizado para el fondo de la aplicación
pub fn background(_theme: &Theme, custom_theme: CustomTheme) -> Style {
    let text_color: Color;
    let background_color: Background;
    if custom_theme == CustomTheme::Light {
        text_color = Color::from_rgb8(80, 80, 80);
        background_color = Background::Color(Color::from_rgb8(220, 220, 230));
    } else {
        text_color = Color::WHITE;
        background_color = Background::Color(Color::from_rgb8(30, 30, 30));
    }

    Style {
        text_color: Some(text_color),
        background: Some(background_color),
        border: Border {
            color: Color::TRANSPARENT,
            ..Default::default()
        },
        shadow: Shadow {
            color: Color::from_rgb8(50, 50, 50),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 5.0,
        },
    }
}

// Estilo personalizado para el boton de configuración
pub fn button_settings(_theme: &Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: None,
        text_color: Color::WHITE,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 5.0.into(),
        },
        shadow: Shadow::default(),
    }
}

// let base_color = match status {
//     button::Status::Active => Color::from_rgb8(0, 120, 215),
//     button::Status::Hovered => Color::from_rgb8(0, 100, 190),
//     button::Status::Pressed => Color::from_rgb8(0, 80, 160),
//     button::Status::Disabled => Color::from_rgb8(100, 100, 100),
// };
