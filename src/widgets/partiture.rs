use crate::message::states::AppMessage;
use crate::utils::frecuency::get_frecuency;
use crate::widgets::notes::Note;
use crate::{models::settings::CustomSettings, widgets::all_notes_overlay::AllNotesOverlay};
use iced::advanced::{Clipboard, Shell};
use iced::event::Status;
use iced::{
    Border, Color, Length, Point, Rectangle, Size, Vector,
    advanced::{
        self, Layout, Widget, layout::Node, overlay, renderer::Quad, renderer::Style, widget::Tree,
    },
};
use iced::{Event, mouse};

// Estructura de la partitura
pub struct Partiture {
    pub notes: Vec<Note>,         // Notas de la partitura
    pub time: f32,                // Tiempo total de la partitura
    pub elapsed: f32,             // Tiempo de actual de la partitura
    pub settings: CustomSettings, // Configuración de la partitura
}

impl Default for Partiture {
    fn default() -> Self {
        Partiture {
            notes: Vec::new(),
            time: 0.0,
            elapsed: 0.0,
            settings: CustomSettings::default(),
        }
    }
}

// Declare the lifetime parameter for the impl block
impl Partiture {
    // Constructor para crear una partitura con notas predefinidas
    pub fn new(notes: Vec<Note>, time: f32, elapsed: f32, settings: CustomSettings) -> Self {
        Self {
            notes,
            time,
            elapsed: elapsed * settings.difficulty.get_multiplier(),
            settings,
        }
    }

    // Dibujar el fondo de la partitura
    fn draw_partiture_background(
        &self,
        renderer: &mut impl iced::advanced::Renderer,
        bounds: iced::Rectangle,
    ) {
        // Dibujar fondo
        renderer.fill_quad(
            Quad {
                bounds: bounds,
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
        // Calcular espaciado entre líneas
        let line_spacing = (bounds.height - (5.0 * 2.0)) / 6.0; // 5 líneas, 4 espacios
        let line_height = 2.0;

        // Dibujar 5 líneas del pentagrama
        for i in 0..=5 {
            let y: f32 = bounds.y + (i as f32 * (line_height + line_spacing));

            let line_rect = Rectangle {
                x: bounds.x,
                y: y,
                width: bounds.width,
                height: line_height,
            };

            // Dibujar línea
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
                iced::Color::BLACK,
            );
        }
    }

    // Dibujar compás en una posición específica
    pub fn draw_compas(
        renderer: &mut impl iced::advanced::Renderer,
        layout_bounds: iced::Rectangle,
        note_x: f32,
    ) {
        let width_percentage = 0.025; // 2.5% del ancho total (ajustable)
        let offset = layout_bounds.width * width_percentage; // Si width=800, offset=20
        let line_rect = Rectangle {
            x: note_x - offset / 2.0, // Ajustar el offset para centrar la línea
            y: layout_bounds.y,       // Ajustar el Y para que esté en el pentagrama
            width: 2.0,
            height: layout_bounds.height, // Altura del compás (ajustable)
        };

        renderer.fill_quad(
            Quad {
                bounds: line_rect,
                border: Border {
                    color: Color::BLACK,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                shadow: Default::default(),
            },
            Color::BLACK,
        );
    }
}

impl<Theme, Renderer> Widget<AppMessage, Theme, Renderer> for Partiture
where
    Renderer: advanced::Renderer + advanced::text::Renderer,
    Theme: Clone + Default,
{
    // Aquí definimos cómo se ve el widget
    fn size(&self) -> Size<Length> {
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

        // // Aplica el padding aquí
        let padded_size: Size = Size::new(size.width - 40.0, size.height - 40.0);
        let mut node: Node = Node::new(padded_size);

        // Posiciona el nodo con el offset de padding
        node = node.move_to(Point::new(20.0, 20.0));

        node
    }

    // Aquí definimos cómo se dibuja el widget
    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &Style,
        layout: Layout<'_>,
        _cursor: iced::mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        // Dibujar el fondo de la partitura (equivalente al container)
        self.draw_partiture_background(renderer, layout.bounds());
        // Dibujar las 5 líneas del pentagrama (equivalente al column con rows)
        self.draw_staff_lines(renderer, layout.bounds());
    }

    // Aquí definimos cómo se manejan los eventos del widget
    fn on_event(
        &mut self,
        _tree: &mut Tree,
        _event: Event,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        _shell: &mut Shell<'_, AppMessage>,
        _viewport: &iced::Rectangle,
    ) -> Status {
        Status::Ignored
    }

    // Elementos flotantes o superpuestos
    fn overlay(
        &mut self,
        _tree: &mut Tree,
        layout: Layout<'_>,
        _renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<AppMessage, Theme, Renderer>> {
        if !self.notes.is_empty() {
            // Extraer bounds del layout con offsets personalizados
            Some(overlay::Element::new(Box::new(AllNotesOverlay {
                partiture_bounds: layout.bounds(),                // Solo Rectangle
                offset: Point::new(translation.x, translation.y), // Translation + padding
                partiture: self, // Referencia al tiempo de la partitura
            })))
        } else {
            None
        }
    }
}
