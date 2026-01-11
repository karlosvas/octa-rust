use {
    crate::{
        asset_path,
        message::states::{AppMessage, SelectionMessage},
        styles::custom_style,
        utils::{helper_json, reusable::create_button},
    },
    iced::{
        Element, Length,
        alignment::{Horizontal, Vertical},
        widget::{Button, Column, Container, Text, column},
    },
    serde_json::Value,
};

pub fn select_partiture_view() -> Element<'static, AppMessage> {
    // Cargar las partituras desde el archivo JSON
    let partitures: Vec<Value> =
        helper_json::load_partiture(&asset_path!("partitures.json")).unwrap();

    // Vista de las partituras
    let mut partiture_column: Column<AppMessage> = column![].spacing(20);

    // Iterar sobre las partituras y crear un contenedor para cada una
    for partiture in &partitures {
        if let Some((name, _data)) = partiture.as_object().and_then(|obj| obj.iter().next()) {
            let name_to_show: String = name
                .replace("-", " ")
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if i == 0 {
                        c.to_uppercase().collect::<String>()
                    } else {
                        c.to_string()
                    }
                })
                .collect::<String>();

            let partiture_button: Button<AppMessage> = Button::new(Text::new(name_to_show))
                // Al pulsar el botón, enviar el mensaje para iniciar el juego con la partitura seleccionada
                .on_press(AppMessage::Selection(SelectionMessage::StartGame(
                    name.clone().leak(),
                )))
                .width(Length::Fixed(500.0))
                .padding(10)
                .style(custom_style::button_selection);

            partiture_column = partiture_column.push(partiture_button);
        }
    }

    // Añadir boton de volver al menú principal
    let back_button: Button<AppMessage> = create_button(
        AppMessage::Selection(SelectionMessage::BackToMenu),
        Some("Volver al menú"),
        None,
        Some(20.0),
    );
    partiture_column = partiture_column.push(back_button);

    Container::new(partiture_column)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .style(custom_style::background)
        .into()
}
