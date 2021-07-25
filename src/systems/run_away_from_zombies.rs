use bbecs::{errors::Errors, World};
use eyre::Result;

use crate::components::{
    acceleration::Acceleration, alive::Alive, human::Human, location::Location, speed::Speed,
    vision_range::VisionRange, zombie::Zombie,
};

pub struct RunAwayFromZombies;

impl RunAwayFromZombies {
    pub fn run(self, world: &World) -> Result<()> {
        let (_, human_query) = world
            .query()
            .with_component::<Human>()
            .with_component::<Alive>()
            .with_component::<Location>()
            .with_component::<VisionRange>()
            .with_component::<Acceleration>()
            .with_component::<Speed>()
            .run()?;
        let zombie_query = match world
            .query()
            .with_component::<Zombie>()
            .with_component::<Location>()
            .run()
        {
            Ok(query) => query.1,
            Err(error) => {
                return match error.downcast_ref::<Errors>() {
                    Some(Errors::ComponentNotFound) => return Ok(()),
                    _ => Err(error),
                }
            }
        };

        for (human_index, wrapped_human_location) in human_query[2].iter().enumerate() {
            let borrowed_human_location = wrapped_human_location.borrow();
            let human_location = borrowed_human_location.downcast_ref::<Location>().unwrap();
            let borrowed_human_vision_range = human_query[3][human_index].borrow();
            let human_vision_range = borrowed_human_vision_range
                .downcast_ref::<VisionRange>()
                .unwrap();

            for (_zombie_index, wrapped_zombie_location) in zombie_query[1].iter().enumerate() {
                let borrowed_zombie_location = wrapped_zombie_location.borrow();
                let zombie_location = borrowed_zombie_location.downcast_ref::<Location>().unwrap();

                if human_location.distance(&**zombie_location) <= human_vision_range.get() {
                    let borrowed_human_speed = human_query[5][human_index].borrow();
                    let human_speed = borrowed_human_speed.downcast_ref::<Speed>().unwrap();
                    let mut run_away_force = **human_location - &**zombie_location;
                    let mut borrowed_human_acceleration = human_query[4][human_index].borrow_mut();
                    let human_acceleration = borrowed_human_acceleration
                        .downcast_mut::<Acceleration>()
                        .unwrap();
                    run_away_force.normalize();
                    run_away_force *= **human_speed;
                    **human_acceleration += run_away_force;
                }
            }
        }
        Ok(())
    }
}
