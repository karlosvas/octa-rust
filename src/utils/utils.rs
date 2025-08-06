use crate::{
    asset_path,
    message::states::AppMessage,
    utils::reusable::create_image,
    widgets::{notes::Note, partiture::Partiture},
};
use iced::{
    Length, Padding,
    widget::{Canvas, Container, Stack},
};

pub fn create_grand_staff(
    partiture_r: Partiture,
    partiture_l: Partiture,
) -> (
    Container<'static, AppMessage>,
    Container<'static, AppMessage>,
) {
    let fixed_height_staff: f32 = 200.0;

    let original_r = Container::new(
        Canvas::new(partiture_r)
            .width(Length::Fill)
            .height(Length::Fixed(fixed_height_staff)),
    )
    .width(Length::Fill)
    .height(Length::Fixed(fixed_height_staff))
    .padding(Padding {
        top: 0.0,
        right: 20.0,
        bottom: 0.0,
        left: 10.0,
    });

    let original_l = Container::new(
        Canvas::new(partiture_l)
            .width(Length::Fill)
            .height(Length::Fixed(fixed_height_staff)),
    )
    .width(Length::Fill)
    .height(Length::Fixed(fixed_height_staff))
    .padding(Padding {
        top: 0.0,
        right: 20.0,
        bottom: 0.0,
        left: 10.0,
    });

    let partitura_r_img =
        Container::new(create_image(&asset_path!("clave-de-sol.png"), 180.0, 80.0))
            .height(Length::Fixed(fixed_height_staff))
            .padding(Padding {
                top: 10.0,
                right: 20.0,
                bottom: 0.0,
                left: 20.0,
            });

    let partitura_l_img =
        Container::new(create_image(&asset_path!("clave-de-fa.png"), 150.0, 80.0))
            .height(Length::Fixed(fixed_height_staff))
            .padding(Padding {
                top: 20.0,
                right: 20.0,
                bottom: 0.0,
                left: 20.0,
            });

    let partiture_r_overlay = Container::new(
        Stack::new()
            .push(original_r)
            .push(partitura_r_img)
            .height(Length::Fixed(fixed_height_staff)),
    )
    .height(Length::Fixed(fixed_height_staff))
    .width(Length::Fill);

    let partiture_l_overlay = Container::new(
        Stack::new()
            .push(original_l)
            .push(partitura_l_img)
            .height(Length::Fixed(fixed_height_staff)),
    )
    .height(Length::Fixed(fixed_height_staff))
    .width(Length::Fill);

    (partiture_r_overlay, partiture_l_overlay)
}

/// Crea un overlay de introducción temporizado para la partitura
pub fn create_tempo_overlay(notes: &mut Vec<Note>, elapsed: f32) {
    // Calcular cuando juntar las notas
    let mut joined: u8 = 0;

    for note in notes.iter_mut() {
        note.is_active = true;
        if note.start > elapsed {
            note.is_active = false;
        }
        if note.duration == 0.5 {
            joined += 1;
            // current_position = note.position;
            if joined == 2 {
                // Si es la segunda vez entonces juntamos
                note.joined = true;
                joined = 0;
            }
        } else {
            joined = 0;
            note.joined = false;
        }
    }
}
