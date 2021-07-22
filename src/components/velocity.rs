use std::ops::{Deref, DerefMut};

use crate::data_structures::vector2::Vector2;

#[derive(Debug)]
pub struct Velocity(pub Vector2);

impl Velocity {
    pub fn new() -> Self {
        Self(Vector2::new_zero())
    }
}

impl Deref for Velocity {
    type Target = Vector2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
