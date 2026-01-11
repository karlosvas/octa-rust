use {
    crate::{
        message::states::AppMessage,
        models::{partiture::Partiture, settings::CustomSettings},
        styles::custom_style::{self, ColorPalette},
        utils::utils,
        widgets::intro_overlay::IntroOverlay,
    },
    iced::{
        Color, Element, Length,
        alignment::{Horizontal, Vertical},
        widget::{Canvas, Column, Container, Stack, Text, column, text::Shaping},
    },
};

// Menú del juego
pub fn game_view<'a>(
    partiture: (&'a Partiture, &'a Partiture),
    settings: &CustomSettings,
) -> Element<'a, AppMessage> {
    // Extraer elapsed antes de mover partiture
    let elapsed: f32 = partiture.0.elapsed;

    // Crear imagen de el gran pentagrama, calve de sol y clave de fa para ambas partituras
    let (partiture_r_overlay, partiture_l_overlay) = utils::create_grand_staff(partiture);

    let _overlay: Canvas<IntroOverlay, AppMessage> = Canvas::new(IntroOverlay { elapsed })
        .width(Length::Fill)
        .height(Length::Fill);

    // Crear la columna principal del juego
    let game_column: Column<AppMessage> = column![
        partiture_r_overlay, // Parte mano derecha de la partitura
        partiture_l_overlay, // Parte mano izquierda de la partitura
    ]
    .spacing(20);

    // Contenedor base del juego
    let game_container = Container::new(game_column)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center);

    // Crear el stack base
    let mut stack: Stack<AppMessage> = Stack::new();

    // Mostramos el timer inicial
    if elapsed < settings.timer {
        stack = stack.push(draw_intro_overlay(elapsed));
    } else {
        stack = stack.push(game_container)
    }

    Container::new(stack)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(custom_style::background)
        .into()
}

// Dibujar el temporizador de introducción
// Renderiza overlay de cuenta regresiva pre-partitura con animaciones fluidas
fn draw_intro_overlay<'a>(elapsed: f32) -> Container<'a, AppMessage> {
    // Solo opacidad, tamaño fijo para evitar temblor
    let counter = Text::new((elapsed.ceil() as i32).max(1).to_string())
        .size(250.0)
        .color(Color::from_rgba(
            ColorPalette::ACCENT_ORANGE.r,
            ColorPalette::ACCENT_ORANGE.g,
            ColorPalette::ACCENT_ORANGE.b,
            1.0,
        ))
        .font(iced::Font {
            weight: iced::font::Weight::Black,
            ..iced::Font::with_name("Arial")
        })
        .shaping(Shaping::Advanced)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center);

    Container::new(counter)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|_| iced::widget::container::Style {
            background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.65).into()),
            border: iced::Border {
                ..Default::default()
            },
            ..Default::default()
        })
}
