use crate::asset_path;
use crate::message::states::{AppMessage, MainMenuMessage};
use crate::models::settings::CustomSettings;
use crate::styles::custom_style;
use crate::utils::reusable::{self, create_image};
use iced::{
    Element, Length,
    alignment::{Horizontal, Vertical},
    widget::{Button, Column, Container, Image, Row, Space, column, row},
};

// Menú de la applicación
pub fn main_menu_view(settings: &CustomSettings) -> Element<AppMessage> {
    // Crear imagen de OctaRust
    let img_octa_rust: Image = create_image(&asset_path!("octarust.png"), 400.0, 400.0);

    // Crear imagen de configuración
    let img_settings: Image = create_image(&asset_path!("settings.png"), 400.0, 400.0);

    // Crear botones para el menú principal
    let button_play: Button<'_, AppMessage> = reusable::create_button(
        AppMessage::MainMenu(MainMenuMessage::SelectPartiture),
        Some("Play"),
        None,
    );

    // Botón de salir y botón de configuración
    let button_exit: Button<'_, AppMessage> = reusable::create_button(
        AppMessage::MainMenu(MainMenuMessage::Exit),
        Some("Exit"),
        None,
    );

    // Botón de configuración con imagen
    let button_settings: Button<'_, AppMessage> = reusable::create_button(
        AppMessage::MainMenu(MainMenuMessage::OpenSettings),
        None,
        Some(img_settings),
    )
    .style(custom_style::button_settings)
    .width(Length::Fixed(50.0));

    // Crear fila para el botón de configuración
    let settings_row: Row<'_, AppMessage> =
        row![Space::with_width(Length::Fixed(350.0)), button_settings].spacing(10);

    // Crear columna principal del menú
    let main_column: Column<'_, AppMessage> =
        column![settings_row, img_octa_rust, button_play, button_exit,].spacing(20);
    Container::new(main_column)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .style(|_theme| custom_style::background(_theme, settings.theme.clone()))
        .into()
}
