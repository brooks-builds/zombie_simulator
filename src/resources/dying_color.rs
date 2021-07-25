use std::ops::Deref;

use ggez::graphics;

#[derive(Debug)]
pub struct DyingColor(pub graphics::Color);

impl Deref for DyingColor {
    type Target = graphics::Color;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
