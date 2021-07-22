use std::ops::{Deref, DerefMut};

use crate::data_structures::vector2::Vector2;

#[derive(Debug)]
pub struct Location(pub Vector2);

impl Deref for Location {
    type Target = Vector2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Location {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
