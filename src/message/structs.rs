use iced::alignment::Horizontal;
use iced::widget::image::Handle;
use iced::widget::{column, container, image, row, text, Container, Space};
use iced::{Alignment, Length};
use crate::message::enums::Message;
use crate::styles;
pub struct Partiture;

impl Partiture {
    pub fn create_partiture(hand: String) -> Container<'static, Message> {
        let note_names= ["Do", "Si", "La", "Sol", "Fa", "Mi", "Re", "Do'"];
        let mut staff = column![];
        let nota_image_path = match hand.as_str() {
            "left" => "assets/clave-de-fa.png",
            "right" => "assets/clave-de-sol.png",
            _ => "assets/default.png"
        };

        for note_name in note_names {
                // Crear una fila para contener la nota y la línea
                let line_row = row![
                    image(Handle::from_path(nota_image_path))
                        .width(Length::Fixed(40.0))
                        .height(Length::Fixed(40.0)),

                    // Nota musical (etiqueta de texto)
                    text(note_name)
                        .width(Length::Fixed(40.0))
                        .horizontal_alignment(Horizontal::Right),
                    
                    // Espacio pequeño entre la nota y la línea
                    container(Space::new(Length::Fixed(10.0), Length::Fill)),

                    // Línea horizontal
                    container(Space::new(Length::Fill, Length::Fixed(1.0)))
                        .width(Length::Fill)
                        .height(Length::Fixed(1.0))
                        .style(styles::custom_style::staff_line as fn(&iced::Theme) -> iced::widget::container::Appearance)
                ].align_items(Alignment::Center); 
                
                staff = staff.push(line_row);
        }

        container(staff)
            .width(Length::Fill)
            .padding(20)
            .style(styles::custom_style::partiture as fn(&iced::Theme) -> iced::widget::container::Appearance)
            .into()
    }
    
}