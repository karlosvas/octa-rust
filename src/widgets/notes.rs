use iced::{
    Border, Color, Point, Rectangle, Size,
    advanced::{
        Layout, Renderer as RenderTrait,
        graphics::core::event::Status,
        layout::Node,
        overlay::Overlay,
        renderer::{self, Quad},
    },
    mouse::Cursor,
};
use serde::{Deserialize, Serialize};

// Notas, con implementacion para el trait Overlay y Serialize/Deserialize con serde_json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub name: String,  // Nombre de la nota (ej. "C4", "G#5")
    pub start: f32,    // Tiempo de inicio en segundos
    pub pitch: u8,     // Número MIDI del tono
    pub duration: f32, // Duración en segundos
    #[serde(skip)]
    pub is_active: bool, // Si la nota está activa (sonando)
    #[serde(skip)]
    pub joined: bool, // Si la nota está unida a otra
    #[serde(skip)]
    pub last_position: Point, // Última posición de la nota
}

// Constructor para la nota musical
impl Note {
    // Constructor para crear una nota con nombre, posición, tono y duración
    #[allow(dead_code)]
    pub fn new(name: String, pitch: u8, duration: f32) -> Self {
        Self {
            name,
            start: 0.0,
            pitch,
            duration,
            is_active: false,
            joined: false,
            last_position: Point::default(),
        }
    }

    // Dibujar redonda (4)
    fn draw_whole_note(&self, renderer: &mut impl RenderTrait, center: Point) {
        let (black, white) = self.get_note_colors();

        let note_head: Rectangle = Rectangle {
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

        // Dibujar la plica
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

        // Dibujar la plica
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

        // Dibujar la plica
        self.draw_stem(renderer, center, black);
        // Dibujar la bandera
        self.draw_flag(renderer, center, black);
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

        // Dibujar la plica
        self.draw_stem(renderer, center, black);
        // Dibujar la bandera
        self.draw_flag(renderer, center, black);
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

        // Dibujar la plica
        self.draw_stem(renderer, center, black);
        // Dibujar la bandera
        self.draw_flag(renderer, center, black);
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

        // Dibujar la plica
        self.draw_stem(renderer, center, black);
        // Dibujar la bandera
        self.draw_flag(renderer, center, black);
    }

    // Dibujar plica vertical, negras y blancas
    fn draw_stem(&self, renderer: &mut impl RenderTrait, mut center: Point, color: Color) {
        if self.pitch < 54 {
            // Mano izquierda - plica hacia arriba
            center.y -= 25.0;
        } else if self.pitch < 60 {
            // Mano izquierda - plica hacia abajo
            center.y += 5.0;
            center.x -= 10.0;
        } else if self.pitch <= 71 {
            // Mano derecha - plica hacia arriba
            center.y -= 25.0;
        } else {
            // Mano derecha - plica hacia abajo
            center.y += 5.0;
            center.x -= 10.0;
        }

        let stem_rect = Rectangle {
            x: center.x + 5.0, // Desde el borde derecho de la cabeza
            y: center.y,       // Hacia arriba
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

    // Dibujar la bandera de las plicas
    fn draw_flag(&self, renderer: &mut impl RenderTrait, mut center: Point, color: Color) {
        if self.joined {
            // Si la nota está unida, no dibujar bandera
            // Dibujar línea desde center a last_position
            let line_rect = Rectangle {
                x: center.x.min(self.last_position.x),
                y: center.y.min(self.last_position.y),
                width: (center.x - self.last_position.x).abs().max(2.0), // mínimo grosor
                height: (center.y - self.last_position.y).abs().max(2.0),
            };
            renderer.fill_quad(
                Quad {
                    bounds: line_rect,
                    border: Border {
                        color,
                        width: 2.0,
                        radius: 1.0.into(),
                    },
                    shadow: Default::default(),
                },
                color,
            );
            // Si la nota está unida, no dibujar bandera
            return;
        }

        if self.pitch < 54 {
            // Mano izquierda - plica hacia arriba
            center.y -= 25.0;
        } else if self.pitch < 60 {
            // Mano izquierda - plica hacia abajo
            center.y += 5.0;
            center.x -= 10.0;
        } else if self.pitch <= 71 {
            // Mano derecha - plica hacia arriba
            center.y -= 25.0;
        } else {
            // Mano derecha - plica hacia abajo
            center.y += 29.0; // Ajustar la posición hacia abajo
            center.x -= 10.0; // Ajustar la posición horizontal
        }

        // Bandera simple
        let flag: Rectangle = Rectangle {
            x: center.x + 8.0,
            y: center.y,
            width: 15.0,
            height: 6.0,
        };
        renderer.fill_quad(
            Quad {
                bounds: flag,
                border: Border {
                    color: color,
                    width: 5.0,
                    radius: 2.0.into(),
                },
                shadow: Default::default(),
            },
            color,
        );
    }

    // Obtener colores según el estado de la nota
    fn get_note_colors(&self) -> (Color, Color) {
        if self.joined {
            // Verde para notas unidas: RGB(0.0, 0.5, 0.0)
            (
                Color::from_rgb(0.0, 0.5, 0.0),
                Color::from_rgb(0.0, 0.5, 0.0),
            )
        } else if self.is_active {
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

    // Dibujar la nota
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
}
