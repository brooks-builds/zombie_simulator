use std::ops::Deref;

#[derive(Debug)]
pub struct Color(pub ggez::graphics::Color);

impl Deref for Color {
    type Target = ggez::graphics::Color;
    fn deref(&self) -> &ggez::graphics::Color {
        &self.0
    }
}
