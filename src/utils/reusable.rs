use iced::{
    Element, Length,
    alignment::{Horizontal, Vertical},
    widget::{Button, Container, Image, Row, Text, image::Handle},
};

use crate::styles::custom_style;

// Crear botones
pub fn create_button<Message>(
    message: Message,
    text: Option<&str>,
    image: Option<Image>,
    size: Option<f32>,
) -> Button<'_, Message>
where
    Message: Clone + 'static,
{
    let mut content: Vec<Element<Message>> = Vec::new();

    // Añadir imagen si existe
    if let Some(img) = image {
        content.push(img.into());
    }

    // Añadir texto si existe
    if let Some(txt) = text {
        content.push(
            Text::new(txt)
                .size(size.unwrap_or(20.0))
                .font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .into(),
        );
    }

    // Crear contenedor con el contenido
    let button_content = if content.len() == 1 {
        content.into_iter().next().unwrap()
    } else {
        Row::with_children(content)
            .spacing(10)
            .align_y(Vertical::Center)
            .into()
    };

    Button::new(
        Container::new(button_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(12)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center),
    )
    .on_press(message)
    .width(Length::Fixed(250.0))
    .height(Length::Fixed(60.0))
    .style(custom_style::buttons)
}

// Crear imagenes
pub fn create_image(path: &str, heigth: f32, width: f32) -> Image {
    Image::new(Handle::from_path(path))
        .width(Length::Fixed(width))
        .height(Length::Fixed(heigth))
}
