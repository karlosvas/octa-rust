#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum Difficulty {
    Easy,   // 0.5
    Normal, // 1.0
    Hard,   // 1.5
}

impl Difficulty {
    // Método para obtener el multiplicador de dificultad
    pub fn get_multiplier(&self) -> f32 {
        match self {
            Difficulty::Easy => 0.5,
            Difficulty::Normal => 1.0,
            Difficulty::Hard => 1.5,
        }
    }

    pub fn get_dificulty_from_f32(val: f32) -> Difficulty {
        match val {
            0.5 => Difficulty::Easy,
            1.0 => Difficulty::Normal,
            1.5 => Difficulty::Hard,
            _ => Difficulty::Normal,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CustomTheme {
    Light,
    Dark,
}

impl CustomTheme {
    pub fn get_theme(&self) -> i32 {
        match self {
            CustomTheme::Light => 1,
            CustomTheme::Dark => 0,
        }
    }

    pub fn get_theme_from_int(val: i32) -> CustomTheme {
        match val {
            1 => CustomTheme::Light,
            0 => CustomTheme::Dark,
            _ => CustomTheme::Light,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct CustomSettings {
    pub theme: CustomTheme,     // Thema actual
    pub difficulty: Difficulty, // Dificultad
    pub timer: f32,             // Tiempo que dura la partitura
}

impl Default for CustomSettings {
    fn default() -> Self {
        Self {
            theme: CustomTheme::Dark,
            difficulty: Difficulty::Normal,
            timer: 3.0,
        }
    }
}
