use {
    crate::{
        message::states::{
            AppMessage, AppState, GameMessage, MainMenuMessage, SelectionMessage, SettingsMessage,
        },
        models::settings::CustomSettings,
        utils::{self, helper_json},
        views::{
            game::game_view,
            main::main_menu_view,
            selection::select_partiture_view,
            settings::{paused_view, settings_view},
        },
        widgets::{notes::Note, partiture::Partiture},
    },
    iced::{
        Element, Event, Subscription,
        event::listen,
        keyboard::{self, Key},
        time::every,
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

// Ruta a assets
#[macro_export]
macro_rules! asset_path {
    ($filename:expr) => {
        format!("{}/assets/{}", env!("CARGO_MANIFEST_DIR"), $filename)
    };
}

//  Estructura de la aplicación
pub struct MyApp {
    state: AppState,                      // Estado de la app
    start_time: Option<Instant>,          // Momento de inicio de la partitura
    actual_time: Option<Instant>,         // Tiempo actual de la partitura
    paused_elapsed: Option<f32>,          // Tiempo pausado
    settings: CustomSettings,             // Ajustes
    finished: Arc<AtomicBool>,            // Fin de la partitura
    partiture_name: Option<&'static str>, // Partitura selecionada
    partiture_r_selected: Partiture,      // Partitura derecha
    partiture_l_selected: Partiture,      // Partitura izquierda
}

// Implementar Default para MyApp
impl Default for MyApp {
    fn default() -> Self {
        Self {
            state: AppState::MainMenu,
            start_time: None,
            actual_time: None,
            paused_elapsed: None,
            settings: MyApp::load_settings(),
            finished: Arc::new(AtomicBool::new(false)),
            partiture_name: None,
            partiture_r_selected: Partiture::default(),
            partiture_l_selected: Partiture::default(),
        }
    }
}

// Implementación de la aplicación
impl MyApp {
    // Método para crear una nueva instancia de MyApp
    pub fn update(&mut self, message: AppMessage) {
        match message {
            // Eventos de teclado en el juego
            AppMessage::Event(msg) => match msg {
                Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => match key {
                    Key::Named(keyboard::key::Named::Escape)
                    | Key::Named(keyboard::key::Named::Space) => {
                        if self.state == AppState::Paused {
                            self.resume_game();
                        } else if self.state == AppState::Game {
                            self.pause_game();
                        }
                    }
                    _ => {}
                },
                _ => {}
            },

            // Manejar mensajes del menu
            AppMessage::MainMenu(msg) => match msg {
                MainMenuMessage::SelectPartiture => {
                    self.state = AppState::SlectionPartiture;
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
                    if self.state == AppState::Game {
                        self.actual_time = Some(instant);
                        let elapsed: f32 = self
                            .actual_time
                            .and_then(|current| {
                                self.start_time
                                    .map(|start| (current.duration_since(start)).as_secs_f32())
                            })
                            .unwrap_or(0.0);

                        let max_duration = self
                            .partiture_l_selected
                            .time
                            .max(self.partiture_r_selected.time);
                        if elapsed > max_duration + (self.settings.timer * 2.0) {
                            // Si el tiempo transcurrido es mayor que la duración máxima, finalizar el juego
                            self.finished.store(true, Ordering::SeqCst);
                            self.state = AppState::Paused;
                        }

                        utils::utils::create_tempo_overlay(
                            &mut self.partiture_l_selected.notes,
                            elapsed,
                        );
                        utils::utils::create_tempo_overlay(
                            &mut self.partiture_r_selected.notes,
                            elapsed,
                        );
                    }
                }

                GameMessage::RestartGame => {
                    let now: Instant = Instant::now();
                    self.actual_time = Some(now);
                    self.start_time = Some(now);
                    self.paused_elapsed = None;
                    self.state = AppState::Game;
                    self.finished.store(false, Ordering::SeqCst);
                }

                GameMessage::ResumeGame => {
                    self.resume_game();
                }
            },

            // Manejar mensajes de configuración
            AppMessage::Settings(msg) => match msg {
                SettingsMessage::ChangeTheme(val) => {
                    self.settings.theme = val;
                    let _ = self.save_settings();
                }
                SettingsMessage::ChangeDifficulty(val) => {
                    self.settings.difficulty = val;
                    self.settings.timer = 3.0;
                    let _ = self.save_settings();
                }
                SettingsMessage::BackToMenu => {
                    self.state = AppState::MainMenu;
                }
            },

            // Manejar mensajes de selección de partitura
            AppMessage::Selection(msg) => match msg {
                // Manejar selección de partitura
                SelectionMessage::StartGame(name) => {
                    let now: Instant = Instant::now();
                    self.partiture_name = Some(name);
                    self.actual_time = Some(now);
                    self.start_time = Some(now);
                    self.paused_elapsed = None;
                    self.finished.store(false, Ordering::SeqCst);
                    let arr: Vec<Value> =
                        helper_json::load_partiture(&asset_path!("notes.json")).unwrap_or_default();

                    // Cargar notas de la partitura seleccionada
                    let mut notes_l: Vec<Note> =
                        helper_json::load_notes_from_file(&arr, name, "left").unwrap_or_default();
                    let mut note_r: Vec<Note> =
                        helper_json::load_notes_from_file(&arr, name, "right").unwrap_or_default();

                    // Sanitizar notas con los datos necesarios y asignar los datos a las partituras
                    helper_json::sanitize_data(
                        &mut self.partiture_l_selected,
                        &mut self.partiture_r_selected,
                        &mut notes_l,
                        &mut note_r,
                    );

                    // Cambiamos el estado a el juego
                    self.state = AppState::Game;
                }
                SelectionMessage::BackToMenu => {
                    self.state = AppState::MainMenu;
                }
            },
        }
    }

    // Método para crear la vista de la aplicación
    pub fn view(&self) -> Element<AppMessage> {
        match self.state {
            AppState::MainMenu => main_menu_view(&self.settings),
            AppState::SlectionPartiture => select_partiture_view(&self.settings),
            AppState::Game => game_view(
                self.start_time,
                self.actual_time,
                &self.settings,
                self.partiture_name,
                &self.partiture_r_selected,
                &self.partiture_l_selected,
            ),
            AppState::Settings => settings_view(&self.settings),
            AppState::Paused => paused_view(self.finished.clone(), &self.settings),
        }
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

    // Método para cargar y guardar la configuración
    pub fn load_settings() -> CustomSettings {
        let path: String = asset_path!("settings.json");
        fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    // Método para guardar la configuración
    pub fn save_settings(&self) -> Result<(), Box<dyn error::Error>> {
        let path: String = asset_path!("settings.json");
        let json: String = serde_json::to_string_pretty(&self.settings)?;
        fs::write(path, json)?;
        Ok(())
    }

    // Métodos auxiliares para manejar pausa/reanudación
    pub fn pause_game(&mut self) {
        // Calcular tiempo transcurrido hasta ahora
        let elapsed = self
            .actual_time
            .and_then(|current| {
                self.start_time
                    .map(|start| current.duration_since(start).as_secs_f32())
            })
            .unwrap_or(0.0);

        self.paused_elapsed = Some(elapsed);
        self.state = AppState::Paused;
    }
    pub fn resume_game(&mut self) {
        // Ajustar start_time para que continue desde donde se pausó
        if let Some(paused_time) = self.paused_elapsed {
            let now = Instant::now();
            self.start_time = Some(now - Duration::from_secs_f32(paused_time));
            self.actual_time = Some(now);
            self.paused_elapsed = None;
        }
        self.state = AppState::Game;
    }
}
