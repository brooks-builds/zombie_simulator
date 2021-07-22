use std::ops::{Deref, DerefMut};

use crate::data_structures::vector2::Vector2;

#[derive(Debug, Clone, Copy, Default)]
pub struct Acceleration(pub Vector2);

impl Acceleration {
    pub fn new() -> Self {
        Acceleration(Vector2::new_zero())
    }
}

impl Deref for Acceleration {
    type Target = Vector2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Acceleration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
