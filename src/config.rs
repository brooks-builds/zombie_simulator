use ggez::graphics::Color;

use crate::resources::arena_size::ArenaSize;

pub struct Config {
    pub arena_size: ArenaSize,
    pub background_color: Color,
    pub size: f32,
    pub target_fps: u32,
    pub speed: f32,
    pub humans: u32,
    pub human_vision_range: f32,
}
