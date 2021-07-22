use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
pub struct EntitySize(pub f32);

impl EntitySize {
    pub fn half(&self) -> f32 {
        self.0 / 2.0
    }
}

impl Deref for EntitySize {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
