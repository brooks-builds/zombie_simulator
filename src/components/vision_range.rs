use std::ops::Deref;

#[derive(Debug)]
pub struct VisionRange(pub f32);

impl VisionRange {
    pub fn get(&self) -> f32 {
        self.0
    }
}

impl Deref for VisionRange {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
