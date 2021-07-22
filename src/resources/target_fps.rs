use std::ops::Deref;

#[derive(Debug)]
pub struct TargetFps(pub u32);

impl Deref for TargetFps {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
