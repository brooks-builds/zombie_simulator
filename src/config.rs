use ggez::graphics::{self, Color};

use crate::resources::arena_size::ArenaSize;

pub struct Config {
    pub arena_size: ArenaSize,
    pub background_color: Color,
    pub size: f32,
    pub target_fps: u32,
    pub speed: f32,
    pub humans: u32,
    pub human_vision_range: f32,
    pub dying_color: graphics::Color,
    pub dying_time: u32,
    pub zombie_color: graphics::Color,
    pub zombie_speed: f32,
    pub zombie_vision_range: f32,
}
