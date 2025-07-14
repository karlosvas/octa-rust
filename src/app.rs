use crate::message::states::{AppMessage, AppState, GameMessage, MainMenuMessage, SettingsMessage};
use crate::models::settings::CustomSettings;
use crate::views::{
    game::game_view,
    main::main_menu_view,
    settings::{paused_view, settings_view},
};
use iced::{Element, Subscription, time::every};
use std::{
    error, fs,
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
    pub actual_time: Option<Instant>,
    pub start_time: Option<Instant>,
    pub settings: CustomSettings,
}

// Implementar Default para MyApp
impl Default for MyApp {
    fn default() -> Self {
        Self {
            state: AppState::MainMenu,
            actual_time: None,
            start_time: None,
            settings: MyApp::load_settings(),
        }
    }
}

// Implementación de la aplicación
impl MyApp {
    // Método para crear una nueva instancia de MyApp
    pub fn update(&mut self, message: AppMessage) {
        match message {
            // Manejar mensajes del menu
            AppMessage::MainMenu(msg) => match msg {
                // Cambiar al estado de juego
                MainMenuMessage::Play => {
                    self.state = AppState::Game;
                    let now: Instant = Instant::now();
                    self.actual_time = Some(now);
                    self.start_time = Some(now);
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
                }
                GameMessage::RestartGame => {
                    let now: Instant = Instant::now();
                    self.actual_time = Some(now);
                    self.start_time = Some(now);
                    self.state = AppState::Game;
                }
                GameMessage::PauseGame => {
                    self.state = AppState::Paused;
                }
                GameMessage::ResumeGame => {
                    self.state = AppState::Game;
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
        }
    }

    // Método para crear la vista de la aplicación
    pub fn view(&self) -> Element<AppMessage> {
        match self.state {
            AppState::MainMenu => main_menu_view(&self.settings),
            AppState::Game => game_view(self.start_time, self.actual_time, &self.settings),
            AppState::Settings => settings_view(&self.settings),
            AppState::Paused => paused_view(&self.settings),
        }
    }

    // Método para manejar las suscripciones de la aplicación
    pub fn subscription(&self) -> Subscription<AppMessage> {
        match self.state {
            AppState::Game => every(Duration::from_millis(16))
                .map(|instant| AppMessage::Game(GameMessage::Tick(instant))),
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
