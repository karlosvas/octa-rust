use crate::message::states::{AppState, Buttons};
use crate::styles::custom_style;
use crate::widgets::partiture;
use iced::{
    Element, Length,
    alignment::{Horizontal, Vertical},
    widget::{Button, Container, Image, Text, column, image::Handle},
};

//  Estructura de la aplicación
#[derive(Debug, Clone)]
pub struct MyApp {
    state: AppState,
}

// Implementar Default para MyApp
impl Default for MyApp {
    fn default() -> Self {
        Self {
            state: AppState::MainMenu,
        }
    }
}

// Implementación de la aplicación
impl MyApp {
    // Actualización del estado de la aplicación
    pub fn update(&mut self, message: Buttons) {
        match message {
            // Iniciar el juego
            Buttons::Play => self.state = AppState::Game,
            // Salir de la aplicación
            Buttons::Exit => std::process::exit(0),
        }
    }

    // Crear botones
    fn create_button(text: &str, message: Buttons) -> Button<'_, Buttons> {
        Button::new(
            Container::new(Text::new(text))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center),
        )
        .on_press(message)
        .width(Length::Fixed(400.0))
        .height(Length::Fixed(50.0))
    }

    // Crear imagenes
    fn create_image(path: &str, heigth: f32, width: f32) -> Image {
        Image::new(Handle::from_path(path))
            .width(Length::Fixed(width))
            .height(Length::Fixed(heigth))
    }

    // Menú de la applicación
    fn main_menu_view(&self) -> Element<Buttons> {
        let img_octa_rust = Self::create_image("assets/octarust.png", 400.0, 400.0);
        let button_play = Self::create_button("Play", Buttons::Play);
        let button_exit = Self::create_button("Exit", Buttons::Exit);
        let main_column = column![img_octa_rust, button_play, button_exit].spacing(20);
        Container::new(main_column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .style(custom_style::background)
            .into()
    }

    // Menú del juego
    fn game_view(&self) -> Element<Buttons> {
        let partiture_l = partiture::Partiture::create_partiture();
        let partiture_r = partiture::Partiture::create_partiture();
        let game_column = column![partiture_l, partiture_r].spacing(10);
        Container::new(game_column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .style(custom_style::background)
            .into()
    }

    // Vista de la aplicación
    pub fn view(&self) -> Element<Buttons> {
        match self.state {
            AppState::MainMenu => self.main_menu_view(),
            AppState::Game => self.game_view(),
            AppState::Settings => self.main_menu_view(),
        }
    }
}
