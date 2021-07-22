#[derive(Debug, Clone, Copy)]
pub struct ArenaSize {
    pub width: f32,
    pub height: f32,
}

impl ArenaSize {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}
