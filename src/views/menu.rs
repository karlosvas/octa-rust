use {
    crate::{
        asset_path,
        message::states::{AppMessage, MainMenuMessage},
        styles::custom_style,
        utils::reusable::{self, create_image},
    },
    iced::{
        Element, Length,
        alignment::{Horizontal, Vertical},
        widget::{Button, Column, Container, Image, column, container, stack},
    },
};

// Menú de la applicación
pub fn main_menu_view() -> Element<'static, AppMessage> {
    // Crear imagen de OctaRust
    let img_octa_rust: Image = create_image(&asset_path!("octarust.png"), 400.0, 400.0);

    // Crear imagen de rueda configuración
    let img_settings: Image = create_image(&asset_path!("settings.png"), 400.0, 400.0);

    // Agrupación de botones
    let list_buttons_container = container(
        column![
            reusable::create_button(
                AppMessage::MainMenu(MainMenuMessage::SelectPartiture),
                Some("Play"),
                None,
                Some(24.0)
            ),
            reusable::create_button(
                AppMessage::MainMenu(MainMenuMessage::Exit),
                Some("Exit"),
                None,
                Some(24.0)
            )
        ]
        .align_x(Horizontal::Center)
        .spacing(10),
    )
    .padding(20);

    // Botón de configuración con imagen
    let button_settings: Button<AppMessage> = reusable::create_button(
        // Llama al evento open settings
        AppMessage::MainMenu(MainMenuMessage::OpenSettings),
        None,
        Some(img_settings),
        None,
    )
    .style(custom_style::button_settings)
    .width(Length::Fixed(100.0));

    // Crear columna principal del menú (imagen y botones centrados)
    let main_content: Column<AppMessage> = column![img_octa_rust, list_buttons_container]
        .spacing(20)
        .align_x(Horizontal::Center);

    // Contenedor para centrar el contenido principal
    let centered_content = container(main_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center);

    // Contenedor para el botón de settings (arriba a la derecha)
    let settings_overlay = container(
        container(button_settings)
            .width(Length::Fill)
            .align_x(Horizontal::Right)
            .padding([0, 20]),
    )
    .width(Length::Fill)
    .padding([40, 300]);

    // Stack: contenido centrado en la capa inferior, settings en la superior
    let content = stack![centered_content, settings_overlay];

    Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(custom_style::background)
        .into()
}
