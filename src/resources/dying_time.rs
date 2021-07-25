use std::ops::Deref;

#[derive(Debug)]
pub struct DyingTime(pub u32);

impl Deref for DyingTime {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
