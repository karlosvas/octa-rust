use crate::utils::helper_json::point_serde;
use iced::{
    Color, Event, Point, Rectangle, Size,
    advanced::{
        Clipboard, Layout, Renderer as RenderTrait, Shell,
        layout::Node,
        overlay::{self, Overlay},
        renderer,
        widget::Operation,
    },
    event::Status,
    mouse::{Cursor, Interaction},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, from_str, from_value};
use std::{error, fs::read_to_string};

// Notas, con implementacion para el trait Overlay y Serialize/Deserialize con serde_json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub name: String, // Nombre de la nota (ej. "C4", "G#5")
    #[serde(with = "point_serde")]
    pub position: Point, // Posición en el pentagrama
    pub start: f32,   // Tiempo de inicio en segundos
    pub pitch: u8,    // Número MIDI del tono
    pub duration: f32, // Duración en segundos
    pub is_active: bool, // Si la nota está activa (sonando)
}

// Constructor para la nota musical
impl Note {
    // pub fn new(name: String, position: Point, pitch: u8, duration: f32) -> Self {
    //     Self {
    //         name,
    //         position,
    //         start: 0.0,
    //         pitch,
    //         duration,
    //         is_active: false,
    //     }
    // }

    // Cargar múltiples notas
    pub fn load_notes_from_file(
        file_path: &str,  // Ruta del archivo JSON
        piece_name: &str, // Nombre de la pieza musical
        hand: &str,       // Mano (izquierda o derecha)
    ) -> Result<Vec<Note>, Box<dyn error::Error>> {
        if file_path.is_empty() {
            return Err("❌ Ruta del archivo no puede estar vacía".into());
        }

        let json_str = match read_to_string(file_path) {
            Ok(content) => content,
            Err(e) => return Err(format!("❌ Error al leer el archivo: {}", e).into()),
        };

        let data: Value = match from_str(&json_str) {
            Ok(value) => value,
            Err(e) => return Err(format!("❌ Error al parsear JSON: {}", e).into()),
        };

        // Buscar la pieza específica
        if let Some(piece_data) = data.get(piece_name) {
            if let Some(notes_array) = piece_data.get(hand) {
                let notes: Vec<Note> = from_value(notes_array.clone())?;
                println!(
                    "✅ Cargadas {} notas para '{}' (mano: {})",
                    notes.len(),
                    piece_name,
                    hand
                );
                Ok(notes)
            } else {
                Err(format!(
                    "❌ Mano '{}' no encontrada en la pieza '{}'",
                    hand, piece_name
                )
                .into())
            }
        } else {
            Err(format!("❌ Pieza '{}' no encontrada en el archivo", piece_name).into())
        }
    }
}

// Implementación del trait de Overlay para la nota musical
impl<Message, Theme, Renderer> Overlay<Message, Theme, Renderer> for Note
where
    Renderer: RenderTrait,
{
    // Define el tamaño y la posición del elemento
    fn layout(&mut self, _renderer: &Renderer, _bounds: Size) -> Node {
        // Calcular tamaño basado en la duración de la nota
        let size = Size::new(20.0 * self.duration, 20.0);
        // Crear un nodo en la posición de la nota
        Node::with_children(
            size,
            vec![], // Sin hijos
        )
        .move_to(iced::Point::new(self.position.x, self.position.y))
    }

    // Dibuja la nota musical en el renderer
    fn draw(
        &self,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: Cursor,
    ) {
        let bounds = layout.bounds();

        // Configurar el color negro para la nota
        let black = Color::BLACK;

        // 1. Dibujar la cabeza de la nota (óvalo negro)
        let note_head_center = Point::new(
            bounds.x + 10.0,                // Un poco hacia la derecha del inicio
            bounds.y + bounds.height / 2.0, // Centro vertical
        );

        // Crear óvalo para la cabeza de la nota
        let note_head = Rectangle {
            x: note_head_center.x - 6.0,
            y: note_head_center.y - 4.0,
            width: 12.0,
            height: 8.0,
        };

        // Dibujar la cabeza de la nota (óvalo relleno)
        renderer.fill_quad(
            iced::advanced::renderer::Quad {
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

        // 2. Dibujar la plica (línea vertical)
        let stem_start = Point::new(
            note_head_center.x + 6.0, // Desde el borde derecho de la cabeza
            note_head_center.y,
        );
        let stem_end = Point::new(
            stem_start.x,
            stem_start.y - 25.0, // Línea hacia arriba
        );

        // Dibujar la plica
        let stem_rect = Rectangle {
            x: stem_start.x - 1.0,
            y: stem_end.y,
            width: 2.0,
            height: 25.0,
        };
        renderer.fill_quad(
            iced::advanced::renderer::Quad {
                bounds: stem_rect,
                border: iced::Border {
                    color: black,
                    width: 0.0,
                    radius: 4.0.into(),
                },
                shadow: Default::default(),
            },
            black,
        );
    }

    // Evento para manejar interacciones, clics, drag & drop, etc.
    fn on_event(
        &mut self,
        _event: Event,
        _layout: Layout<'_>,
        _cursor: Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        _shell: &mut Shell<'_, Message>,
    ) -> Status {
        Status::Ignored
    }

    // Evento para manejar notas con el cursor arrastradolas o editandolas.
    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> Interaction {
        if cursor.is_over(layout.bounds()) {
            Interaction::Pointer
        } else {
            Interaction::default()
        }
    }

    // Evento para manejar el cursor
    fn operate(
        &mut self,
        _layout: Layout<'_>,
        _renderer: &Renderer,
        _operation: &mut dyn Operation,
    ) {
        // Implementación vacía si no necesitas operaciones
    }

    // Verifica si el cursor está sobre el elemento
    fn is_over(&self, layout: Layout<'_>, _renderer: &Renderer, cursor_position: Point) -> bool {
        layout.bounds().contains(cursor_position)
    }

    // Método para manejar el overlay
    fn overlay<'a>(
        &'a mut self,
        _layout: Layout<'_>,
        _renderer: &Renderer,
    ) -> Option<overlay::Element<'a, Message, Theme, Renderer>> {
        None
    }
}
