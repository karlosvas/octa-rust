use {
    iced::{Color, Point},
    serde::{Deserialize, Serialize},
};

// Notas, con implementacion para el trait Overlay y Serialize/Deserialize con serde_json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub start: f32,    // Tiempo de inicio en segundos
    pub pitch: u8,     // Número MIDI del tono
    pub duration: f32, // Duración en segundos
    #[serde(skip)]
    pub is_active: bool, // Si la nota está activa (sonando)
    #[serde(skip)]
    pub joined: bool, // Si la nota está unida a otra
    #[serde(skip)]
    pub last_position: Point, // Nota anterior
    #[serde(skip)]
    pub is_rest: bool,
}

pub struct PaletteColors {
    pub primary: Color,
    pub secondary: Color,
}
