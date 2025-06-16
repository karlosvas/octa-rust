use crate::message::enums;
use crate::styles::custom_style;
use crate::widgets::partiture;
use iced::alignment::{Horizontal, Vertical};
use iced::{
    Element, Length,
    widget::{Container, column},
};

//  Estructura de la aplicación
#[derive(Default, Debug, Clone, Copy)]
pub struct MyApp;

// Implementación de la aplicación
impl MyApp {
    // Actualización del estado de la aplicación
    pub fn update(&mut self, message: enums::Message) {
        match message {}
    }

    // Vista de la aplicación
    pub fn view(&self) -> Element<enums::Message> {
        let partiture_l = partiture::Partiture::create_partiture();
        let partiture_r = partiture::Partiture::create_partiture();

        Container::new(
            // Main content
            column![partiture_r, partiture_l].spacing(10), // Body
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(50)
        .align_x(Horizontal::Center) // Centra horizontalmente
        .align_y(Vertical::Center) // Centra verticalmente
        .style(custom_style::background)
        .into()
    }
}
