use crate::message::states::{AppMessage, AppState, GameMessage, MainMenuMessage, SettingsMessage};
use crate::styles::custom_style;
use crate::utils::{reusable, utils};
use crate::widgets::notes::Note;
use crate::widgets::partiture::Partiture;
use iced::Subscription;
use iced::time::every;
use iced::{
    Element, Length,
    alignment::{Horizontal, Vertical},
    widget::{Container, Space, column, row},
};
use std::sync::Arc;
use std::time::{Duration, Instant};

macro_rules! asset_path {
    ($filename:expr) => {
        format!("{}/assets/{}", env!("CARGO_MANIFEST_DIR"), $filename)
    };
}

//  Estructura de la aplicación
pub struct MyApp {
    state: AppState,
    pub actual_time: Option<Arc<Instant>>,
    pub start_time: Option<Arc<Instant>>,
}

// Implementar Default para MyApp
impl Default for MyApp {
    fn default() -> Self {
        Self {
            state: AppState::MainMenu,
            actual_time: None,
            start_time: None,
        }
    }
}

// Implementación de la aplicación
impl MyApp {
    // Método para crear una nueva instancia de MyApp
    pub fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::MainMenu(msg) => match msg {
                // Cambiar al estado de juego
                MainMenuMessage::Play => {
                    self.state = AppState::Game;
                    let now = Instant::now();
                    self.actual_time = Some(Arc::new(now));
                    self.start_time = Some(Arc::new(now));
                }
                // Salir de la aplicación
                MainMenuMessage::Exit => {
                    std::process::exit(0);
                }
                // Abrir configuración
                MainMenuMessage::OpenSettings => {
                    self.state = AppState::Settings;
                }
            },
            AppMessage::Game(msg) => match msg {
                GameMessage::Tick(instant) => {
                    self.actual_time = Some(Arc::new(instant));
                }
            },
            AppMessage::Settings(msg) => match msg {
                // SettingsMessage::ChangeTheme => {}
                SettingsMessage::BackToMenu => {
                    self.state = AppState::MainMenu;
                }
            },
        }
    }

    // Menú de la applicación
    fn main_menu_view(&self) -> Element<AppMessage> {
        // Crear imagen de OctaRust
        let img_octa_rust = reusable::create_image(&asset_path!("octarust.png"), 400.0, 400.0);

        // Crear imagen de configuración
        let img_settings = reusable::create_image(&asset_path!("settings.png"), 400.0, 400.0);

        // Crear botones para el menú principal
        let button_play = reusable::create_button(
            AppMessage::MainMenu(MainMenuMessage::Play),
            Some("Play"),
            None,
        );

        // Botón de salir y botón de configuración
        let button_exit = reusable::create_button(
            AppMessage::MainMenu(MainMenuMessage::Exit),
            Some("Exit"),
            None,
        );

        // Botón de configuración con imagen
        let button_settings = reusable::create_button(
            AppMessage::MainMenu(MainMenuMessage::OpenSettings),
            None,
            Some(img_settings),
        )
        .style(custom_style::button_settings)
        .width(Length::Fixed(50.0));

        // Crear fila para el botón de configuración
        let settings_row =
            row![Space::with_width(Length::Fixed(400.0)), button_settings].spacing(10);

        // Crear columna principal del menú
        let main_column =
            column![settings_row, img_octa_rust, button_play, button_exit,].spacing(20);
        Container::new(main_column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .style(custom_style::background)
            .into()
    }

    // Menú del juego
    fn game_view(&self) -> Element<AppMessage> {
        let elapsed = if let (Some(start), Some(current)) =
            (self.start_time.as_ref(), self.actual_time.as_ref())
        {
            (**current).duration_since(**start).as_secs_f32()
        } else {
            0.0
        };

        // Crear instancias de partituras con notas
        let mut partiture_l = Partiture::new("for-elise".to_string(), Vec::new(), 0.0, elapsed);
        let mut partiture_r = Partiture::new("for-elise".to_string(), Vec::new(), 0.0, elapsed);

        // Cargar notas del archivo JSON
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

        // Añadir notas a las partituras y actualizar datos de tiempo
        for note in &notes_l {
            partiture_l.add_note(note.clone());
            partiture_l.time += note.duration;
        }
        for note in &notes_r {
            partiture_r.add_note(note.clone());
            partiture_r.time += note.duration;
        }

        // Crear elementos de partitura para la vista junto a las notas
        let mut partiture_r_overlay = Element::new(partiture_r);
        let mut partiture_l_overlay = Element::new(partiture_l);

        // Crear imagen de el gran pentagrama, calve de sol y clave de fa para ambas partituras
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

    // Vista de configuración
    fn settings_view(&self) -> Element<AppMessage> {
        let exit = reusable::create_button(
            AppMessage::Settings(SettingsMessage::BackToMenu),
            Some("Back to Main Menu"),
            None,
        );
        Container::new(column![exit])
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .style(custom_style::background)
            .into()
    }

    // Método para crear la vista de la aplicación
    pub fn view(&self) -> Element<AppMessage> {
        // Quita 'pub' aquí
        match self.state {
            AppState::MainMenu => self.main_menu_view(),
            AppState::Game => self.game_view(),
            AppState::Settings => self.settings_view(),
        }
    }

    // Método para manejar las suscripciones de la aplicación
    pub fn subscription(&self) -> Subscription<AppMessage> {
        if let AppState::Game = self.state {
            every(Duration::from_millis(16))
                .map(|instant| AppMessage::Game(GameMessage::Tick(instant)))
        } else {
            Subscription::none()
        }
    }
}
