use eyre::Result;
use ggez::{
    conf::WindowMode,
    event,
    graphics::{self, BLACK},
    ContextBuilder,
};
use zombie_simulator::{config::Config, resources::arena_size::ArenaSize, MainState};

fn main() -> Result<()> {
    let window_mode = WindowMode::default().dimensions(800.0, 800.0);
    let (mut context, mut event_loop) = ContextBuilder::new("zombie_simulator", "Brooks Builds")
        .window_mode(window_mode)
        .build()?;

    let config = Config {
        arena_size: ArenaSize::new(800.0, 800.0),
        background_color: BLACK,
        size: 5.0,
        target_fps: 60,
        speed: 0.15,
        humans: 750, // set to 1250 for release
        human_vision_range: 50.0,
        dying_color: graphics::Color::new(0.3, 0.3, 0.3, 1.0),
        dying_time: 600,
        zombie_color: ggez::graphics::Color::new(0.0, 1.0, 0.0, 1.0),
        zombie_speed: 0.01,
        zombie_vision_range: 100.0,
    };
    let mut main_state = MainState::new(config, &mut context)?;
    event::run(&mut context, &mut event_loop, &mut main_state)?;
    Ok(())
}
