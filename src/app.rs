use iced::alignment::{Horizontal, Vertical};
use iced::{Element, Sandbox};
use iced::widget::{Container};
use iced::widget::column;
use iced::Length;

use crate::styles;
use crate::message::enums::Message;
use crate::message::structs;

// Struct principal de la aplicación
#[derive(Default, Debug, Clone, Copy)]
pub struct MyApp;

// Implementación de la aplicación
impl Sandbox for MyApp {
    type Message = Message;

    // Constructor de la aplicación
    fn new() -> Self { MyApp::default() }

    // Título de la ventana
    fn title(&self) -> String { String::from("OctaRust") }

    // Actualización del estado de la aplicación
    fn update(&mut self, message: Message) {
        match message {
        }
    }

    // Vista de la aplicación
    fn view(&self) -> Element<Message> {
        let partiture_l = structs::Partiture::create_partiture("left".to_string());
        let partiture_r = structs::Partiture::create_partiture("right".to_string());

        Container::new(
            // Main content
            column![
                partiture_r, partiture_l
            ].spacing(10)

            // Body
            ).width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .align_x(Horizontal::Center) // Centra horizontalmente
            .align_y(Vertical::Center)   // Centra verticalmente
            .style(styles::custom_style::background as fn(&iced::Theme) -> iced::widget::container::Appearance)
            .into()
    }
}
