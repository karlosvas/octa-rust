use iced::{
    Background, Border, Color, Shadow, Theme, Vector,
    widget::{button, container, text, toggler},
};

// Paleta de colores de la aplicación
pub struct ColorPalette;

impl ColorPalette {
    // Tema Light
    pub const LIGHT_TEXT: Color = Color::from_rgb(0.314, 0.314, 0.314); // rgb8(80, 80, 80)
    pub const LIGHT_BACKGROUND: Color = Color::from_rgb(0.906, 0.851, 0.780); // rgb8(231, 217, 199) - beige/café claro

    // Tema Dark
    pub const DARK_TEXT: Color = Color::WHITE;
    pub const DARK_BACKGROUND: Color = Color::from_rgb(0.196, 0.145, 0.118); // rgb8(50, 37, 30) - café oscuro

    // Colores de acento marrones
    pub const ACCENT_RED: Color = Color::from_rgb(0.98, 0.10, 0.10);
    pub const ACCENT_BROWN: Color = Color::from_rgb(0.545, 0.271, 0.075); // rgb8(139, 69, 19) - SaddleBrown
    pub const ACCENT_BROWN_HOVER: Color = Color::from_rgb(0.647, 0.325, 0.106); // rgb8(165, 83, 27)
    pub const ACCENT_BROWN_PRESSED: Color = Color::from_rgb(0.471, 0.235, 0.063); // rgb8(120, 60, 16)
    pub const ACCENT_BROWN_DISABLED: Color = Color::from_rgb(0.392, 0.196, 0.098); // rgb8(100, 50, 25)

    // Colores de acento naranja
    pub const ACCENT_ORANGE: Color = Color::from_rgb(0.902, 0.490, 0.118); // rgb8(230, 125, 30)
    pub const ACCENT_ORANGE_HOVER: Color = Color::from_rgb(1.0, 0.549, 0.0); // rgb8(255, 140, 0) - DarkOrange
    pub const ACCENT_ORANGE_PRESSED: Color = Color::from_rgb(0.804, 0.431, 0.098); // rgb8(205, 110, 25)
    pub const ACCENT_ORANGE_DISABLED: Color = Color::from_rgb(0.706, 0.392, 0.157); // rgb8(180, 100, 40)

    // Colores de sombra
    pub const SHADOW_DARK: Color = Color::from_rgb(0.196, 0.196, 0.196); // rgb8(50, 50, 50)

    // Colores generales
    pub const TRANSPARENT: Color = Color::TRANSPARENT;
}

// Estilo personalizado para el fondo de la aplicación
pub fn background(theme: &Theme) -> container::Style {
    let (text_color, background_color) = match theme {
        Theme::Light => (
            ColorPalette::LIGHT_TEXT,
            Background::Color(ColorPalette::LIGHT_BACKGROUND),
        ),
        Theme::Dark => (
            ColorPalette::DARK_TEXT,
            Background::Color(ColorPalette::DARK_BACKGROUND),
        ),
        _ => (
            ColorPalette::DARK_TEXT,
            Background::Color(ColorPalette::DARK_BACKGROUND),
        ),
    };

    container::Style {
        text_color: Some(text_color),
        background: Some(background_color),
        border: Border {
            color: ColorPalette::TRANSPARENT,
            ..Default::default()
        },
        shadow: Shadow {
            color: ColorPalette::SHADOW_DARK,
            offset: Vector::new(0.0, 2.0),
            blur_radius: 5.0,
        },
    }
}

// Estilo personalizado para el boton de configuración
pub fn button_settings(_theme: &Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: None,
        text_color: ColorPalette::ACCENT_BROWN,
        border: Border {
            color: ColorPalette::TRANSPARENT,
            width: 0.0,
            radius: 5.0.into(),
        },
        shadow: Shadow::default(),
    }
}

// Estilo personalizado para el título de la partitura
pub fn partiture_title(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(ColorPalette::ACCENT_RED),
        ..Default::default()
    }
}

// Botón de volber atrás
pub fn button_back(_theme: &Theme, status: button::Status) -> button::Style {
    let bg: Color = match status {
        button::Status::Active => ColorPalette::ACCENT_BROWN,
        button::Status::Hovered => ColorPalette::ACCENT_BROWN_HOVER,
        button::Status::Pressed => ColorPalette::ACCENT_BROWN_PRESSED,
        button::Status::Disabled => ColorPalette::ACCENT_BROWN_DISABLED,
    };

    button::Style {
        background: Some(iced::Background::Color(bg)),
        text_color: ColorPalette::DARK_TEXT,
        border: Border::default(),
        shadow: Shadow::default(),
    }
}

