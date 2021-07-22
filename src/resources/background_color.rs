use std::ops::Deref;

use ggez::graphics::Color;

pub struct BackgroundColor(pub Color);

impl Deref for BackgroundColor {
    type Target = Color;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
