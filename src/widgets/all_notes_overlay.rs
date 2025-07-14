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
                partiture_time: self.partiture.time.clone()
                    + 5.0 * self.partiture.settings.difficulty.get_multiplier(),
            })))
        } else {
            None
        }
    }
}

// Implementación de métodos específicos para dibujar notas en el overlay
impl<'a> AllNotesOverlay<'a> {
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
        let work_area = Rectangle {
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

        let note_y: f32 = self.calculate_note_y_in_staff(note, &layout_bounds);

        // Ejemplo de cómo crear un layout personalizado para dibujar una nota
        let custom_node =
            Node::new(Size::new(20.0, 20.0)).move_to(iced::Point::new(note_x, note_y)); // x, y: posición deseada

        let custom_layout = Layout::new(&custom_node);

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

    // Convierte el pitch de la nota a su nombre natural
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

    // Calcula la posición Y de la nota en el pentagrama
    fn calculate_note_y_in_staff(&self, note: &Note, staff_area: &Rectangle) -> f32 {
        let line_spacing = staff_area.height / 6.0;

        let note_pos: f32 = match Self::note_name_from_pitch(note.pitch) {
            'C' => 5.30,
            'D' => 5.10,
            'E' => 4.80,
            'F' => 4.30,
            'G' => 3.80,
            'A' => 3.30,
            'B' => 2.80,
            _ => 1.5,
        };

        staff_area.y + note_pos * line_spacing
    }
}
