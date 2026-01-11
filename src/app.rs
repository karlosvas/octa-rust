use {
    crate::{
        message::states::{
            AppMessage, AppState, GameMessage, MainMenuMessage, SelectionMessage, SettingsMessage,
        },
        models::{
            note::Note,
            partiture::{Hand, Partiture, PieceMetadata},
            settings::CustomSettings,
        },
        utils::helper_json::{
            get_metadata_and_section, get_price_metdata_compas, load_notes_from_file,
            load_partiture, sanitize_data,
        },
        views::{
            game::game_view,
            menu::main_menu_view,
            selection::select_partiture_view,
            settings::{paused_view, settings_view},
        },
    },
    iced::{
        Element, Event, Length, Subscription, Theme,
        event::listen,
        keyboard::{self, Key},
        time::every,
        widget::{Container, Text},
    },
    serde_json::Value,
    std::{
        error, fs,
        process::exit,
        sync::{
            Arc,
            atomic::{AtomicBool, Ordering},
        },
        time::{Duration, Instant},
    },
};

/// Macro para la ruta de assets
#[macro_export]
macro_rules! asset_path {
    ($filename:expr) => {
        format!("{}/assets/{}", env!("CARGO_MANIFEST_DIR"), $filename)
    };
}

/// Estructura de la aplicación
pub struct MyApp {
    state: AppState,                                    // Estado de la app
    start_time: Option<Instant>,                        // Momento de inicio de la partitura
    actual_time: Option<Instant>,                       // Tiempo actual de la partitura
    pause_started: Option<Instant>,                     // Momento en que se pausó
    is_paused: Arc<AtomicBool>,                         // Tiempo pausado
    settings: CustomSettings,                           // Ajustes
    finished: Arc<AtomicBool>,                          // Fin de la partitura
    partiture_name: Option<&'static str>,               // Partitura selecionada
    partiture_selected: Option<(Partiture, Partiture)>, // Partitura derecha, izquierda
}

/// Implementar Default para MyApp
impl Default for MyApp {
    fn default() -> Self {
        Self {
            state: AppState::MainMenu,
            start_time: None,
            actual_time: None,
            pause_started: None,
            is_paused: Arc::new(AtomicBool::new(false)),
            settings: MyApp::load_settings(),
            finished: Arc::new(AtomicBool::new(false)),
            partiture_name: None,
            partiture_selected: None,
        }
    }
}

/// Implementación de la aplicación octarust
impl MyApp {
    // Método para manejar los mensajes de la aplicación
    pub fn update(&mut self, message: AppMessage) {
        match message {
            // Eventos de teclado en el juego
            AppMessage::Event(msg) => match msg {
                Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => match key {
                    Key::Named(keyboard::key::Named::Escape)
                    | Key::Named(keyboard::key::Named::Space) => {
                        // Si no estaba pausado lo pausamos, y si estaba pausado lo despausamos
                        self.is_paused.fetch_not(Ordering::SeqCst);

                        if self.is_paused.load(Ordering::SeqCst) {
                            // Pausando: guardar el momento actual
                            self.pause_started = Some(Instant::now());
                            self.state = AppState::Paused;
                        } else {
                            self.resume_game()
                        }
                    }
                    _ => {}
                },
                _ => {}
            },

            // Manejar mensajes del menu
            AppMessage::MainMenu(msg) => match msg {
                // Seleccionar partitura
                MainMenuMessage::SelectPartiture => {
                    self.state = AppState::SelectionPartiture;
                }
                // Salir de la aplicación
                MainMenuMessage::Exit => {
                    exit(0);
                }
                // Abrir configuración
                MainMenuMessage::OpenSettings => {
                    self.state = AppState::Settings;
                }
            },

            // Manejar mensajes del juego
            AppMessage::Game(msg) => match msg {
                GameMessage::Tick(instant) => {
                    // Tiempo transcurrido desde que inició la applicacion
                    let elapsed: f32 = self
                        .start_time
                        .map(|start| instant.duration_since(start).as_secs_f32())
                        .unwrap_or(0.0);

                    // Esperamos a que la partitura tenga un valor
                    if let Some((ref mut left_partiture, ref mut right_partiture)) =
                        self.partiture_selected
                    {
                        // Si el tiempo transcurrido es mayor que la duración máxima + el timer del inicio y del final, finalizar el juego
                        if elapsed > (right_partiture.time + (self.settings.timer * 2.0)) {
                            self.finished.store(true, Ordering::SeqCst);
                            self.state = AppState::Paused;
                        }

                        // Actualizamos el tiempo elapsed
                        right_partiture.elapsed = elapsed;
                        left_partiture.elapsed = elapsed;
                    }
                }
                GameMessage::RestartGame => {
                    if let Some(name) = self.partiture_name {
                        self.start_game_with_partiture(name)
                    }
                }
                GameMessage::ResumeGame => self.resume_game(),
            },

            // Manejar mensajes de configuración
            AppMessage::Settings(msg) => match msg {
                SettingsMessage::ChangeTheme(val) => {
                    // Convert Theme to CustomTheme for storage
                    use crate::models::settings::CustomTheme;
                    self.settings.theme = match val {
                        Theme::Light => CustomTheme::Light,
                        Theme::Dark => CustomTheme::Dark,
                        _ => CustomTheme::Dark, // Default to Dark for other themes
                    };
                    self.save_settings().unwrap_or_else(|e| {
                        log::error!("{}", e);
                        return;
                    });
                }
                SettingsMessage::BackToMenu => {
                    self.state = AppState::MainMenu;
                }
            },

            // Manejar mensajes de selección de partitura
            AppMessage::Selection(msg) => match msg {
                // Manejar selección de partitura
                SelectionMessage::StartGame(name) => self.start_game_with_partiture(name),
                SelectionMessage::BackToMenu => self.state = AppState::MainMenu,
            },
        }
    }

