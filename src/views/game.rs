use crate::{
    message::states::AppMessage,
    models::settings::CustomSettings,
    styles::custom_style,
    utils::utils,
    widgets::{intro_overlay::IntroOverlay, notes::Note, partiture::Partiture},
};
use iced::{
    Element, Length, Padding,
    alignment::{Horizontal, Vertical},
    widget::{Canvas, Column, Container, Stack, Text, column},
};
use std::time::Instant;

// Menú del juego
pub fn game_view<'a>(
    start_time: Option<Instant>,
    actual_time: Option<Instant>,
    settings: &'a CustomSettings,
    partiture_name: Option<&'static str>,
    partiture_r: &'a Partiture,
    partiture_l: &'a Partiture,
) -> Element<'a, AppMessage> {
    // Crear el título de la partitura
    let elapsed: f32 =
        if let (Some(start), Some(current)) = (start_time.as_ref(), actual_time.as_ref()) {
            current.duration_since(*start).as_secs_f32()
        } else {
            0.0
        };

    // Crear instancias de partituras con notas
    let partiture_r: Partiture = Partiture::new(
        partiture_r.notes.clone(),
        partiture_r
            .notes
            .iter()
            .fold(0.0, |acc: f32, note: &Note| acc + note.duration),
        elapsed,
        settings.clone(),
        "right".to_string(),
    );
    let partiture_l: Partiture = Partiture::new(
        partiture_l.notes.clone(),
        partiture_l
            .notes
            .iter()
            .fold(0.0, |acc: f32, note: &Note| acc + note.duration),
        elapsed,
        settings.clone(),
        "left".to_string(),
    );

    // Esperamos a que pase el temporizador
    let name = if elapsed < 3.0 {
        ""
    } else {
        partiture_name.unwrap_or("")
    };

    // Crear el título de la partitura
    let title: Element<AppMessage> = Container::new(
        Text::new(name)
            .size(40)
            .width(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .style(custom_style::partiture_title),
    )
    .padding(Padding {
        top: 100.0,
        right: 0.0,
        bottom: 0.0,
        left: 0.0,
    })
    .into();

    // Crear imagen de el gran pentagrama, calve de sol y clave de fa para ambas partituras
    let (partiture_r_overlay, partiture_l_overlay) =
        utils::create_grand_staff(partiture_r, partiture_l);

    let overlay = Canvas::new(IntroOverlay { elapsed })
        .width(Length::Fill)
        .height(Length::Fill);

    // Crear la columna principal del juego
    let game_content: Column<AppMessage> = column![
        title, // Título de la partitura
        partiture_r_overlay,
        partiture_l_overlay // Parte mano derecha de la partitura
    ]
    .height(Length::Fill)
    .spacing(20);

    // Contenedor principal del juego
    Container::new(Stack::new().push(game_content).push(overlay))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .style(|_theme| custom_style::background(_theme, settings.theme.clone()))
        .into()
}
