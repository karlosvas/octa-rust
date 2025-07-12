use crate::utils::helper_json::point_serde;
use iced::{
    Border, Color, Event, Point, Rectangle, Size,
    advanced::{
        Clipboard, Layout, Renderer as RenderTrait, Shell,
        layout::Node,
        overlay::{self, Overlay},
        renderer::{self, Quad},
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
    // Constructor para crear una nota con nombre, posición, tono y duración
    #[allow(dead_code)]
    pub fn new(name: String, position: Point, pitch: u8, duration: f32) -> Self {
        Self {
            name,
            position,
            start: 0.0,
            pitch,
            duration,
            is_active: false,
        }
    }

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

    // Dibujar redonda (4)
    fn draw_whole_note(&self, renderer: &mut impl RenderTrait, center: Point) {
        let (black, white) = self.get_note_colors();

        let note_head = Rectangle {
            x: center.x - 7.0,
            y: center.y - 7.0,
            width: 18.0,
            height: 18.0,
        };
        // Óvalo blanco con borde negro grueso
        renderer.fill_quad(
            Quad {
                bounds: note_head,
                border: iced::Border {
                    color: black,
                    width: 4.0,
                    radius: 9.0.into(),
                },
                shadow: Default::default(),
            },
            white,
        );
    }

    // Dibujar blanca (2)
    fn draw_half_note(&self, renderer: &mut impl RenderTrait, center: Point) {
        let (black, _white) = self.get_note_colors();

        let note_head = Rectangle {
            x: center.x - 5.0,
            y: center.y - 1.0,
            width: 15.0,
            height: 15.0,
        };

        // Fondo blanco
        renderer.fill_quad(
            Quad {
                bounds: note_head,
                border: iced::Border {
                    color: black,
                    width: 4.0,
                    radius: 4.0.into(),
                },
                shadow: Default::default(),
            },
            Color::WHITE,
        );

        // 2. Dibujar la plica (línea vertical)
        self.draw_stem(renderer, center, black);
    }

    // Dibujar negra (1)
    fn draw_quarter_note(&self, renderer: &mut impl RenderTrait, center: Point) {
        let (black, _white) = self.get_note_colors();

        // 1. Dibujar la cabeza de la nota (óvalo NEGRO)
        let note_head = Rectangle {
            x: center.x - 5.0,
            y: center.y,
            width: 15.0,
            height: 15.0,
        };

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
        self.draw_stem(renderer, center, black);
    }

    // Dibujar corchea (0.5)
    fn draw_eighth_note(&self, renderer: &mut impl RenderTrait, center: Point) {
        let (black, _white) = self.get_note_colors();

        let note_head: Rectangle = Rectangle {
            x: center.x - 5.0,
            y: center.y,
            width: 15.0,
            height: 15.0,
        };
        // Óvalo negro
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
        // Plica
        self.draw_stem(renderer, center, black);
        // Bandera simple
        let flag = Rectangle {
            x: center.x + 8.0,
            y: center.y - 25.0,
            width: 15.0,
            height: 6.0,
        };
        renderer.fill_quad(
            Quad {
                bounds: flag,
                border: iced::Border {
                    color: black,
                    width: 0.0,
                    radius: 3.0.into(),
                },
                shadow: Default::default(),
            },
            black,
        );
    }

    // Dibujar semicorchea (0.25)
    fn draw_sixteenth_note(&self, renderer: &mut impl RenderTrait, center: Point) {
        self.draw_eighth_note(renderer, center);
        let (black, _white) = self.get_note_colors();

        let flag2 = Rectangle {
            x: center.x + 10.0,
            y: center.y - 18.0,
            width: 12.0,
            height: 6.0,
        };

        renderer.fill_quad(
            Quad {
                bounds: flag2,
                border: Border {
                    color: black,
                    width: 0.0,
                    radius: 3.0.into(),
                },
                shadow: Default::default(),
            },
            black,
        );
    }

    // Dibujar fusa (0.125)
    fn draw_thirty_second_note(&self, renderer: &mut impl RenderTrait, center: Point) {
        let (black, _white) = self.get_note_colors();

        self.draw_sixteenth_note(renderer, center);
        let flag3 = Rectangle {
            x: center.x + 10.0,
            y: center.y - 11.0,
            width: 12.0,
            height: 6.0,
        };
        renderer.fill_quad(
            Quad {
                bounds: flag3,
                border: iced::Border {
                    color: black,
                    width: 0.0,
                    radius: 3.0.into(),
                },
                shadow: Default::default(),
            },
            black,
        );
    }

    // Dibujar semifusa (0.0625)
    fn draw_sixty_fourth_note(&self, renderer: &mut impl RenderTrait, center: Point) {
        let (black, _white) = self.get_note_colors();

        self.draw_thirty_second_note(renderer, center);
        // Cuarta bandera
        let flag4 = Rectangle {
            x: center.x + 10.0,
            y: center.y - 4.0,
            width: 12.0,
            height: 6.0,
        };
        renderer.fill_quad(
            Quad {
                bounds: flag4,
                border: Border {
                    color: black,
                    width: 0.0,
                    radius: 3.0.into(),
                },
                shadow: Default::default(),
            },
            black,
        );
    }

    // Dibujar plica vertical, negras y blancas
    fn draw_stem(&self, renderer: &mut impl RenderTrait, center: Point, color: Color) {
        let stem_rect = Rectangle {
            x: center.x + 5.0,  // Desde el borde derecho de la cabeza
            y: center.y - 25.0, // Hacia arriba
            width: 5.0,
            height: 30.0,
        };

        renderer.fill_quad(
            Quad {
                bounds: stem_rect,
                border: iced::Border {
                    color,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                shadow: Default::default(),
            },
            color,
        );
    }

    // Obtener colores según el estado de la nota
    fn get_note_colors(&self) -> (Color, Color) {
        if self.is_active {
            // Rojo cangrejo más vivo pero pastel: RGB(0.94, 0.35, 0.25)
            (
                Color::from_rgb(0.94, 0.35, 0.25),
                Color::from_rgb(0.94, 0.35, 0.25),
            )
        } else {
            (Color::BLACK, Color::WHITE)
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
        Node::new(Size::ZERO)
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
        // Obtenemos la posicion (x,y)
        let bounds: Rectangle = layout.bounds();
        let note_head_center = Point::new(
            bounds.x, // Un poco hacia la derecha del inicio
            bounds.y, // Centro vertical
        );

        match self.duration {
            4.0 => self.draw_whole_note(renderer, note_head_center),
            2.0 => self.draw_half_note(renderer, note_head_center),
            1.0 => self.draw_quarter_note(renderer, note_head_center),
            0.5 => self.draw_eighth_note(renderer, note_head_center),
            0.25 => self.draw_sixteenth_note(renderer, note_head_center),
            0.125 => self.draw_thirty_second_note(renderer, note_head_center),
            0.0625 => self.draw_sixty_fourth_note(renderer, note_head_center),
            _ => self.draw_quarter_note(renderer, note_head_center),
        }
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
