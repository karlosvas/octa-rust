use crate::{
    message::states::AppMessage,
    widgets::{intro_and_pause_overlay::TemporizedIntroOverlay, notes::Note, partiture::Partiture},
};
use iced::{
    Point, Rectangle, Size,
    advanced::{
        self, Layout, Overlay,
        layout::Node,
        overlay::{self},
        renderer::Style,
    },
    mouse::Cursor,
};

// Estructura de overlay para mostrar todas las notas y compas de la partitura
pub struct AllNotesOverlay<'a> {
    pub partiture_bounds: Rectangle,  // Bounds de la partitura
    pub offset: Point,                // Offset horizontal personalizado
    pub partiture: &'a mut Partiture, // Referencia a la partitura
}

impl<'a, Theme, Renderer> Overlay<AppMessage, Theme, Renderer> for AllNotesOverlay<'a>
where
    Renderer: advanced::Renderer + advanced::text::Renderer,
    Theme: Clone + Default,
{
    fn layout(&mut self, _renderer: &Renderer, _bounds: Size) -> Node {
        // Crear un nodo con tamaño y posición personalizada
        let mut node: Node = Node::new(Size::new(
            self.partiture_bounds.width,
            self.partiture_bounds.height,
        ));

        // Aplicar offset personalizado si es necesario
        node = node.move_to(Point::new(
            self.partiture_bounds.x + self.offset.x,
            self.partiture_bounds.y + self.offset.y,
        ));

        node
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: Layout<'_>,
        _cursor: iced::mouse::Cursor,
    ) {
        if self.partiture.elapsed < self.partiture.settings.timer {
            return;
        }

        let mut curret_time: f32 = 0.0;
        for note in self.partiture.notes.iter() {
            self.draw_note_in_overlay::<AppMessage, Theme, Renderer>(
                note,
                renderer,
                layout.bounds(),
                &mut curret_time,
            );
        }
    }

    fn overlay<'b>(
        &'b mut self,
        _layout: Layout<'_>,
        _renderer: &Renderer,
    ) -> Option<overlay::Element<'b, AppMessage, Theme, Renderer>> {
        if !self.partiture.notes.is_empty() {
            Some(overlay::Element::new(Box::new(TemporizedIntroOverlay {
                elapsed: self.partiture.elapsed.clone(),
                partiture_time: self.partiture.time.clone() + self.partiture.settings.timer + 3.0, // Tiempo total de la partitura, mas intro mas 3 segundos de espera
            })))
        } else {
            None
        }
    }
}

// Implementación de métodos específicos para dibujar notas en el overlay
impl<'a> AllNotesOverlay<'a> {
    // Método para dibujar una nota en el overlay
    fn draw_note_in_overlay<AppMessage, Theme, Renderer>(
        &self,
        note: &Note,
        renderer: &mut Renderer,
        layout_bounds: Rectangle,
        curret_time: &mut f32,
    ) where
        Renderer: iced::advanced::Renderer,
        Theme: Clone + Default,
    {
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
                Partiture::draw_compas(renderer, work_area, note_x);
                *curret_time = 0.0;
            }
        }

        let note_y: f32 =
            self.calculate_note_y_in_staff(note, &layout_bounds, &self.partiture.hand.clone());

        // Ejemplo de cómo crear un layout personalizado para dibujar una nota
        let custom_node: Node =
            Node::new(Size::new(20.0, 20.0)).move_to(iced::Point::new(note_x, note_y)); // x, y: posición deseada

        let custom_layout: Layout<'_> = Layout::new(&custom_node);
        // Ahora puedes llamar a draw con tu layout personalizado
        <Note as Overlay<AppMessage, Theme, Renderer>>::draw(
            note,
            renderer,
            &Theme::default(),
            &Style::default(),
            custom_layout,
            Cursor::default(),
        );
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
