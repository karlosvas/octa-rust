use crate::{
    message::states::AppMessage,
    models::settings::CustomSettings,
    styles::custom_style,
    utils::utils,
    widgets::{notes::Note, partiture::Partiture},
};
use iced::{
    Element, Length, Renderer, Theme,
    alignment::{Horizontal, Vertical},
    widget::{Container, Text, column},
};
use std::time::Instant;

// Menú del juego
pub fn game_view<'a>(
    partiture_name: Option<&'a str>,
    start_time: Option<Instant>,
    actual_time: Option<Instant>,
    settings: &'a CustomSettings,
    notes_l: &'a Vec<Note>,
    notes_r: &'a Vec<Note>,
) -> Element<'a, AppMessage> {
    // Crear el título de la partitura
    let elapsed: f32 =
        if let (Some(start), Some(current)) = (start_time.as_ref(), actual_time.as_ref()) {
            current.duration_since(*start).as_secs_f32()
        } else {
            0.0
        };

    // Crear instancias de partituras con notas
    let partiture_l: Partiture = Partiture::new(
        notes_l.clone(),
        notes_l
            .iter()
            .fold(0.0, |acc: f32, note: &Note| acc + note.duration),
        elapsed,
        settings.clone(),
        "left".to_string(),
    );
    let partiture_r: Partiture = Partiture::new(
        notes_r.clone(),
        notes_r
            .iter()
            .fold(0.0, |acc: f32, note: &Note| acc + note.duration),
        elapsed,
        settings.clone(),
        "right".to_string(),
    );

    // Crear el título de la partitura
    let title: Element<AppMessage> = Text::new(partiture_name.unwrap_or("for-elise"))
        .size(40)
        .width(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .style(custom_style::partiture_title)
        .into();

    // Crear elementos de partitura para la vista junto a las notas
    let mut partiture_r_overlay: Element<'_, AppMessage, Theme, Renderer> =
        Element::new(partiture_r);
    let mut partiture_l_overlay: Element<'_, AppMessage, Theme, Renderer> =
        Element::new(partiture_l);

    // Crear imagen de el gran pentagrama, calve de sol y clave de fa para ambas partituras
    utils::create_grand_staff(&mut partiture_r_overlay, &mut partiture_l_overlay);

    // Crear la columna principal del juego
    let game_column = column![
        title,               // Título de la partitura
        partiture_r_overlay, // Parte mano derecha de la partitura
        partiture_l_overlay, // Parte mano izquierda de la partitura
    ]
    .spacing(20);

    // Contenedor principal del juego
    Container::new(game_column)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .style(|_theme| custom_style::background(_theme, settings.theme.clone()))
        .into()
}
