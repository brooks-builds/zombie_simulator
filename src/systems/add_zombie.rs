use bbecs::World;

use crate::{
    components::{
        acceleration::Acceleration, color::Color, draw_vision_range::DrawVisionRange,
        location::Location, speed::Speed, velocity::Velocity, vision_range::VisionRange,
    },
    resources::clicked_location::ClickedLocation,
};

pub struct AddZombie;

const ZOMBIE_COLOR: ggez::graphics::Color = ggez::graphics::Color::new(0.0, 1.0, 0.0, 1.0);
const ZOMBIE_SPEED: f32 = 0.1;
const ZOMBIE_VISION_RANGE: f32 = 100.0;

impl AddZombie {
    pub fn run(self, world: &mut World) {
        let clicked_location = world.get_resource_mut::<ClickedLocation>().unwrap();

        let location = if let Some(location) = **clicked_location {
            location
        } else {
            return;
        };

        clicked_location.clear();
        world
            .register_entity()
            .with_component(Location(location))
            .with_component(Color(ZOMBIE_COLOR))
            .with_component(Speed(ZOMBIE_SPEED))
            .with_component(Acceleration::new())
            .with_component(Velocity::new())
            .with_component(VisionRange(ZOMBIE_VISION_RANGE))
            .with_component(DrawVisionRange);
    }
}
