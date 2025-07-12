use iced::{
    Border, Color, Size,
    advanced::{
        Layout, Overlay,
        layout::Node,
        overlay::{self},
        renderer::Quad,
        text::Text,
    },
    alignment::{Horizontal, Vertical},
    event::Status,
    mouse::{Cursor, Interaction},
    widget::text::{LineHeight, Shaping, Wrapping},
};

pub struct TemporizedIntroOverlay {
    pub actual_time: f32,
}

impl<Message, Theme, Renderer> Overlay<Message, Theme, Renderer> for TemporizedIntroOverlay
where
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
    Theme: Clone + Default,
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
        let elapsed = self.actual_time.floor();

        if elapsed > 5.0 {
            return;
        }

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

        renderer.fill_text(
            Text {
                content: elapsed.to_string(),
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
    }

    fn on_event(
        &mut self,
        _event: iced::Event,
        _layout: Layout<'_>,
        _cursor: iced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        _shell: &mut iced::advanced::Shell<'_, Message>,
    ) -> Status {
        Status::Ignored
    }

    fn mouse_interaction(
        &self,
        _layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &iced::Rectangle,
        _renderer: &Renderer,
    ) -> Interaction {
        Interaction::default()
    }

    fn operate(
        &mut self,
        _layout: Layout<'_>,
        _renderer: &Renderer,
        _operation: &mut dyn iced::advanced::widget::Operation,
    ) {
    }

    fn is_over(
        &self,
        _layout: Layout<'_>,
        _renderer: &Renderer,
        _cursor_position: iced::Point,
    ) -> bool {
        false
    }

    fn overlay<'b>(
        &'b mut self,
        _layout: Layout<'_>,
        _renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        None
    }
}
