use crate::widgets::notes::Note;
use iced::{
    Color, Point, Rectangle, Size,
    advanced::{
        Layout,
        layout::Node,
        overlay::{self},
        renderer::Quad,
    },
    event::Status,
    mouse::Interaction,
};

pub struct AllNotesOverlay<'a> {
    pub notes: &'a mut Vec<Note>,    // Notas de la partitura
    pub partiture_bounds: Rectangle, // Bounds de la partitura
    pub offset_x: f32,               // Offset horizontal personalizado
    pub offset_y: f32,               // Offset vertical personalizado
    pub partiture_time: f32,         // Referencia a la partitura
}

impl<'a, Message, Theme, Renderer> overlay::Overlay<Message, Theme, Renderer>
    for AllNotesOverlay<'a>
where
    Renderer: iced::advanced::Renderer,
{
    fn layout(&mut self, _renderer: &Renderer, _bounds: Size) -> Node {
        // Crear un nodo con tamaño y posición personalizada
        let mut node = Node::new(Size::new(
            self.partiture_bounds.width,
            self.partiture_bounds.height,
        ));

        // Aplicar offset personalizado si es necesario
        node = node.move_to(Point::new(
            self.partiture_bounds.x + self.offset_x,
            self.partiture_bounds.y + self.offset_y,
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
        for note in self.notes.iter() {
            self.draw_note_in_overlay(note, renderer, layout.bounds());
        }
    }

    fn on_event(
        &mut self,
        _event: iced::Event,
        _layout: Layout<'_>,
        _cursor: iced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        _shell: &mut iced::advanced::Shell<'_, Message>,
    ) -> Status {
        Status::Ignored
    }

    fn mouse_interaction(
        &self,
        _layout: Layout<'_>,
        _cursor: iced::mouse::Cursor,
        _viewport: &iced::Rectangle,
        _renderer: &Renderer,
    ) -> Interaction {
        iced::mouse::Interaction::default()
    }

    fn operate(
        &mut self,
        _layout: Layout<'_>,
        _renderer: &Renderer,
        _operation: &mut dyn iced::advanced::widget::Operation,
    ) {
    }

    fn is_over(
        &self,
        _layout: Layout<'_>,
        _renderer: &Renderer,
        _cursor_position: iced::Point,
    ) -> bool {
        false
    }

    fn overlay<'b>(
        &'b mut self,
        _layout: Layout<'_>,
        _renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        None
    }
}

impl<'a> AllNotesOverlay<'a> {
    fn draw_note_in_overlay(
        &self,
        note: &Note,
        renderer: &mut impl iced::advanced::Renderer,
        layout_bounds: Rectangle,
    ) {
        // Calcular el área disponible para las notas (con padding)
        let work_area = Rectangle {
            x: layout_bounds.x + 20.0,
            y: layout_bounds.y,
            width: layout_bounds.width - 40.0,
            height: layout_bounds.height - 40.0,
        };

        // ✅ Calcular posición X basada en el tiempo
        let time_ratio = if self.partiture_time > 0.0 {
            note.start / self.partiture_time // Proporción del tiempo transcurrido
        } else {
            0.0
        };

        let padding_left = 100.0;
        let note_x = work_area.x + (time_ratio * work_area.width) + padding_left;

        if note_x < work_area.x || (note_x + 25.0) > work_area.x + work_area.width {
            return;
        }

        let note_y = self.calculate_note_y_in_staff(note, &layout_bounds);

        self.draw_notes(renderer, note, note_x, note_y);
    }

    fn note_name_from_pitch(pitch: u8) -> char {
        // Notas naturales según su pitch mod 12
        match pitch % 12 {
            0 => 'C',
            2 => 'D',
            4 => 'E',
            5 => 'F',
            7 => 'G',
            9 => 'A',
            11 => 'B',
            _ => 'C', // para sostenidos/bemoles toma la nota natural inferior más cercana
        }
    }

    fn calculate_note_y_in_staff(&self, note: &Note, staff_area: &Rectangle) -> f32 {
        let line_spacing = staff_area.height / 6.0;

        let note_pos = match Self::note_name_from_pitch(note.pitch) {
            'C' => 4.90,
            'D' => 4.70,
            'E' => 4.50,
            'F' => 4.10,
            'G' => 3.70,
            'A' => 3.30,
            'B' => 2.90,
            _ => 1.5,
        };

        staff_area.y + note_pos * line_spacing
    }

    fn draw_notes(
        &self,
        renderer: &mut impl iced::advanced::Renderer,
        note: &Note,
        note_x: f32,
        note_y: f32,
    ) {
        // Dibujamos la nota
        // Actualizar posición de la nota
        let black = self.get_note_color(&note);

        // 1. Dibujar la cabeza de la nota
        let note_head = Rectangle {
            x: note_x - 5.0,
            y: note_y - 1.0,
            width: 15.0,
            height: 15.0,
        };

        // 2. Dibujar la plica (stem)
        let stem_rect = Rectangle {
            x: note_x + 5.0,
            y: note_y - 25.0,
            width: 5.0,
            height: 30.0,
        };
        renderer.fill_quad(
            Quad {
                bounds: note_head,
                border: iced::Border {
                    color: black,
                    width: 0.0,
                    radius: 4.0.into(),
                },
                shadow: Default::default(),
            },
            black,
        );

        renderer.fill_quad(
            Quad {
                bounds: stem_rect,
                border: iced::Border {
                    color: black,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                shadow: Default::default(),
            },
            black,
        );
    }

    fn get_note_color(&self, note: &Note) -> Color {
        if note.is_active {
            iced::Color::from_rgb(0.2, 0.8, 0.2) // Verde para notas activas
        } else {
            iced::Color::BLACK // Negro para notas normales
        }
    }

    // fn update_notes_based_on_partiture(&mut self) {
    //     if let Some(current_time) = self.get_current_playback_time() {
    //         for note in self.notes.iter_mut() {
    //             note.is_active =
    //                 note.start <= current_time && current_time <= note.start + note.duration;
    //         }
    //     }
    // }

    // fn get_current_playback_time(&self) -> Option<f32> {
    //     // ✅ Obtener tiempo actual de reproducción
    //     // Esto dependería de tu sistema de reproducción
    //     None // Placeholder
    // }
}
