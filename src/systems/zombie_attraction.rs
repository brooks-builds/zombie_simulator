use bbecs::{errors::Errors, World};
use eyre::Result;

use crate::components::{
    acceleration::Acceleration, location::Location, speed::Speed, vision_range::VisionRange,
    zombie::Zombie,
};

pub struct ZombieAttraction;

impl ZombieAttraction {
    /// Get the zombies and humans, and then for each zombie have it move towards the closest
    /// human that it can see.
    pub fn run(self, world: &World) -> Result<()> {
        let zombies_query = match world
            .query()
            .with_component::<Zombie>()
            .with_component::<Location>()
            .with_component::<Acceleration>()
            .with_component::<VisionRange>()
            .with_component::<Speed>()
            .run()
        {
            Ok(query) => query,
            Err(error) => {
                return match error.downcast_ref::<bbecs::errors::Errors>() {
                    Some(Errors::ComponentNotFound) => Ok(()),
                    _ => Err(error),
                }
            }
        };
        let human_query = world.query().with_component::<Location>().run()?;
        let wrapped_human_locations = human_query.first().unwrap();

        if zombies_query[0].is_empty() {
            return Ok(());
        }

        for (index, wrapped_zombie_location) in zombies_query[1].iter().enumerate() {
            let borrowed_zombie_location = wrapped_zombie_location.borrow();
            let zombie_location = borrowed_zombie_location.downcast_ref::<Location>().unwrap();
            let borrowed_zombie_vision_range = zombies_query[3][index].borrow();
            let zombie_vision_range = borrowed_zombie_vision_range
                .downcast_ref::<VisionRange>()
                .unwrap();
            let borrowed_zombie_speed = zombies_query[4][index].borrow();
            let zombie_speed = borrowed_zombie_speed.downcast_ref::<Speed>().unwrap();
            let mut borrowed_zombie_acceleration = zombies_query[2][index].borrow_mut();
            let zombie_acceleration = borrowed_zombie_acceleration
                .downcast_mut::<Acceleration>()
                .unwrap();

            for wrapped_human_location in wrapped_human_locations {
                let borrowed_human_location = wrapped_human_location.borrow();
                let human_location = borrowed_human_location.downcast_ref::<Location>().unwrap();
                if zombie_location.distance(&human_location) < zombie_vision_range.get() {
                    let mut force = **human_location - zombie_location.get();
                    force.normalize();
                    force *= **zombie_speed;
                    **zombie_acceleration += force;
                }
            }
        }
        Ok(())
    }
}
