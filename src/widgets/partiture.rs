use crate::{
    models::settings::CustomSettings,
    widgets::{all_notes_overlay::AllNotesOverlay, notes::Note},
};
use iced::{
    Color, Pixels, Point, Rectangle, Renderer, Size, Theme,
    mouse::Cursor,
    widget::canvas::{Frame, Geometry, Path, Program},
};

// Estructura de la partitura
pub struct Partiture {
    pub notes: Vec<Note>,         // Notas de la partitura
    pub time: f32,                // Tiempo total de la partitura
    pub elapsed: f32,             // Tiempo de actual de la partitura
    pub settings: CustomSettings, // Configuración de la partitura
    pub hand: String,             // Mano utilizada (izquierda o derecha)
}

impl Default for Partiture {
    fn default() -> Self {
        Partiture {
            notes: Vec::new(),
            time: 0.0,
            elapsed: 0.0,
            settings: CustomSettings::default(),
            hand: "right".to_string(),
        }
    }
}

// Declare the lifetime parameter for the impl block
impl Partiture {
    // Constructor para crear una partitura con notas predefinidas
    pub fn new(
        notes: Vec<Note>,
        time: f32,
        elapsed: f32,
        settings: CustomSettings,
        hand: String,
    ) -> Self {
        Self {
            notes,
            time,
            elapsed: elapsed * settings.difficulty.get_multiplier(),
            settings,
            hand,
        }
    }

    // Dibujar el fondo de la partitura
    fn draw_partiture_background(&self, frame: &mut Frame, bounds: Rectangle) {
        frame.fill(
            &Path::rectangle(bounds.position(), bounds.size()),
            Color::WHITE,
        );
    }

    // Dibujar las líneas del pentagrama
    fn draw_staff_lines(&self, frame: &mut Frame, bounds: iced::Rectangle) {
        // Cada línea tiene grosor 2.0, y hay 4 espacios entre ellas
        // Para distribuirlo bien, usamos 6 secciones (5 líneas generan 6 huecos entre ellas)
        let line_height: f32 = 2.0;
        let line_spacing: f32 = (bounds.height - (5.0 * line_height)) / 5.0;

        // Dibujar 5 líneas del pentagrama
        for i in 0..5 {
            let y: f32 = bounds.y + (i as f32 * (line_height + line_spacing));

            let linea: Path = Path::rectangle(
                Point::new(bounds.x, y),
                Size::new(bounds.width, line_height),
            );
            frame.fill(&linea, Color::BLACK);
        }
    }

    // Dibujar compás en una posición específica
    pub fn draw_compas(frame: &mut Frame, layout_bounds: iced::Rectangle, note_x: f32) {
        let width_percentage = 0.025; // 2.5% del ancho total (ajustable)
        let offset = layout_bounds.width * width_percentage; // Si width=800, offset=20
        let line_rect = Rectangle {
            x: note_x - offset / 2.0, // Ajustar el offset para centrar la línea
            y: layout_bounds.y,       // Ajustar el Y para que esté en el pentagrama
            width: 2.0,
            height: layout_bounds.height, // Altura del compás (ajustable)
        };

        let line_path: Path = Path::rectangle(line_rect.position(), line_rect.size());

        frame.fill(&line_path, Color::BLACK);
    }

    fn draw_intro_overlay(&self, frame: &mut Frame, bounds: Rectangle) {
        let elapsed: i32 = self.elapsed.floor() as i32;
        if elapsed <= 2 {
            // Dibuja el fondo semitransparente
            frame.fill(
                &Path::rectangle(bounds.position(), bounds.size()),
                Color::from_rgba(0.0, 0.0, 0.0, 0.5),
            );
            // Dibuja el número grande en el centro
            frame.fill_text(iced::widget::canvas::Text {
                content: (3 - elapsed).to_string(),
                position: Point::new(bounds.width / 2.0, bounds.height / 2.0),
                color: Color::WHITE,
                size: Pixels(120.0),
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
                ..Default::default()
            });
        }
    }
}

impl<AppMessage> Program<AppMessage> for Partiture {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame: Frame<Renderer> = Frame::new(renderer, bounds.size());
        let relative_bounds = Rectangle {
            x: 0.0,
            y: 0.0,
            width: bounds.width,
            height: bounds.height,
        };

        // Dibujar el fondo de la partitura (equivalente al container)
        self.draw_partiture_background(&mut frame, relative_bounds);
        // Dibujar las 5 líneas del pentagrama (equivalente al column con rows)
        self.draw_staff_lines(&mut frame, relative_bounds);

        // Dibuja todas las notas usando AllNotesOverlay
        let overlay: AllNotesOverlay<'_> = AllNotesOverlay { partiture: &self };
        overlay.draw(&mut frame, relative_bounds);

        self.draw_intro_overlay(&mut frame, relative_bounds);

        // Retorna el frame como geometría
        vec![frame.into_geometry()]
    }
}
