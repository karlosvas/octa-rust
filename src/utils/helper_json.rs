use {
    crate::widgets::{notes::Note, partiture::Partiture},
    serde_json::{Value, from_str, from_value},
    std::fs::read_to_string,
};

pub fn load_partiture(file_path: &str) -> Result<Vec<Value>, String> {
    if file_path.is_empty() {
        return Err("❌ Ruta del archivo no puede estar vacía".into());
    }

    let json_str: String = match read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => return Err(format!("❌ Error al leer el archivo: {}", e).into()),
    };

    let data: Value = match from_str(&json_str) {
        Ok(value) => value,
        Err(e) => return Err(format!("❌ Error al parsear JSON: {}", e).into()),
    };

    // El JSON ahora es un array de objetos, cada uno con una pieza
    match data.as_array() {
        Some(arr) => Ok(arr.clone()),
        None => Err("❌ El JSON no es un array de partituras".into()),
    }
}

// Cargar múltiples notas
pub fn load_notes_from_file(
    arr: &Vec<Value>, // Array de partituras
    piece_name: &str, // Nombre de la pieza musical
    hand: &str,       // Mano (izquierda o derecha)
) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
    // Buscar la pieza por nombre
    for piece_obj in arr {
        if let Some(piece_data) = piece_obj.get(piece_name) {
            if let Some(notes_array) = piece_data.get(hand) {
                let notes: Vec<Note> = from_value(notes_array.clone())?;
                return Ok(notes);
            } else {
                return Err(format!(
                    "❌ Mano '{}' no encontrada en la pieza '{}'",
                    hand, piece_name
                )
                .into());
            }
        }
    }
    Err(format!("❌ Pieza '{}' no encontrada en el archivo", piece_name).into())
}

pub fn sanitize_data(
    partiture_l: &mut Partiture,
    partiture_r: &mut Partiture,
    notes_l: &mut Vec<Note>,
    notes_r: &mut Vec<Note>,
) {
    let mut calculate_join: f32 = 0.0;
    let mut joined: bool = false;

    for i in 0..notes_l.len() {
        partiture_l.time += notes_l[i].duration;
        if notes_l[i].duration <= 0.5 {
            calculate_join += notes_l[i].duration;
            if calculate_join >= 1.0 {
                calculate_join = 0.0;
                joined = true;
            }
        } else {
            calculate_join = 0.0;
            joined = false;
        }

        if joined {
            notes_l[i].joined = true;
            if i > 0 {
                notes_l[i - 1].joined = true;
            }
        }
    }
    partiture_l.notes = notes_l.clone();

    for i in 0..notes_r.len() {
        partiture_r.time += notes_r[i].duration;
        if notes_r[i].duration <= 0.5 {
            calculate_join += notes_r[i].duration;
            if calculate_join >= 1.0 {
                calculate_join = 0.0;
                joined = true;
            }
        } else {
            calculate_join = 0.0;
            joined = false;
        }

        if joined {
            notes_r[i].joined = true;
            if i > 0 {
                notes_r[i - 1].joined = true;
            }
        }
    }
    partiture_r.notes = notes_r.clone();
}
