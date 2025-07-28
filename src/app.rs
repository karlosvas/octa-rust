use crate::{
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
};
use iced::{Element, Event, Point, Subscription, event::listen, keyboard, time::every};
use serde_json::Value;
use std::{
    error, fs,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
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
    state: AppState,
    pub selected_partiture: Option<String>,
    pub actual_time: Option<Instant>,
    pub start_time: Option<Instant>,
    pub settings: CustomSettings,
    pub finished: Arc<AtomicBool>,
    pub partiture_r_selected: Partiture,
    pub partiture_l_selected: Partiture,
}

// Implementar Default para MyApp
impl Default for MyApp {
    fn default() -> Self {
        Self {
            state: AppState::MainMenu,
            selected_partiture: None,
            actual_time: None,
            start_time: None,
            settings: MyApp::load_settings(),
            finished: Arc::new(AtomicBool::new(false)),
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
            AppMessage::Event(event) => {
                // Para la partitura derecha
                if let Some(msg) = Self::detected_active(&event, &self.partiture_r_selected) {
                    self.update(msg);
                }

                // Para la partitura izquierda
                if let Some(msg) = Self::detected_active(&event, &self.partiture_l_selected) {
                    self.update(msg);
                }
            }

            // Manejar mensajes del menu
            AppMessage::MainMenu(msg) => match msg {
                MainMenuMessage::SelectPartiture => {
                    self.state = AppState::SlectionPartiture;
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
            // Manejar mensajes del juego
            AppMessage::Game(msg) => match msg {
                GameMessage::Tick(instant) => {
                    self.actual_time = Some(instant);
                    let mut elapsed: f32 = self
                        .actual_time
                        .and_then(|current| {
                            self.start_time
                                .map(|start| current.duration_since(start).as_secs_f32())
                        })
                        .unwrap_or(0.0);

                    elapsed -= self.settings.timer * 2.0;

                    utils::utils::create_tempo_overlay(
                        &mut self.partiture_l_selected.notes,
                        elapsed,
                    );
                    utils::utils::create_tempo_overlay(
                        &mut self.partiture_r_selected.notes,
                        elapsed,
                    );
                }
                GameMessage::RestartGame => {
                    let now: Instant = Instant::now();
                    self.actual_time = Some(now);
                    self.start_time = Some(now);
                    self.state = AppState::Game;
                    self.finished.store(false, Ordering::SeqCst);
                }
                GameMessage::PauseGame => {
                    self.state = AppState::Paused;
                }
                GameMessage::ResumeGame => {
                    self.state = AppState::Game;
                }
                GameMessage::Finished => {
                    self.finished.store(true, Ordering::SeqCst);
                    self.state = AppState::Paused;
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
                SelectionMessage::StartGame(string) => {
                    let now: Instant = Instant::now();
                    self.selected_partiture = Some(string.clone());
                    self.actual_time = Some(now);
                    self.start_time = Some(now);
                    self.finished.store(false, Ordering::SeqCst);
                    let arr: Vec<Value> =
                        helper_json::load_partiture(&asset_path!("notes.json")).unwrap_or_default();

                    // Cargar notas de la partitura seleccionada
                    let mut notes_l: Vec<Note> =
                        helper_json::load_notes_from_file(&arr, string.as_str(), "left")
                            .unwrap_or_default();
                    let mut note_r: Vec<Note> =
                        helper_json::load_notes_from_file(&arr, string.as_str(), "right")
                            .unwrap_or_default();

                    // Sanitizar notas con los datos necesarios
                    let (notes_l_sanitized, notes_r_sanitized) =
                        helper_json::sanitize_notes(&mut notes_l, &mut note_r);
                    self.partiture_l_selected.notes = notes_l_sanitized;
                    self.partiture_r_selected.notes = notes_r_sanitized;

                    // Cambiamos el estado a el juego
                    self.state = AppState::Game;
                }
                SelectionMessage::BackToMenu => {
                    self.state = AppState::MainMenu;
                }
            },
        }
    }

    fn detected_active(event: &Event, partiture: &Partiture) -> Option<AppMessage> {
        if partiture.elapsed > partiture.time {
            Some(AppMessage::Game(GameMessage::Finished))
        } else {
            match event {
                Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => match key {
                    keyboard::Key::Named(keyboard::key::Named::Escape) => {
                        Some(AppMessage::Game(GameMessage::PauseGame))
                    }
                    keyboard::Key::Named(keyboard::key::Named::Space) => {
                        Some(AppMessage::Game(GameMessage::PauseGame))
                    }
                    _ => None,
                },
                _ => None,
            }
        }
    }

    // Método para crear la vista de la aplicación
    pub fn view(&self) -> Element<AppMessage> {
        match self.state {
            AppState::MainMenu => main_menu_view(&self.settings),
            AppState::SlectionPartiture => select_partiture_view(&self.settings),
            AppState::Game => game_view(
                self.selected_partiture.as_deref(),
                self.start_time,
                self.actual_time,
                &self.settings,
                &self.partiture_l_selected,
                &self.partiture_r_selected,
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
}
