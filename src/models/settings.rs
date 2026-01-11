use iced::Theme;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum CustomTheme {
    Light,
    Dark,
}

impl CustomTheme {
    pub fn to_iced_theme(&self) -> Theme {
        match self {
            CustomTheme::Light => Theme::Light,
            CustomTheme::Dark => Theme::Dark,
        }
    }

    pub fn from_iced_theme(theme: &Theme) -> Self {
        match theme {
            Theme::Light => CustomTheme::Light,
            Theme::Dark => CustomTheme::Dark,
            _ => CustomTheme::Dark,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CustomSettings {
    pub theme: CustomTheme, // Tema actual (serializable)
    pub timer: f32,         // Tiempo que dura la partitura
}

impl CustomSettings {
    pub fn get_iced_theme(&self) -> Theme {
        self.theme.to_iced_theme()
    }
}

impl Default for CustomSettings {
    fn default() -> Self {
        Self {
            theme: CustomTheme::Dark,
            timer: 3.0,
        }
    }
}
