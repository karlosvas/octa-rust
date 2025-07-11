use crate::message::states::{AppState, Buttons};
use crate::styles::custom_style;
use crate::utils::{reusable, utils};
use crate::widgets::notes::Note;
use crate::widgets::partiture::Partiture;
use iced::{
    Element, Length,
    alignment::{Horizontal, Vertical},
    widget::{Container, column},
};

//  Estructura de la aplicación
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

    // Menú de la applicación
    fn main_menu_view(&self) -> Element<Buttons> {
        let img_octa_rust = reusable::create_image(
            &format!("{}/assets/octarust.png", env!("CARGO_MANIFEST_DIR")),
            400.0,
            400.0,
        );
        let button_play = reusable::create_button("Play", Buttons::Play);
        let button_exit = reusable::create_button("Exit", Buttons::Exit);

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
        // Crear instancias de partituras con notas
        let mut partiture_l = Partiture::new("for-elise".to_string(), Vec::new(), 0.0);
        let mut partiture_r = Partiture::new("for-elise".to_string(), Vec::new(), 0.0);

        // Cargar notas desde el archivo JSON
        let notes_l: Vec<Note> = Note::load_notes_from_file(
            &format!("{}/notes.json", env!("CARGO_MANIFEST_DIR")),
            &partiture_l.name,
            "left",
        )
        .unwrap();
        let notes_r: Vec<Note> = Note::load_notes_from_file(
            &format!("{}/notes.json", env!("CARGO_MANIFEST_DIR")),
            &partiture_r.name,
            "right",
        )
        .unwrap();

        partiture_l.time = notes_l.iter().map(|n| n.duration).sum();
        partiture_r.time = notes_r.iter().map(|n| n.duration).sum();

        // Añadir notas a las partituras
        for note in notes_l.into_iter() {
            partiture_l.add_note(note);
        }
        for note in notes_r.into_iter() {
            partiture_r.add_note(note);
        }

        // Crear elementos de partitura para la vista
        let mut partiture_r_overlay = Element::new(partiture_r);
        let mut partiture_l_overlay = Element::new(partiture_l);

        // Crear el gran pentagrama para ambas partituras
        utils::create_grand_staff(&mut partiture_r_overlay, &mut partiture_l_overlay);

        // Crear la columna principal del juego
        let game_column = column![
            partiture_r_overlay, // Parte derecha de la partitura
            partiture_l_overlay, // Parte izquierda de la partitura
        ]
        .spacing(20);

        // Contenedor principal del juego
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
            // AppState::Settings => self.main_menu_view(),
        }
    }
}
