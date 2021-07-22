use std::ops::Deref;

#[derive(Debug, Copy, Clone)]
pub struct Speed(pub f32);

impl Deref for Speed {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
