use crate::message::states::{AppMessage, GameMessage};
use iced::{
    Border, Color, Size,
    advanced::{self, Clipboard, Layout, Overlay, Shell, layout::Node, overlay, renderer::Quad},
    alignment::{Horizontal, Vertical},
    event::{Event, Status},
    keyboard, mouse,
    widget::text::{LineHeight, Shaping, Wrapping},
};

pub struct TemporizedIntroOverlay {
    pub elapsed: f32,        // Tiempo transcurrido
    pub partiture_time: f32, // Tiempo total de la partitura
}

impl<Theme, Renderer> Overlay<AppMessage, Theme, Renderer> for TemporizedIntroOverlay
where
    Renderer: advanced::Renderer + advanced::text::Renderer,
{
    fn layout(&mut self, _renderer: &Renderer, bounds: Size) -> Node {
        Node::new(bounds)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: Layout<'_>,
        _cursor: iced::mouse::Cursor,
    ) {
        let elapsed: i32 = self.elapsed.floor() as i32;

        // MOstrar el tiempo transcurrido, [3,2,1]
        if elapsed <= 2 {
            renderer.fill_text(
                advanced::Text {
                    content: (3 - elapsed).to_string(),
                    bounds: Size::new(layout.bounds().width, layout.bounds().height),
                    size: iced::Pixels(200.0),
                    font: renderer.default_font(),
                    horizontal_alignment: Horizontal::Center,
                    vertical_alignment: Vertical::Center,
                    line_height: LineHeight::default(),
                    shaping: Shaping::Advanced,
                    wrapping: Wrapping::None,
                },
                layout.bounds().center(),
                Color::from_rgb(0.98, 0.10, 0.10),
                layout.bounds(),
            );

            renderer.fill_quad(
                Quad {
                    bounds: layout.bounds(),
                    border: Border {
                        color: Color::from_rgba(0.8, 0.8, 0.8, 0.5),
                        width: 0.0,
                        radius: 0.0.into(),
                    },
                    shadow: Default::default(),
                },
                Color::from_rgba(0.0, 0.0, 0.0, 0.5),
            );
        }
    }

    fn on_event(
        &mut self,
        event: Event,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, AppMessage>,
    ) -> Status {
        if self.elapsed > self.partiture_time {
            // Enviar mensaje usando shell.publish()
            shell.publish(AppMessage::Game(GameMessage::Finished));
            // Opcionalmente, puedes devolver Captured para indicar que el evento fue procesado
            Status::Captured
        } else {
            match event {
                Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => match key {
                    keyboard::Key::Named(keyboard::key::Named::Escape) => {
                        shell.publish(AppMessage::Game(GameMessage::PauseGame));
                        Status::Captured
                    }
                    keyboard::Key::Named(keyboard::key::Named::Space) => {
                        shell.publish(AppMessage::Game(GameMessage::PauseGame));
                        Status::Captured
                    }
                    _ => Status::Ignored,
                },
                _ => Status::Ignored,
            }
        }
    }

    fn overlay<'b>(
        &'b mut self,
        _layout: Layout<'_>,
        _renderer: &Renderer,
    ) -> Option<overlay::Element<'b, AppMessage, Theme, Renderer>> {
        None
    }
}
