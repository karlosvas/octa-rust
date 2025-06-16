use iced::{Color, Theme, Border, Shadow, Background};
use iced::widget::container;

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

pub fn partiture(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: Some(Color::BLACK),
        background: Some(Background::Color(Color::WHITE)),
        border: Border {
            color: Color::TRANSPARENT,
            ..Default::default()
        },
        shadow: Shadow {
            color: Color::TRANSPARENT,
            ..Default::default()
        },
    }
}

pub fn staff_line(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: Some(Color::WHITE),
        background: Some(Background::Color(Color::from_rgb8(30, 30, 30))),
        border: Border {
            color: Color::TRANSPARENT,
            ..Default::default()
        },
        shadow: Shadow {
            color: Color::TRANSPARENT,
            ..Default::default()
        },
    }
}