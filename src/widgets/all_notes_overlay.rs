use crate::widgets::{notes::Note, partiture::Partiture};
use iced::{Point, Rectangle, widget::canvas::Frame};

// Estructura de overlay para mostrar todas las notas y compas de la partitura
pub struct AllNotesOverlay<'a> {
    pub partiture: &'a Partiture, // Referencia a la partitura
}

impl<'a> AllNotesOverlay<'a> {
    pub fn draw(&self, frame: &mut Frame, layout_bounds: Rectangle) {
        if self.partiture.elapsed < self.partiture.settings.timer {
            return;
        }

        let mut curret_time: f32 = 0.0;
        let mut last_position: Point = Point::default();

        for note in self.partiture.notes.iter() {
            self.draw_note_in_overlay(
                note,
                frame,
                layout_bounds,
                &mut curret_time,
                &mut last_position,
            );
        }
    }
}

// Implementación de métodos específicos para dibujar notas en el overlay
impl<'a> AllNotesOverlay<'a> {
    // Método para dibujar una nota en el overlay
    fn draw_note_in_overlay(
        &self,
        note: &Note,
        frame: &mut Frame,
        layout_bounds: Rectangle,
        curret_time: &mut f32,
        last_position: &mut Point,
    ) {
        // Calcular el área disponible para las notas (con padding)
        let work_area: Rectangle = Rectangle {
            x: layout_bounds.x + 120.0, // Padding izquierdo
            y: layout_bounds.y,
            width: layout_bounds.width - 120.0,
            height: layout_bounds.height,
        };

        // Progreso de la nota basado en el tiempo actual
        let init_counter: f32 = self.partiture.settings.timer + 3.0;

        let progress: f32 =
            (note.start - (self.partiture.elapsed - (init_counter))) / self.partiture.time;
        // Calcular posición X basada en el tiempo actual y el inicio de la nota
        // La nota se mueve de izquierda a derecha según el tiempo actual
        let note_x: f32 = work_area.x + progress * work_area.width;

        // Calculamos cuando dibujar el compás
        *curret_time += note.duration;

        // Solo dibujar la nota si está dentro del área visible
        if note_x < work_area.x || note_x > (work_area.x + work_area.width) {
            if *curret_time > 4.0 {
                *curret_time = 0.0;
            }
            return;
        } else {
            if *curret_time > 4.0 {
                Partiture::draw_compas(frame, work_area, note_x);
                *curret_time = 0.0;
            }
        }

        // Calculamos la posición en y
        let note_y: f32 =
            self.calculate_note_y_in_staff(note, &layout_bounds, &self.partiture.hand.clone());

        // Obtenemos la posicion actual de la nota
        let mut actual_position: Point = Point::new(note_x, note_y);
        // Creamos la instancia de la nota que vamos a crear
        let new_note: Note = Note {
            last_position: *last_position,
            ..note.clone()
        };

        // Dibujamos la nota creada en el layout
        new_note.draw(frame, actual_position);

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

        let actual_position: Point = Point::new(actual_position.x + 8.0, actual_position.y);
        // Obtenemos el punto actual y se lo asignamos a el último punto para tener la referencia del anterior a el
        *last_position = actual_position;
    }

    // Método para calcular la posición Y de la nota en el pentagrama
    fn calculate_note_y_in_staff(&self, note: &Note, staff_area: &Rectangle, hand: &str) -> f32 {
        if hand == "right" {
            let line_spacing: f32 = staff_area.height / 6.0;

            // Cada nota tiene una posición en el pentagrama según su nombre y octava
            let pitch: u8 = note.pitch;
            let staff_y_offset: f32 = staff_area.y + staff_area.height;

            // Calcular cuántos "pasos" está por encima del Do4 (MIDI 60)
            let steps_from_c4: f32 = match pitch {
                60 => 0.8, // C4
                61 => 0.8, // C#4 → misma línea que C4
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
        } else {
            let line_spacing: f32 = staff_area.height / 6.0;
            let pitch: u8 = note.pitch;
            let staff_y_offset: f32 = staff_area.y + staff_area.height;

            // Calcular cuántos pasos está por encima de Fa2 (MIDI 41)
            let steps_from_f2: f32 = match pitch {
                36 => -5.0, // C2
                37 => -5.0,
                38 => -4.5,
                39 => -4.5,
                40 => -1.0, // E2
                41 => 0.0,  // F2 (línea central en clave de fa)
                42 => 0.0,
                43 => 1.0, // G2
                44 => 1.0,
                45 => 2.0, // A2
                46 => 2.0,
                47 => 3.0, // B2
                48 => 4.0, // C3
                49 => 4.0,
                50 => 5.0,
                51 => 5.0,
                52 => 6.0,
                53 => 6.0,
                54 => 7.0,
                55 => 7.0,
                56 => 8.0,
                57 => 8.0,
                58 => 9.0,
                59 => 9.0,
                60 => 10.0, // C4
                _ => 0.0,
            };

            // Igual que antes: medio espacio por paso
            staff_y_offset - (steps_from_f2 * (line_spacing / 2.0)) - 5.0
        }
    }
}
