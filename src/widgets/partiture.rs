use crate::widgets::all_notes_overlay::AllNotesOverlay;
use crate::widgets::notes::Note;
use iced::{
    Length, Rectangle, Size,
    advanced::{Layout, Widget, layout::Node, overlay, renderer::Quad, widget::Tree},
};

// Estructura de la partitura
pub struct Partiture {
    pub name: String,     // Nombre de la partitura
    pub notes: Vec<Note>, // Notas de la partitura
    pub time: f32,        // Tiempo total de la partitura
}

impl Partiture {
    // Constructor para crear una partitura con notas predefinidas
    pub fn new(name: String, notes: Vec<Note>, time: f32) -> Self {
        Self {
            name: name,
            notes: notes,
            time: time,
        }
    }

    // Añadir una nota a la partitura
    pub fn add_note(&mut self, note: Note) {
        self.notes.push(note);
    }

    // Dibujar el fondo de la partitura
    fn draw_partiture_background(
        &self,
        renderer: &mut impl iced::advanced::Renderer,
        bounds: iced::Rectangle,
    ) {
        // Aplicar padding (equivalente a .padding(20))
        let padded_bounds = Rectangle {
            x: bounds.x + 20.0,
            y: bounds.y + 20.0,
            width: bounds.width - 40.0,
            height: bounds.height - 40.0,
        };

        // Dibujar fondo (equivalente a .style(styles::custom_style::partiture))
        renderer.fill_quad(
            Quad {
                bounds: padded_bounds,
                border: iced::Border {
                    color: iced::Color::from_rgb(0.9, 0.9, 0.9), // Color de fondo
                    width: 1.0,
                    radius: 8.0.into(),
                },
                shadow: Default::default(),
            },
            iced::Color::from_rgb(0.98, 0.98, 0.98), // Fondo blanco cremoso
        );
    }

    // Dibujar las líneas del pentagrama
    fn draw_staff_lines(
        &self,
        renderer: &mut impl iced::advanced::Renderer,
        bounds: iced::Rectangle,
    ) {
        // Calcular área de trabajo (con padding)
        let work_area = Rectangle {
            x: bounds.x + 20.0,
            y: bounds.y + 20.0,
            width: bounds.width - 40.0,
            height: bounds.height - 40.0,
        };

        // Calcular espaciado entre líneas (equivalente a .spacing(20))
        let line_spacing = (work_area.height - (5.0 * 2.0)) / 6.0; // 5 líneas, 4 espacios
        let line_height = 2.0; // Height::Fixed(2.0)

        // ✅ Dibujar 5 líneas del pentagrama (equivalente al for loop)
        for i in 0..6 {
            let y = work_area.y + (i as f32 * (line_height + line_spacing));

            let line_rect = Rectangle {
                x: work_area.x,
                y: y,
                width: work_area.width,
                height: line_height,
            };

            // Dibujar línea (equivalente a .style(styles::custom_style::staff_line))
            renderer.fill_quad(
                iced::advanced::renderer::Quad {
                    bounds: line_rect,
                    border: iced::Border {
                        color: iced::Color::BLACK,
                        width: 0.0,
                        radius: 0.0.into(),
                    },
                    shadow: Default::default(),
                },
                iced::Color::BLACK, // Color de las líneas del pentagrama
            );
        }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Partiture
where
    Renderer: iced::advanced::Renderer,
    Theme: Clone,
{
    // Aquí definimos el tamaño del widget
    fn size(&self) -> iced::Size<Length> {
        Size::new(Length::Fill, Length::Fixed(400.0))
    }

    // Aquí definimos cómo se comporta el widget al recibir mensajes
    fn layout(
        &self,
        _tree: &mut iced::advanced::widget::Tree,
        _renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> Node {
        let size = limits.resolve(Length::Fill, Length::Fixed(200.0), Size::ZERO);
        Node::new(size)
    }

    // Aquí definimos cómo se dibuja el widget
    fn draw(
        &self,
        _tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: Layout<'_>,
        _cursor: iced::mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        // Dibujar el fondo de la partitura (equivalente al container)
        self.draw_partiture_background(renderer, layout.bounds());
        // Dibujar las 5 líneas del pentagrama (equivalente al column con rows)
        self.draw_staff_lines(renderer, layout.bounds());
    }

    // Elementos flotantes o superpuestos
    fn overlay<'a>(
        &'a mut self,
        _tree: &'a mut Tree,
        layout: Layout<'_>,
        _renderer: &Renderer,
        translation: iced::Vector,
    ) -> Option<overlay::Element<'a, Message, Theme, Renderer>> {
        if !self.notes.is_empty() {
            // Extraer bounds del layout con offsets personalizados
            Some(overlay::Element::new(Box::new(AllNotesOverlay {
                notes: &mut self.notes,            // Mutable reference
                partiture_bounds: layout.bounds(), // Solo Rectangle
                offset_x: translation.x,           // Translation + padding
                offset_y: translation.y,           // Translation + padding
                partiture_time: self.time,         // Referencia al tiempo de la partitura
            })))
        } else {
            None
        }
    }
}
