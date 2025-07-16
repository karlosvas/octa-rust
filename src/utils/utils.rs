use crate::app::MyApp;
use crate::message::states::AppMessage;
use crate::widgets::notes::Note;
use crate::widgets::partiture::Partiture;
use crate::{asset_path, utils::reusable::create_image};
use iced::Point;
use iced::{
    Element, Length, Padding,
    alignment::Vertical,
    widget::{Container, Image, Stack},
};
use std::mem::replace;

pub fn create_grand_staff(
    partiture_r_overlay: &mut Element<AppMessage>,
    partiture_l_overlay: &mut Element<AppMessage>,
) {
    // Primero extraemos el valor original usando std::mem::replace
    let original_r: Element<'_, AppMessage> =
        replace(partiture_r_overlay, Element::new(Partiture::default()));

    // Luego hacemos lo mismo con la partitura izquierda
    let original_l: Element<'_, AppMessage> =
        replace(partiture_l_overlay, Element::new(Partiture::default()));

    // Imagenes de clave de sol y fa
    let partitura_r_img: Image = create_image(&asset_path!("clave-de-sol.png"), 200.0, 80.0);
    let partitura_l_img: Image = create_image(&asset_path!("clave-de-fa.png"), 200.0, 90.0);

    // Usamos Stack para superponer las imágenes sobre las partituras cambiando el valor de la referencia mutable
    *partiture_r_overlay = Stack::new()
        .height(Length::Fixed(250.0))
        .push(original_r) // Usamos el valor original
        .push(
            Container::new(partitura_r_img)
                .height(Length::Fill)
                .align_y(Vertical::Center)
                .padding(Padding {
                    top: 10.0,
                    right: 20.0,
                    bottom: 0.0,
                    left: 20.0,
                }),
        )
        .into();

    // Usamos Stack para superponer las imágenes sobre las partituras cambiando el valor de la referencia mutable
    *partiture_l_overlay = Stack::new()
        .height(Length::Fixed(250.0))
        .push(original_l) // Usamos el valor original
        .push(
            Container::new(partitura_l_img)
                .height(Length::Fill)
                .align_y(Vertical::Center)
                .padding(Padding {
                    top: 0.0,
                    right: 20.0,
                    bottom: 35.0,
                    left: 20.0,
                }),
        )
        .into();
}

/// Crea un overlay de introducción temporizado para la partitura
pub fn create_tempo_overlay(notes: &mut Vec<Note>, elapsed: f32) {
    // Calcular cuando juntar las notas
    let mut joined: u8 = 0;
    // let mut current_position: Point;

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
                // note.position = current_position;
                joined = 0;
            }
        } else {
            joined = 0;
            note.joined = false;
        }
    }
}
