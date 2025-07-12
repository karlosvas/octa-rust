use iced::{
    Element, Length,
    alignment::{Horizontal, Vertical},
    widget::{Button, Container, Image, Row, Text, image::Handle},
};

// Crear botones
pub fn create_button<Message>(
    message: Message,
    text: Option<&str>,
    image: Option<Image>,
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
        content.push(Text::new(txt).size(32).into());
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
