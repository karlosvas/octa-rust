use iced::{Color, Theme};
use iced::Background;
use iced::widget::container::Appearance;

pub fn background(_theme: &Theme) -> Appearance {
    Appearance {
        background: Some(Background::Color(Color::from_rgb8(30, 30, 30))),
        text_color: Some(Color::WHITE),
        border_radius: 0.0,
        border_width: 0.0,
        border_color: Color::BLACK,
    }
}

pub fn partiture(_theme: &Theme) -> Appearance {
    Appearance {
        background: Some(Background::Color(Color::WHITE)),
        text_color: Some(Color::BLACK),
        border_radius: 0.0,
        border_width: 0.0,
        border_color: Color::BLACK,
    }
}

pub fn staff_line(_theme: &Theme) -> Appearance {
    Appearance {
        background: Some(Background::Color(Color::BLACK)),
        text_color: None,
        border_radius: 0.0,
        border_width: 0.0,
        border_color: Color::TRANSPARENT,
    }
}