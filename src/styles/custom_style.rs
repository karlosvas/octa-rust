use iced::widget::container;
use iced::{Background, Border, Color, Shadow, Theme};

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
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 5.0,
        },
    }
}
