use std::ops::Deref;

#[derive(Debug)]
pub struct ZombieVisionRange(pub f32);

impl ZombieVisionRange {
    pub fn get(&self) -> f32 {
        self.0
    }
}

impl Deref for ZombieVisionRange {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
