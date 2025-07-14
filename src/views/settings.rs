use crate::message::states::{AppMessage, GameMessage, SettingsMessage};
use crate::models::settings::{CustomSettings, CustomTheme, Difficulty};
use crate::styles::custom_style;
use crate::utils::reusable;
use iced::{
    Element, Length, Theme,
    alignment::{Horizontal, Vertical},
    widget::{Button, Column, Container, Slider, column},
};

// Vista de configuración
pub fn settings_view(settings: &CustomSettings) -> Element<AppMessage> {
    // Slider para cambiar el tema
    let theme_slider: Slider<'_, i32, AppMessage, Theme> =
        Slider::new(0..=1, settings.theme.get_theme(), |val| {
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
        |val| {
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
pub fn paused_view(settings: &CustomSettings) -> Element<AppMessage> {
    // Crear botón para reanudar el juego
    let resume_button: Button<'_, AppMessage> = reusable::create_button(
        AppMessage::Game(GameMessage::ResumeGame),
        Some("Resume Game"),
        None,
    );

    // Crear botón para reiniciar el juego
    let restart_button: Button<'_, AppMessage> = reusable::create_button(
        AppMessage::Game(GameMessage::RestartGame),
        Some("Restart Game"),
        None,
    );

    // Crear botón para volver al menú principal
    let back_to_menu_button: Button<'_, AppMessage> = reusable::create_button(
        AppMessage::Settings(SettingsMessage::BackToMenu),
        Some("Back to Main Menu"),
        None,
    );

    // Crear columna de pausa
    let pause_column: Column<'_, AppMessage> =
        column![resume_button, restart_button, back_to_menu_button,].spacing(20);

    Container::new(pause_column)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .style(|_theme| custom_style::background(_theme, settings.theme.clone()))
        .into()
}
