use {
    crate::models::{note::Note, partiture::Partiture},
    iced::{Point, Rectangle, widget::canvas::Frame},
};

// Estructura de overlay para mostrar todas las notas y compas de la partitura
pub struct AllNotesOverlay<'a> {
    pub partiture: &'a Partiture, // Referencia a la partitura
}

impl<'a> AllNotesOverlay<'a> {
    // Método principal para dibujar todas las notas en el overlay
    pub fn draw(&self, frame: &mut Frame, layout_bounds: Rectangle) {
        let mut last_position: Point = Point::default();

        for note in self.partiture.notes.iter() {
            self.draw_note_in_overlay(note, frame, layout_bounds, &mut last_position);
        }
    }

    // Método para dibujar una nota en el overlay
    fn draw_note_in_overlay(
        &self,
        note: &Note,
        frame: &mut Frame,
        layout_bounds: Rectangle,
        last_position: &mut Point,
    ) {
        let work_area = Rectangle {
            x: layout_bounds.x + self.partiture.img_width,
            y: layout_bounds.y,
            width: layout_bounds.width - self.partiture.img_width,
            height: layout_bounds.height,
        };

        // Tiempo actual ajustado por el timer inicial
        let current_time: f32 = self.partiture.elapsed - self.partiture.settings.timer;
        let pixels_per_second: f32 = self.partiture.calculate_pixels_per_second();

        let note_end_time: f32 = note.start + note.duration;

        // Calcular posición X basada en el tiempo de inicio
        let note_x = work_area.x + (note.start - current_time) * pixels_per_second;

        // Culling: solo dibujar notas visibles en pantalla
        if note_x < work_area.x - 50.0 || note_x > work_area.x + work_area.width + 50.0 {
            return;
        }

        let is_currently_active = current_time >= note.start && current_time < note_end_time;

        let note_y = self.calculate_note_y_in_staff(note, &layout_bounds);
        let mut actual_position = Point::new(note_x, note_y);

        let new_note: Note = Note {
            last_position: *last_position,
            is_active: is_currently_active,
            ..note.clone()
        };

        new_note.draw(frame, actual_position.clone());
        Self::draw_plicas(&new_note, &mut actual_position);

        *last_position = Point::new(actual_position.x + 8.0, actual_position.y);
    }
    fn draw_plicas(new_note: &Note, actual_position: &mut Point) {
        // Plicas
        if new_note.pitch < 54 {
            // Mano izquierda - plica hacia arriba
            actual_position.y -= 25.0;
        } else if new_note.pitch < 60 {
            // Mano izquierda - plica hacia abajo
            actual_position.y += 5.0;
            actual_position.x -= 10.0;
        } else if new_note.pitch <= 71 {
            // Mano derecha - plica hacia arriba
            actual_position.y -= 25.0;
        } else {
            // Mano derecha - plica hacia abajo
            actual_position.y += 29.0; // Ajustar la posición hacia abajo
            actual_position.x -= 10.0; // Ajustar la posición horizontal
        }
    }

    // Método para calcular la posición Y de la nota en el pentagrama
    fn calculate_note_y_in_staff(&self, note: &Note, staff_area: &Rectangle) -> f32 {
        let line_spacing: f32 = staff_area.height / 6.0;

        // Cada nota tiene una posición en el pentagrama según su nombre y octava
        let pitch: u8 = note.pitch;
        let staff_y_offset: f32 = staff_area.y + staff_area.height;

        // Calcular cuántos "pasos" está por encima del Do4 (MIDI 60)
        let steps_from_c4: f32 = match pitch {
            60 => 0.8, // C4
            61 => 0.8, // C#4
            62 => 1.6, // D4
            63 => 1.6,
            64 => 2.0, // E4
            65 => 3.0, // F4
            66 => 3.0,
            67 => 4.0, // G4
            68 => 4.0,
            69 => 5.0, // A4
            70 => 5.0,
            71 => 6.0, // B4
            72 => 7.0, // C5
            73 => 7.0,
            74 => 8.0,
            75 => 8.0,
            76 => 9.1,
            77 => 10.1,
            78 => 10.3,
            79 => 10.5,
            80 => 11.0,
            81 => 11.0,
            82 => 11.5,
            83 => 11.5,
            _ => 0.0,
        };

        // Cada paso son medio espacio de pentagrama (línea o espacio)
        staff_y_offset - (steps_from_c4 as f32 * (line_spacing / 2.0)) - 5.0
    }
}
