use {
    crate::{
        asset_path, message::states::AppMessage, models::partiture::Partiture,
        utils::reusable::create_image,
    },
    iced::{
        Length, Padding,
        widget::{Canvas, Container, Stack},
    },
};

pub fn create_grand_staff<'a>(
    partiture: (&'a Partiture, &'a Partiture),
) -> (Container<'a, AppMessage>, Container<'a, AppMessage>) {
    let fixed_height_staff: f32 = 200.0;

    // Crear el overlay de la partitura con clave de sol y fa
    let partiture_r: Container<AppMessage> = Container::new(
        // Crea el canvas para la partitura y se inicializa con la partitura dada
        Canvas::new(partiture.0)
            .width(Length::Fill)
            .height(Length::Fixed(fixed_height_staff)),
    )
    .width(Length::Fill)
    .height(Length::Fixed(fixed_height_staff));
    let partiture_l: Container<AppMessage> = Container::new(
        // Crea el canvas para la partitura y se inicializa con la partitura dada
        Canvas::new(partiture.1)
            .width(Length::Fill)
            .height(Length::Fixed(fixed_height_staff)),
    )
    .width(Length::Fill)
    .height(Length::Fixed(fixed_height_staff));

    // Añadir la imagen de la clave de sol y fa en la esquina izquierda
    let partitura_r_img: Container<AppMessage> =
        Container::new(create_image(&asset_path!("clave-de-sol.png"), 180.0, 80.0))
            .height(Length::Fixed(fixed_height_staff))
            .padding(Padding {
                top: 10.0,
                right: 20.0,
                bottom: 0.0,
                left: 20.0,
            });
    let partitura_l_img: Container<AppMessage> =
        Container::new(create_image(&asset_path!("clave-de-fa.png"), 150.0, 80.0))
            .height(Length::Fixed(fixed_height_staff))
            .padding(Padding {
                top: 20.0,
                right: 20.0,
                bottom: 0.0,
                left: 20.0,
            });

    // Creamos el overlay donde se encuentran todas las notas y lo dibujamos, tanto para la izquierda como a la derecha, este overlay gestiona las capas de la applicacion donde la mas inferior es la de la aprtitura, la sigen las notas, luego el fondo transparente y por ultimos la ssettings
    let partiture_r_overlay: Container<AppMessage> = Container::new(
        Stack::new()
            .push(partiture_r)
            .push(partitura_r_img)
            .height(Length::Fixed(fixed_height_staff)),
    )
    .height(Length::Fixed(fixed_height_staff))
    .width(Length::Fill)
    .padding(Padding {
        top: 0.0,
        right: 50.0,
        bottom: 0.0,
        left: 50.0,
    });
    let partiture_l_overlay: Container<AppMessage> = Container::new(
        Stack::new()
            .push(partiture_l)
            .push(partitura_l_img)
            .height(Length::Fixed(fixed_height_staff)),
    )
    .height(Length::Fixed(fixed_height_staff))
    .width(Length::Fill)
    .padding(Padding {
        top: 0.0,
        right: 50.0,
        bottom: 0.0,
        left: 50.0,
    });

    (partiture_r_overlay, partiture_l_overlay)
}
