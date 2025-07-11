use crate::message::states::Buttons;
use crate::utils::reusable;
use crate::widgets::partiture::Partiture;
use iced::widget::Stack;
use iced::{
    Element, Length,
    alignment::Vertical,
    widget::{Container, Image},
};

pub fn create_grand_staff(
    partiture_r_overlay: &mut Element<Buttons>,
    partiture_l_overlay: &mut Element<Buttons>,
) {
    // Primero extraemos el valor original usando std::mem::replace
    let original_r = std::mem::replace(
        partiture_r_overlay,
        Element::new(Partiture::new("temp".to_string(), Vec::new(), 0.0)),
    );

    // Luego hacemos lo mismo con la partitura izquierda
    let original_l = std::mem::replace(
        partiture_l_overlay,
        Element::new(Partiture::new("temp".to_string(), Vec::new(), 0.0)),
    );

    // Imagenes de clave de sol y fa
    let partitura_l_img: Image = reusable::create_image(
        &format!("{}/assets/clave-de-fa.png", env!("CARGO_MANIFEST_DIR")),
        150.0,
        80.0,
    );
    let partitura_r_img: Image = reusable::create_image(
        &format!("{}/assets/clave-de-sol.png", env!("CARGO_MANIFEST_DIR")),
        125.0,
        80.0,
    );

    // Usamos Stack para superponer las imágenes sobre las partituras cambiando el valor de la referencia mutable
    *partiture_r_overlay = Stack::new()
        .push(original_r) // Usamos el valor original
        .push(
            Container::new(partitura_r_img)
                .height(Length::Fill)
                .align_y(Vertical::Center)
                .padding(iced::Padding {
                    top: 25.0,
                    right: 20.0,
                    bottom: 0.0,
                    left: 20.0,
                }),
        )
        .into();

    // Usamos Stack para superponer las imágenes sobre las partituras cambiando el valor de la referencia mutable
    *partiture_l_overlay = Stack::new()
        .push(original_l) // Usamos el valor original
        .push(
            Container::new(partitura_l_img)
                .height(Length::Fill)
                .align_y(Vertical::Center)
                .padding(iced::Padding {
                    top: 0.0,
                    right: 20.0,
                    bottom: 22.0,
                    left: 20.0,
                }),
        )
        .into();
}
