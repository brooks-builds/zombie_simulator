use std::ops::Deref;

#[derive(Debug)]
pub struct ZombieSpeed(pub f32);

impl ZombieSpeed {
    pub fn get(&self) -> f32 {
        self.0
    }
}

impl Deref for ZombieSpeed {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
