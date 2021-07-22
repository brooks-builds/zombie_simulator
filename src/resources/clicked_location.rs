use std::ops::{Deref, DerefMut};

use crate::data_structures::vector2::Vector2;

#[derive(Debug)]
pub struct ClickedLocation(pub Option<Vector2>);

impl ClickedLocation {
    pub fn set(&mut self, x: f32, y: f32) {
        self.0 = Some(Vector2 { x, y });
    }

    pub fn clear(&mut self) {
        self.0 = None;
    }
}

impl Deref for ClickedLocation {
    type Target = Option<Vector2>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ClickedLocation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
