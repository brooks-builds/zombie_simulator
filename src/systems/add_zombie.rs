use bbecs::World;

use crate::{
    components::{
        acceleration::Acceleration, color::Color, draw_vision_range::DrawVisionRange,
        location::Location, speed::Speed, velocity::Velocity, vision_range::VisionRange,
        zombie::Zombie,
    },
    resources::{
        clicked_location::ClickedLocation, zombie_color::ZombieColor, zombie_speed::ZombieSpeed,
        zombie_vision_range::ZombieVisionRange,
    },
};

pub struct AddZombie;

impl AddZombie {
    pub fn run(self, world: &mut World) {
        let zombie_color = world
            .get_resource::<ZombieColor>()
            .unwrap()
            .get_ggez_color();
        let zombie_speed = world.get_resource::<ZombieSpeed>().unwrap().get();
        let zombie_vision_range = world.get_resource::<ZombieVisionRange>().unwrap().get();
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
            .with_component(Color(zombie_color))
            .with_component(Speed(zombie_speed))
            .with_component(Acceleration::new())
            .with_component(Velocity::new())
            .with_component(VisionRange(zombie_vision_range))
            .with_component(DrawVisionRange)
            .with_component(Zombie);
    }
}
