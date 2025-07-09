use crate::message::states;
use crate::styles;
use iced::{
    Length,
    widget::{Container, Space, column, container, row},
};

pub struct Partiture;

impl Partiture {
    pub fn create_partiture() -> Container<'static, states::Buttons> {
        let mut staff = column![].spacing(20);

        for _ in 0..8 {
            // Crear una fila para contener la nota y la línea
            let line_row = row![
                // Línea horizontal
                container(Space::new(Length::Fill, Length::Fixed(1.0)))
                    .width(Length::Fill)
                    .height(Length::Fixed(1.0))
                    .style(styles::custom_style::staff_line)
            ];
            staff = staff.push(line_row);
        }

        container(staff)
            .width(Length::Fill)
            .height(Length::Shrink)
            .padding(20)
            .style(styles::custom_style::partiture)
            .into()
    }
}
