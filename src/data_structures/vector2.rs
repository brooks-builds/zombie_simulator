use std::ops::{AddAssign, MulAssign, Sub};

use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn new_random_range(minimum: f32, maximum: f32) -> Self {
        let mut rng = thread_rng();
        let x = rng.gen_range(minimum..maximum);
        let y = rng.gen_range(minimum..maximum);
        Self { x, y }
    }

    pub fn new_zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn to_mint_vector2(self) -> ggez::mint::Vector2<f32> {
        ggez::mint::Vector2 {
            x: self.x,
            y: self.y,
        }
    }

    pub fn clear(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }

    pub fn clear_x(&mut self) {
        self.x = 0.0;
    }

    pub fn clear_y(&mut self) {
        self.y = 0.0;
    }

    /// Get the magnitude of this vector.
    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn distance(&self, other: &Self) -> f32 {
        (*self - other).magnitude()
    }

    /// Normalize the vector. This will set the magnitude to 1.
    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        self.x /= magnitude;
        self.y /= magnitude;
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<&Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: &Vector2) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
