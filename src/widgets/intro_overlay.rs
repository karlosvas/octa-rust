use iced::{
    Color, Pixels, Point, Rectangle, Renderer, Theme,
    alignment::{Horizontal, Vertical},
    mouse::Cursor,
    widget::canvas::{Frame, Geometry, Path, Program, Text},
};

pub struct IntroOverlay {
    pub elapsed: f32,
}
impl<Message> Program<Message> for IntroOverlay {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        let elapsed = self.elapsed as i32;

        if elapsed <= 2 {
            frame.fill(
                &Path::rectangle(bounds.position(), bounds.size()),
                Color::from_rgba(0.0, 0.0, 0.0, 0.5),
            );

            frame.fill_text(Text {
                content: (3 - elapsed).to_string(),
                position: Point::new(bounds.width / 2.0, bounds.height / 2.0),
                color: Color::from_rgb(0.94, 0.35, 0.25),
                size: Pixels(150.0),
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                ..Default::default()
            });
        }

        vec![frame.into_geometry()]
    }
}
