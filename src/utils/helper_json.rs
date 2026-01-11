use {
    crate::models::{
        note::Note,
        partiture::{Hand, Partiture, PieceMetadata},
    },
    serde_json::{Value, from_str, from_value},
    std::fs::read_to_string,
};

pub fn load_partiture(file_path: &str) -> Result<Vec<Value>, String> {
    if file_path.is_empty() {
        return Err("Ruta del archivo no puede estar vacía".into());
    }

    let json_str: String = match read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => return Err(format!("Error al leer el archivo: {}", e).into()),
    };

    let data: Value = match from_str(&json_str) {
        Ok(value) => value,
        Err(e) => return Err(format!("Error al parsear JSON: {}", e).into()),
    };

    // El JSON ahora es un array de objetos, cada uno con una pieza
    match data.as_array() {
        Some(arr) => Ok(arr.clone()),
        None => Err("El JSON no es un array de partituras".into()),
    }
}

// Cargar múltiples notas
pub fn load_notes_from_file(
    hand: &Hand, // Mano (izquierda o derecha)
    metadata: &Value,
    sections: &Value,
) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
    // Obtener la estructura desde metadata
    let structure = metadata
        .get("structure")
        .and_then(Value::as_array)
        .ok_or("Metadata/structure faltante o con formato inválido")?;

    let mut notes: Vec<Note> = Vec::new();
    for section in structure {
        // Pasamos a string la seccion actual
        let label: &str = section
            .as_str()
            .ok_or("Cada entrada de structure debe ser string")?;

        // Obtenemos la sección de notas y luego la sub-sección para la mano
        let section_selected: &Value = sections.get(label).ok_or("Sección no encontrada")?;
        let selection_by_hand: &Value = section_selected
            .get(hand.to_string())
            .ok_or("Sección para esta mano no encontrada")?;

        let mut section_notes: Vec<Note> = from_value(selection_by_hand.clone())?;
        notes.append(&mut section_notes);
    }

    Ok(notes)
}

// Obtiene la metadata y seccion de los datos de la partitura
pub fn get_metadata_and_section(
    file_notes: &[Value],
    piece_name: &str,
) -> Result<(Value, Value), Box<dyn std::error::Error>> {
    for piece_obj in file_notes {
        // Obtenemos la metadata de la pieza
        if let Some(piece_data) = piece_obj.get(piece_name) {
            // Obtener y clonar metadata
            let metadata: Value = piece_data
                .get("metadata")
                .ok_or("No se encontró 'metadata'")?
                .clone();

            // Obtener y clonar sections
            let sections: Value = piece_data
                .get("sections")
                .ok_or("No se encontraron 'sections'")?
                .clone();

            return Ok((metadata, sections));
        }
    }

    Err(format!("No se pudo obtener metadata/sections de '{}'", piece_name).into())
}

// Obtiene el PieceMetadata de los metadatos
pub fn get_price_metdata_compas(
    metadata: Value,
) -> Result<PieceMetadata, Box<dyn std::error::Error>> {
    // Obtener duración base de la unidad de nota (f32)
    let base_note_value: f32 = metadata
        .get("note_duration_unit_seconds")
        .and_then(|v| v.as_f64().map(|f| f as f32))
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error al obtener la duración de la unidad de nota",
            )
        })?;

    // Obtener el compás como string, por ejemplo "3/8"
    let meter_str: &str = metadata
        .get("meter")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Error al obtener el meter")
        })?;

    // Pasar el string de meter de "3/8" a (u8, u8)
    let parts: Vec<&str> = meter_str.split('/').collect();
    if parts.len() != 2 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Meter inválido: '{}'", meter_str),
        )
        .into());
    }
    let num: u8 = parts[0].parse::<u8>().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Numerador inválido: {}", e),
        )
    })?;
    let den: u8 = parts[1].parse::<u8>().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Denominador inválido: {}", e),
        )
    })?;

    Ok(PieceMetadata {
        time_signature: (num, den),
        base_note_value,
    })
}

// En basae a la informacion añadimos todo lo necesairo que devemos cambiar mediante el código,
// por ejemplo cuando se juntan las notas
pub fn sanitize_data(partiture: &mut Partiture) {
    let beat_duration: f32 = partiture
        .metadata
        .as_ref()
        .map(|m| m.base_note_value)
        .unwrap_or(0.0);

    let mut current_beat_time: f32 = 0.0;
    let mut group_start_idx: Option<usize> = None;

    let notes_len: usize = partiture.notes.len();

    for i in 0..notes_len {
        let duration: f32 = partiture.notes[i].duration;
        let is_rest: bool = partiture.notes[i].is_rest;

        if duration <= beat_duration && !is_rest {
            if group_start_idx.is_none() {
                group_start_idx = Some(i);
            }

            current_beat_time += duration;

            if current_beat_time >= beat_duration {
                if let Some(start) = group_start_idx {
                    for j in start..=i {
                        partiture.notes[j].joined = true;
                    }
                }
                current_beat_time = 0.0;
                group_start_idx = None;
            }
        } else {
            current_beat_time = 0.0;
            group_start_idx = None;
        }
    }
}
