use iced::{
    Background, Border, Color, Shadow, Theme, Vector,
    widget::{button, container},
};

// Estilo personalizado para el fondo de la aplicación
pub fn background(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: Some(Color::WHITE),
        background: Some(Background::Color(Color::from_rgb8(30, 30, 30))),
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
