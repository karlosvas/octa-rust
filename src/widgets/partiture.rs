use {
    crate::{
        models::partiture::{Partiture, PieceMetadata},
        widgets::all_notes_overlay::AllNotesOverlay,
    },
    iced::{
        Color, Point, Rectangle, Renderer, Size, Theme,
        mouse::Cursor,
        widget::canvas::{Frame, Geometry, Path, Program},
    },
};

/// En este archivo se define la funcionalidad de la partitura musical. Se ejecuta automaticamente en
/// un canvas dentro de un contenedor.
/// Justo al crearlo en la vista del juego (game_view).
// Implementation for Partiture
impl Partiture {
    pub fn calculate_pixels_per_second(&self) -> f32 {
        let base_note_value: f32 = self
            .metadata
            .as_ref()
            .map(|m| m.base_note_value)
            .unwrap_or(0.5);

        const PIXELS_PER_BASE_NOTE: f32 = 50.0; // "Todas las corcheas ocupan 50px"

        PIXELS_PER_BASE_NOTE / base_note_value
    }

    // Dibujar las líneas del pentagrama
    fn draw_staff_lines(&self, frame: &mut Frame, bounds: iced::Rectangle) {
        let line_height: f32 = 2.0;
        let line_spacing: f32 = (bounds.height - (5.0 * line_height)) / 5.0;

        for i in 0..5 {
            let y: f32 = bounds.y + (i as f32 * (line_height + line_spacing));
            let linea: Path = Path::rectangle(
                Point::new(bounds.x, y),
                Size::new(bounds.width, line_height),
            );
            frame.fill(&linea, Color::BLACK);
        }
    }

    // Calcular duración de un compás completo
    fn calculate_bar_duration(metadata: &PieceMetadata) -> f32 {
        let (beats, _) = metadata.time_signature;
        beats as f32 * metadata.base_note_value
    }

    // Dibujar línea divisoria de compás
    // Dibujar todas las líneas de compás
    fn draw_bar_lines(&self, frame: &mut Frame, layout_bounds: iced::Rectangle) {
        let bar_duration: f32 = self
            .metadata
            .as_ref()
            .map(|metadata| Self::calculate_bar_duration(metadata))
            .unwrap_or(0.0);

        let pixels_per_second: f32 = self.calculate_pixels_per_second();
        let start_x: f32 = layout_bounds.x + self.img_width;

        let current_time: f32 = self.elapsed - self.settings.timer;
        let scroll_offset: f32 = current_time * pixels_per_second;

        let num_bars: usize = (self.time / bar_duration).ceil() as usize;

        for bar_index in 1..=num_bars {
            let bar_time: f32 = bar_index as f32 * bar_duration;
            let bar_absolute_x: f32 = bar_time * pixels_per_second;
            let x_pos: f32 = start_x + bar_absolute_x - scroll_offset;

            if x_pos >= start_x - 10.0 && x_pos <= start_x + layout_bounds.width + 10.0 {
                Self::draw_bar_line(frame, layout_bounds, x_pos);
            }
        }
    }

    // Dibujar UNA línea divisoria de compás en posición específica
    fn draw_bar_line(frame: &mut Frame, layout_bounds: iced::Rectangle, x_position: f32) {
        let line_rect: Rectangle = Rectangle {
            x: x_position - 1.0,
            y: layout_bounds.y,
            width: 2.0,
            height: layout_bounds.height,
        };

        let line_path: Path = Path::rectangle(line_rect.position(), line_rect.size());
        frame.fill(&line_path, Color::BLACK);
    }

    // Dibujar el fondo y estructura completa de la partitura
    fn draw_partiture(&self, frame: &mut Frame, relative_bounds: Rectangle) {
        let screen_size = frame.size();
        frame.fill(&Path::rectangle(Point::ORIGIN, screen_size), Color::WHITE);

        self.draw_staff_lines(frame, relative_bounds);
        self.draw_bar_lines(frame, relative_bounds);
    }
}

impl<AppMessage> Program<AppMessage> for Partiture {
    type State = ();

    // Dibujar el elemento al crearlo
    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        // Crear un frame para dibujar
        let mut frame: Frame<Renderer> = Frame::new(renderer, bounds.size());
        // Definir los límites relativos (0,0) al tamaño del canvas todo el ancho y alto
        let relative_bounds: Rectangle = Rectangle {
            x: 0.0,
            y: 0.0,
            width: bounds.width,
            height: bounds.height,
        };

        // Dibujar el fondo de la partitura
        self.draw_partiture(&mut frame, relative_bounds);

        // Dibuja todas las notas usando AllNotesOverlay
        let overlay: AllNotesOverlay = AllNotesOverlay { partiture: &self };
        overlay.draw(&mut frame, relative_bounds);

        // Retorna el frame como geometría
        vec![frame.into_geometry()]
    }
}
