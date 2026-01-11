use {
    crate::models::{note::Note, settings::CustomSettings},
    core::fmt,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Hand {
    Left,
    Right,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Hand::Left => write!(f, "left"),
            Hand::Right => write!(f, "right"),
        }
    }
}

// Estructura de la partitura
pub struct Partiture {
    pub notes: Vec<Note>,                // Notas de la partitura
    pub time: f32,                       // Tiempo total de la partitura
    pub elapsed: f32,                    // Tiempo de actual de la partitura
    pub settings: CustomSettings,        // Configuración de la partitura
    pub hand: Hand,                      // Mano utilizada (izquierda o derecha)
    pub metadata: Option<PieceMetadata>, // Metadata de la partitura selecionada
    pub img_width: f32,
}

// 1. METADATOS NECESARIOS DE LA OBRA (necesario para el cálculo)
#[derive(Clone)]
pub struct PieceMetadata {
    pub time_signature: (u8, u8), // Ej: (3, 8) para compás 3/8
    pub base_note_value: f32, // Duración en segundos de la unidad del compás (ej: 0.5s para corchea en 3/8)
}
