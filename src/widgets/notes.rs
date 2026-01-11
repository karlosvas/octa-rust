use {
    crate::models::note::{Note, PaletteColors},
    iced::{
        Color, Point, Rectangle, Size,
        widget::canvas::{Frame, Path, Stroke, Style},
    },
};

// Constructor para la nota musical
impl Note {
    // Constructor para crear una nota con nombre, posición, tono y duración
    pub fn new(pitch: u8, duration: f32, joined: bool, last_position: Point) -> Self {
        Self {
            start: 0.0,
            pitch,
            duration,
            is_active: false,
            joined,
            last_position,
            is_rest: pitch == 0,
        }
    }

    // Dibujar redonda (4)
    fn draw_whole_note(&self, frame: &mut Frame, center: Point, palette: &PaletteColors) {
        // Dibuja la cabeza de la nota como un círculo
        let head: Path = Path::rectangle(center, Size::new(8.0, 5.0));

        // Relleno blanco
        frame.fill(&head, palette.secondary);

        // Borde negro grueso
        frame.stroke(
            &head,
            Stroke {
                style: Style::Solid(palette.primary),
                width: 4.0,
                ..Stroke::default()
            },
        );
    }

    // Dibujar blanca (2)
    fn draw_half_note(&self, frame: &mut Frame, center: Point, palette: &PaletteColors) {
        // Dibuja la cabeza de la nota como un círculo
        let head: Path = Path::rectangle(center, Size::new(8.0, 5.0));
        // Relleno blanco
        frame.fill(&head, palette.secondary);
        // Borde negro grueso
        frame.stroke(
            &head,
            Stroke {
                style: Style::Solid(palette.primary),
                width: 4.0,
                ..Stroke::default()
            },
        );
    }

    // Dibujar negra (1)
    fn draw_quarter_note(&self, frame: &mut Frame, center: Point, palette: &PaletteColors) {
        // Dibuja la cabeza de la nota como un círculo
        let head: Path = Path::rectangle(center, Size::new(10.0, 5.0));
        // Relleno blanco
        frame.fill(&head, palette.secondary);
        // Borde negro grueso
        frame.stroke(
            &head,
            Stroke {
                style: Style::Solid(palette.primary),
                width: 4.0,
                ..Stroke::default()
            },
        );
    }

    // Dibujar corchea (0.5)
    fn draw_eighth_note(&self, frame: &mut Frame, center: Point, palette: &PaletteColors) {
        // Dibuja la cabeza de la nota como un círculo
        let head: Path = Path::rectangle(center, Size::new(10.0, 5.0));

        // Relleno negro
        frame.fill(&head, palette.primary);
        // Borde negro grueso
        frame.stroke(
            &head,
            Stroke {
                style: Style::Solid(palette.primary),
                width: 4.0,
                ..Stroke::default()
            },
        );
    }

    // Dibujar semicorchea (0.25)
    fn draw_sixteenth_note(&self, frame: &mut Frame, center: Point, palette: &PaletteColors) {
        self.draw_eighth_note(frame, center, palette);

        let head: Path = Path::rectangle(center, Size::new(6.0, 3.0));
        frame.fill(&head, palette.secondary);
    }

    // Dibujar fusa (0.125)
    fn draw_thirty_second_note(&self, frame: &mut Frame, center: Point, palette: &PaletteColors) {
        self.draw_sixteenth_note(frame, center, palette);

        self.draw_sixteenth_note(frame, center, palette);

        let head: Path = Path::rectangle(center, Size::new(2.0, 2.0));
        frame.fill(&head, palette.secondary);
    }

    // Dibujar semifusa (0.0625)
    fn draw_sixty_fourth_note(&self, frame: &mut Frame, center: Point, palette: &PaletteColors) {
        self.draw_thirty_second_note(frame, center, palette);
        let head: Path = Path::rectangle(center, Size::new(2.0, 2.0));
        frame.fill(&head, palette.secondary);
    }

    // Dibujar plica vertical, negras y blancas
    fn draw_stem(&self, frame: &mut Frame, mut center: Point, color: Color) {
        if self.pitch < 54 {
            // Mano izquierda - plica hacia arriba
            center.y -= 25.0;
        } else if self.pitch < 60 {
            // Mano izquierda - plica hacia abajo
            center.y += 5.0;
            center.x -= 9.0;
        } else if self.pitch <= 71 {
            // Mano derecha - plica hacia arriba
            center.y -= 25.0;
        } else {
            // Mano derecha - plica hacia abajo
            center.y += 5.0;
            center.x -= 9.0;
        }

        let stem_rect = Rectangle {
            x: center.x + 7.0, // Desde el borde derecho de la cabeza
            y: center.y,       // Hacia arriba
            width: 5.0,
            height: 30.0,
        };

        let path: Path = Path::rectangle(
            Point::new(stem_rect.x, stem_rect.y),
            Size::new(stem_rect.width, stem_rect.height),
        );
        frame.fill(&path, color);
    }

    // Color de la nota
    fn get_note_colors(&self) -> PaletteColors {
        if self.is_active {
            PaletteColors {
                primary: Color::from_rgb(0.94, 0.35, 0.25),
                secondary: Color::WHITE,
            }
        } else {
            PaletteColors {
                primary: Color::BLACK,
                secondary: Color::WHITE,
            }
        }
    }

    // Dibujar la nota
    pub fn draw(&self, frame: &mut Frame, center: Point) {
        let palette: PaletteColors = self.get_note_colors();

        match self.duration {
            4.0 => self.draw_whole_note(frame, center, &palette),
            2.0 => self.draw_half_note(frame, center, &palette),
            1.0 => self.draw_quarter_note(frame, center, &palette),
            0.5 => self.draw_eighth_note(frame, center, &palette),
            0.25 => self.draw_sixteenth_note(frame, center, &palette),
            0.125 => self.draw_thirty_second_note(frame, center, &palette),
            0.0625 => self.draw_sixty_fourth_note(frame, center, &palette),
            _ => self.draw_quarter_note(frame, center, &palette),
        }

        // Dibujar la bandera
        self.draw_stem(frame, center, palette.primary);
    }
}