// Estilo de botón al cambiar entre temas
pub fn toogle_theme(theme: &Theme, _status: toggler::Status) -> toggler::Style {
    match theme {
        Theme::Dark => toggler::Style {
            background: ColorPalette::DARK_BACKGROUND,
            foreground: ColorPalette::LIGHT_BACKGROUND,
            background_border_width: 1.0,
            background_border_color: Color::BLACK,
            foreground_border_width: 1.0,
            foreground_border_color: Color::TRANSPARENT,
        },
        Theme::Light => toggler::Style {
            background: ColorPalette::LIGHT_BACKGROUND,
            foreground: ColorPalette::DARK_BACKGROUND,
            background_border_width: 1.0,
            background_border_color: Color::BLACK,
            foreground_border_width: 1.0,
            foreground_border_color: Color::TRANSPARENT,
        },
        _ => toggler::Style {
            background: ColorPalette::DARK_BACKGROUND,
            foreground: ColorPalette::LIGHT_BACKGROUND,
            background_border_width: 1.0,
            background_border_color: Color::BLACK,
            foreground_border_width: 1.0,
            foreground_border_color: Color::TRANSPARENT,
        },
    }
}

// Estilo de los botones por defecto
pub fn buttons(_theme: &Theme, status: button::Status) -> button::Style {
    let (bg, shadow_offset, shadow_blur) = match status {
        button::Status::Active => (ColorPalette::ACCENT_BROWN, Vector::new(0.0, 4.0), 8.0),
        button::Status::Hovered => (
            ColorPalette::ACCENT_BROWN_HOVER,
            Vector::new(0.0, 6.0),
            12.0,
        ),
        button::Status::Pressed => (
            ColorPalette::ACCENT_BROWN_PRESSED,
            Vector::new(0.0, 2.0),
            4.0,
        ),
        button::Status::Disabled => (
            ColorPalette::ACCENT_BROWN_DISABLED,
            Vector::new(0.0, 0.0),
            0.0,
        ),
    };

    // Usar siempre blanco para mejor contraste con el fondo marrón
    let text_color = Color::WHITE;

    let border_color = match status {
        button::Status::Hovered => Color::from_rgb(0.902, 0.490, 0.118),
        _ => ColorPalette::TRANSPARENT,
    };

    button::Style {
        background: Some(Background::Color(bg)),
        text_color,
        border: Border {
            color: border_color,
            width: if matches!(status, button::Status::Hovered) {
                2.0
            } else {
                0.0
            },
            radius: 8.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            offset: shadow_offset,
            blur_radius: shadow_blur,
        },
    }
}

pub fn button_selection(_theme: &Theme, status: button::Status) -> button::Style {
    let (bg, shadow_offset, shadow_blur) = match status {
        button::Status::Active => (ColorPalette::ACCENT_ORANGE, Vector::new(0.0, 4.0), 8.0),
        button::Status::Hovered => (
            ColorPalette::ACCENT_ORANGE_HOVER,
            Vector::new(0.0, 6.0),
            12.0,
        ),
        button::Status::Pressed => (
            ColorPalette::ACCENT_ORANGE_PRESSED,
            Vector::new(0.0, 2.0),
            4.0,
        ),
        button::Status::Disabled => (
            ColorPalette::ACCENT_ORANGE_DISABLED,
            Vector::new(0.0, 0.0),
            0.0,
        ),
    };

    // Usar siempre blanco para mejor contraste con el fondo marrón
    let text_color: Color = Color::WHITE;

    let border_color: Color = match status {
        button::Status::Hovered => Color::from_rgb(0.902, 0.490, 0.118),
        _ => ColorPalette::TRANSPARENT,
    };

    button::Style {
        background: Some(Background::Color(bg)),
        text_color,
        border: Border {
            color: border_color,
            width: if matches!(status, button::Status::Hovered) {
                2.0
            } else {
                0.0
            },
            radius: 3.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            offset: shadow_offset,
            blur_radius: shadow_blur,
        },
    }
}
