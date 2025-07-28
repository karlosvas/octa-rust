use crate::{
    asset_path,
    message::states::{AppMessage, SelectionMessage},
    models::settings::CustomSettings,
    styles::custom_style,
    utils::helper_json,
};
use iced::widget::{Button, Column};
use iced::{
    Element, Length,
    alignment::{Horizontal, Vertical},
    widget::{Container, Text, column},
};
use serde_json::Value;

pub fn select_partiture_view(settings: &CustomSettings) -> Element<AppMessage> {
    // Cargar las partituras desde el archivo JSON
    let partitures: Vec<Value> = helper_json::load_partiture(&asset_path!("notes.json")).unwrap();

    // Vista de las partituras
    let mut partiture_column: Column<'_, AppMessage> = column![];

    // Iterar sobre las partituras y crear un contenedor para cada una
    for partiture in &partitures {
        if let Some((name, _data)) = partiture.as_object().and_then(|obj| obj.iter().next()) {
            let info: String = format!("Nombre: {}\n", name);

            let partiture_button: Button<'_, AppMessage> = Button::new(Text::new(info))
                .on_press(AppMessage::Selection(SelectionMessage::StartGame(
                    name.clone(),
                )))
                .width(Length::Fixed(500.0))
                .padding(10);

            partiture_column = partiture_column.push(partiture_button);
        }
    }

    // Añadir boton de volver al menú principal
    let back_button: Button<'_, AppMessage> = Button::new(Text::new("Volver al menú"))
        .on_press(AppMessage::Selection(SelectionMessage::BackToMenu))
        .width(Length::Fixed(500.0))
        .padding(10);
    partiture_column = partiture_column.push(back_button);

    Container::new(partiture_column)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .style(|_theme| custom_style::background(_theme, settings.theme.clone()))
        .into()
}
