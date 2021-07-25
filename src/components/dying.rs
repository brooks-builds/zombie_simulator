#[derive(Debug)]
pub struct Dying(pub u32);

impl Dying {
    pub fn decrement_if_possible(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }

    pub fn is_dead(&self) -> bool {
        self.0 == 0
    }
}