    // Método para crear la vista de la aplicación, dependiendo del estado de la aplicación, mostrar la vista correspondiente, empezamos con el menú principal
    pub fn view(&self) -> Element<'_, AppMessage> {
        match self.state {
            AppState::MainMenu => main_menu_view(),
            AppState::SelectionPartiture => select_partiture_view(),
            AppState::Game => match &self.partiture_selected {
                Some((left, right)) => game_view((left, right), &self.settings),
                _ => Container::new(Text::new("Cargando partitura..."))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into(),
            },
            AppState::Settings => settings_view(&self.settings.get_iced_theme()),
            AppState::Paused => paused_view(self.finished.clone()),
        }
    }

    // Método para obtener el tema actual de la aplicación
    pub fn theme(&self) -> iced::Theme {
        self.settings.get_iced_theme()
    }

    // Método para manejar las suscripciones de la aplicación
    pub fn subscription(&self) -> Subscription<AppMessage> {
        match self.state {
            AppState::Game => Subscription::batch(vec![
                every(Duration::from_millis(16))
                    .map(|instant| AppMessage::Game(GameMessage::Tick(instant))),
                listen().map(AppMessage::Event),
            ]),
            AppState::Paused => {
                // Solo escuchar eventos de teclado, sin tick de tiempo
                listen().map(AppMessage::Event)
            }
            _ => Subscription::none(),
        }
    }

    // Cargar y guardar la configuración
    fn load_settings() -> CustomSettings {
        let path: String = asset_path!("settings.json");
        fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    // Guardar la configuración
    fn save_settings(&self) -> Result<(), Box<dyn error::Error>> {
        let path: String = asset_path!("settings.json");
        let json: String = serde_json::to_string_pretty(&self.settings)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Reanudar el juego
    fn resume_game(&mut self) {
        if let (Some(pause_start), Some(start)) = (self.pause_started, self.start_time) {
            let pause_duration = Instant::now().duration_since(pause_start);
            self.start_time = Some(start + pause_duration);
        }
        self.pause_started = None;
        self.state = AppState::Game;
    }

    /// Empezar juego con partitura
    fn start_game_with_partiture(&mut self, name: &'static str) {
        // Iniciamos los tiempos
        let now: Instant = Instant::now();
        self.partiture_name = Some(name);
        self.actual_time = Some(now);
        self.start_time = Some(now);
        self.pause_started = None;

        // Le decimos que no a terminado y que no esta pausado
        self.finished.store(false, Ordering::SeqCst);
        self.is_paused.store(false, Ordering::SeqCst);

        // Array con todas las notas
        let file_notes: Vec<Value> = match load_partiture(&asset_path!("partitures.json")) {
            Ok(notes) => notes,
            Err(e) => {
                log::error!("Partituras no encontradas -> {}", e);
                return;
            }
        };

        // Obtenemos la metadata y la secciones
        let (metadata, sections) = match get_metadata_and_section(&file_notes, name) {
            Ok((m, s)) => (m, s),
            Err(e) => {
                log::error!("No se pudo obtener metadata/sections: {}", e);
                return;
            }
        };

        // Cargar notas de la partitura seleccionada
        let notes_l: Vec<Note> = match load_notes_from_file(&Hand::Left, &metadata, &sections) {
            Ok(notes) => notes,
            Err(e) => {
                log::error!("Error cargando notas mano derecha -> {}", e);
                return;
            }
        };
        let notes_r: Vec<Note> = match load_notes_from_file(&Hand::Right, &metadata, &sections) {
            Ok(notes) => notes,
            Err(e) => {
                log::error!("Error cargando notas mano derecha -> {}", e);
                return;
            }
        };

        //  Calcular duración total antes de crear Partiture (usar máximo, no el último elemento)
        let duration_left: f32 = notes_l
            .iter()
            .map(|n| n.start + n.duration)
            .fold(0.0, f32::max);
        let duration_right: f32 = notes_r
            .iter()
            .map(|n| n.start + n.duration)
            .fold(0.0, f32::max);
        let total_duration: f32 = duration_left.max(duration_right);

        // Crear las partituras con las notas cargadas
        let mut partiture_l: Partiture = Partiture {
            notes: notes_l,
            time: total_duration,
            elapsed: 0.0,
            settings: self.settings.clone(),
            hand: Hand::Left,
            metadata: None,
            img_width: 200.0,
        };
        let mut partiture_r: Partiture = Partiture {
            notes: notes_r,
            time: total_duration,
            elapsed: 0.0,
            settings: self.settings.clone(),
            hand: Hand::Right,
            metadata: None,
            img_width: 200.0,
        };

        let fur_elise_meta: PieceMetadata = match get_price_metdata_compas(metadata) {
            Ok(data) => data,
            Err(e) => {
                log::error!("Error obteniendo metadata del compás: {}", e);
                return;
            }
        };

        // Damos los valores de metadata a las partituras para que lo tengan en cuenta a ala hora de dibujar compases velocidad etc
        (partiture_r.metadata, partiture_l.metadata) =
            (Some(fur_elise_meta.clone()), Some(fur_elise_meta));

        // Sanitizar notas con los datos necesarios y asignar los datos a las partituras
        sanitize_data(&mut partiture_r);
        sanitize_data(&mut partiture_l);

        // Actualizamos con la nueva partitura
        self.partiture_selected = Some((partiture_l, partiture_r));

        // Cambiamos el estado a el juego
        self.state = AppState::Game;
    }
}
