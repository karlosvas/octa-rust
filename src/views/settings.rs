use crate::{
    message::states::{AppMessage, GameMessage, SettingsMessage},
    models::settings::{CustomSettings, CustomTheme, Difficulty},
    styles::custom_style,
    utils::reusable,
};
use iced::{
    Element, Length, Theme,
    alignment::{Horizontal, Vertical},
    widget::{Button, Column, Container, Slider, column},
};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

// Vista de configuración
pub fn settings_view(settings: &CustomSettings) -> Element<AppMessage> {
    // Slider para cambiar el tema
    let theme_slider: Slider<'_, i32, AppMessage, Theme> =
        Slider::new(0..=1, settings.theme.get_theme(), |val: i32| {
            AppMessage::Settings(SettingsMessage::ChangeTheme(
                CustomTheme::get_theme_from_int(val),
            ))
        })
        .step(1)
        .width(400.0);

    // Slider para cambiar la dificultad
    let dificulty_slider: Slider<'_, f32, AppMessage, Theme> = Slider::new(
        0.5f32..=1.5f32,
        settings.difficulty.get_multiplier(),
        |val: f32| {
            AppMessage::Settings(SettingsMessage::ChangeDifficulty(
                Difficulty::get_dificulty_from_f32(val),
            ))
        },
    )
    .step(0.5f32)
    .width(400.0);

    // Boton para volver al menú principal
    let back_to_menu: Button<'_, AppMessage> = reusable::create_button(
        AppMessage::Settings(SettingsMessage::BackToMenu),
        Some("Back to Main Menu"),
        None,
    );

    // Crear columna de configuración
    Container::new(column![theme_slider, dificulty_slider, back_to_menu].spacing(20))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .style(|theme| custom_style::background(theme, settings.theme.clone()))
        .into()
}

// Menú de pausa
pub fn paused_view(finished: Arc<AtomicBool>, settings: &CustomSettings) -> Element<AppMessage> {
    let mut pause_column: Column<AppMessage> = column![].spacing(20);

    if !finished.load(Ordering::SeqCst) {
        // Crear botón para reanudar el juego
        let resume_button: Button<AppMessage> = reusable::create_button(
            AppMessage::Game(GameMessage::ResumeGame),
            Some("Resume Game"),
            None,
        );
        pause_column = pause_column.push(resume_button);
    }

    // Crear botón para reiniciar el juego
    let restart_button: Button<AppMessage> = reusable::create_button(
        AppMessage::Game(GameMessage::RestartGame),
        Some("Restart Game"),
        None,
    );
    pause_column = pause_column.push(restart_button);

    // Crear botón para volver al menú principal
    let back_to_menu_button: Button<AppMessage> = reusable::create_button(
        AppMessage::Settings(SettingsMessage::BackToMenu),
        Some("Back to Main Menu"),
        None,
    );
    pause_column = pause_column.push(back_to_menu_button);

    // Contenedor principal del menú de pausa
    Container::new(pause_column)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .style(|_theme| custom_style::background(_theme, settings.theme.clone()))
        .into()
}
