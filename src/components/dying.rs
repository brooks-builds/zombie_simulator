#[derive(Debug)]
pub struct Dying(pub u32);

impl Dying {
    pub fn decrement(&mut self) {
        self.0 -= 1;
    }
}
