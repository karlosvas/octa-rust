use crate::widgets::{
    notes::Note, partiture::Partiture, temporized_intro_overlay::TemporizedIntroOverlay,
};
use iced::{
    Point, Rectangle, Size,
    advanced::{
        Layout, Overlay,
        layout::Node,
        overlay::{self},
        renderer::Style,
    },
    event::Status,
    mouse::{Cursor, Interaction},
};

pub struct AllNotesOverlay<'a> {
    pub notes: &'a mut Vec<Note>,    // Notas de la partitura
    pub partiture_bounds: Rectangle, // Bounds de la partitura
    pub offset_x: f32,               // Offset horizontal personalizado
    pub offset_y: f32,               // Offset vertical personalizado
    pub actual_time: f32,            // Tiempo actual de la partitura
}

impl<'a, Message, Theme, Renderer> overlay::Overlay<Message, Theme, Renderer>
    for AllNotesOverlay<'a>
where
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
    Theme: Clone + Default,
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
        let mut curret_time: f32 = 0.0;
        for note in self.notes.iter() {
            self.draw_note_in_overlay::<Message, Theme, Renderer>(
                note,
                renderer,
                layout.bounds(),
                &mut curret_time,
            );
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
        if !self.notes.is_empty() {
            // Extraer bounds del layout con offsets personalizados
            Some(overlay::Element::new(Box::new(TemporizedIntroOverlay {
                actual_time: self.actual_time.clone(),
            })))
        } else {
            None
        }
    }
}

// Implementación de métodos específicos para dibujar notas en el overlay
impl<'a> AllNotesOverlay<'a> {
    fn draw_note_in_overlay<Message, Theme, Renderer>(
        &self,
        note: &Note,
        renderer: &mut Renderer,
        layout_bounds: Rectangle,
        curret_time: &mut f32,
    ) where
        Renderer: iced::advanced::Renderer, // ← Bound necesario
        Theme: Clone + Default,
    {
        // Calcular el área disponible para las notas (con padding)
        let work_area = Rectangle {
            x: layout_bounds.x + 20.0,
            y: layout_bounds.y,
            width: layout_bounds.width - 40.0,
            height: layout_bounds.height - 40.0,
        };

        // TODO: Arreglar X de la nota
        // Calcular posición X basada en el tiempo actual y el inicio de la nota
        // La nota se mueve de izquierda a derecha según el tiempo actual
        let time_ratio = if self.actual_time > 0.0 {
            (self.actual_time - note.start) / note.duration.max(1.0)
        } else {
            0.0
        };

        // Limitar el ratio entre 0 y 1
        let time_ratio = time_ratio.clamp(0.0, 1.0);
        let note_x = work_area.x + (time_ratio * work_area.width);

        // Solo dibujar la nota si está dentro del área visible
        if note_x < work_area.x || (note_x + 25.0) > work_area.x + work_area.width {
            return;
        }

        let note_y = self.calculate_note_y_in_staff(note, &layout_bounds);

        // Ejemplo de cómo crear un layout personalizado para dibujar una nota
        let custom_node =
            Node::new(Size::new(20.0, 20.0)).move_to(iced::Point::new(note_x, note_y)); // x, y: posición deseada

        let custom_layout = Layout::new(&custom_node);

        // Ahora puedes llamar a draw con tu layout personalizado
        <Note as Overlay<Message, Theme, Renderer>>::draw(
            note,
            renderer,
            &Theme::default(),
            &Style::default(),
            custom_layout,
            Cursor::default(),
        );

        // Calculamos cuando dibujar el compás
        *curret_time += note.duration;
        if *curret_time > 4.0 {
            Partiture::draw_compas(renderer, work_area, note_x);
            *curret_time = 0.0;
        }
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
