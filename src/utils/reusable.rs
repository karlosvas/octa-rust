use iced::{
    Length,
    alignment::{Horizontal, Vertical},
    widget::{Button, Container, Image, Text, image::Handle},
};

use crate::message::states::Buttons;

// Crear botones
pub fn create_button(text: &str, message: Buttons) -> Button<'_, Buttons> {
    Button::new(
        Container::new(Text::new(text))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center),
    )
    .on_press(message)
    .width(Length::Fixed(400.0))
    .height(Length::Fixed(50.0))
}

// Crear imagenes
pub fn create_image(path: &str, heigth: f32, width: f32) -> Image {
    Image::new(Handle::from_path(path))
        .width(Length::Fixed(width))
        .height(Length::Fixed(heigth))
}
