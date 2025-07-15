use crate::message::states::AppMessage;
use crate::styles::custom_style;
use crate::utils::frecuency::get_frecuency;
use crate::utils::helper_json;
use crate::utils::utils;
use crate::widgets::notes::Note;
use crate::widgets::partiture::Partiture;
use crate::{asset_path, models::settings::CustomSettings};
use iced::widget::Text;
use iced::{
    Element, Length, Renderer, Theme,
    alignment::{Horizontal, Vertical},
    widget::{Container, column},
};
use serde_json::Value;
use std::time::Instant;

// Menú del juego
pub fn game_view<'a>(
    partiture_name: Option<&'a str>,
    start_time: Option<Instant>,
    actual_time: Option<Instant>,
    settings: &'a CustomSettings,
) -> Element<'a, AppMessage> {
    let elapsed: f32 =
        if let (Some(start), Some(current)) = (start_time.as_ref(), actual_time.as_ref()) {
            current.duration_since(*start).as_secs_f32()
        } else {
            0.0
        };

    // Cargar notas del archivo JSON
    let arr: Result<Vec<Value>, String> = helper_json::load_partiture(&asset_path!("notes.json"));
    let (notes_l, notes_r) = match &arr {
        Ok(data) => (
            helper_json::load_notes_from_file(data, partiture_name.unwrap_or("for-elise"), "left")
                .unwrap_or_default(),
            helper_json::load_notes_from_file(data, partiture_name.unwrap_or("for-elise"), "right")
                .unwrap_or_default(),
        ),
        Err(e) => {
            println!("Error cargando partitura: {}", e);
            (Vec::new(), Vec::new())
        }
    };

    // Crear instancias de partituras con notas
    let mut partiture_l: Partiture = Partiture::new(
        Vec::new(),
        notes_l
            .iter()
            .fold(0.0, |acc: f32, note: &Note| acc + note.duration),
        elapsed,
        settings.clone(),
    );
    let mut partiture_r: Partiture = Partiture::new(
        Vec::new(),
        notes_r
            .iter()
            .fold(0.0, |acc: f32, note: &Note| acc + note.duration),
        elapsed,
        settings.clone(),
    );

    // Crear el título de la partitura
    let title: Element<AppMessage> = Text::new(partiture_name.unwrap_or("for-elise"))
        .size(40)
        .width(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .style(custom_style::partiture_title)
        .into();

    // Añadir notas a las partituras y actualizar datos de tiempo
    partiture_l.notes.extend(notes_l.iter().cloned());
    partiture_r.notes.extend(notes_r.iter().cloned());

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
