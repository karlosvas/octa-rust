use {
    crate::{
        message::states::{AppMessage, GameMessage, SettingsMessage},
        styles::custom_style,
        utils::reusable,
    },
    iced::{
        Element, Length, Theme,
        alignment::{Horizontal, Vertical},
        widget::{Button, Column, Container, column, toggler},
    },
    std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

// Vista de configuración
pub fn settings_view(theme: &Theme) -> Element<'static, AppMessage> {
    let theme_toggle = toggler(theme == &Theme::Dark)
        .label("Cambiar Tema")
        .text_size(24)
        .size(30)
        .text_alignment(Horizontal::Center)
        .style(custom_style::toogle_theme)
        .on_toggle(|is_dark| {
            AppMessage::Settings(SettingsMessage::ChangeTheme(if is_dark {
                Theme::Dark
            } else {
                Theme::Light
            }))
        });

    let back_to_menu: Button<AppMessage> = reusable::create_button(
        AppMessage::Settings(SettingsMessage::BackToMenu),
        Some("Back to Main Menu"),
        None,
        Some(20.0),
    );

    let content_view = Container::new(column![theme_toggle, back_to_menu].spacing(20))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .width(400)
        .max_width(400);

    Container::new(content_view)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .style(custom_style::background)
        .into()
}

// Menú de pausa
pub fn paused_view(finished: Arc<AtomicBool>) -> Element<'static, AppMessage> {
    // Crear columna para el menú de pausa
    let mut pause_column: Column<AppMessage> = column![].spacing(20);

    // Si el juego no ha terminado, añadir el botón de reanudar
    if !finished.load(Ordering::SeqCst) {
        // Crear botón para reanudar el juego
        let resume_button: Button<AppMessage> = reusable::create_button(
            AppMessage::Game(GameMessage::ResumeGame),
            Some("Resume Game"),
            None,
            Some(24.0),
        );
        pause_column = pause_column.push(resume_button);
    }

    // Crear botón para reiniciar el juego
    let restart_button: Button<AppMessage> = reusable::create_button(
        // Llama al evento Restar Game
        AppMessage::Game(GameMessage::RestartGame),
        Some("Restart Game"),
        None,
        Some(24.0),
    );
    pause_column = pause_column.push(restart_button);

    // Crear botón para volver al menú principal
    let back_to_menu_button: Button<AppMessage> = reusable::create_button(
        // Llama al evento back to menu
        AppMessage::Settings(SettingsMessage::BackToMenu),
        Some("Back to Main Menu"),
        None,
        Some(24.0),
    );
    pause_column = pause_column.push(back_to_menu_button);

    // Contenedor principal del menú de pausa
    Container::new(pause_column)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .style(|theme| custom_style::background(theme))
        .into()
}
