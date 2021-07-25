use std::ops::Deref;

#[derive(Debug, Copy, Clone)]
pub struct Speed(pub f32);

impl Speed {
    pub fn set(&mut self, new_speed: f32) {
        self.0 = new_speed;
    }
}

impl Deref for Speed {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
