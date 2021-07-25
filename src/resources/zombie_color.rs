use std::ops::Deref;

use ggez::graphics;

#[derive(Debug)]
pub struct ZombieColor(pub graphics::Color);

impl ZombieColor {
    pub fn get_ggez_color(&self) -> graphics::Color {
        self.0
    }
}

impl Deref for ZombieColor {
    type Target = graphics::Color;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
