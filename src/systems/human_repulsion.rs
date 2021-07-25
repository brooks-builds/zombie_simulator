use bbecs::World;
use eyre::Result;

use crate::components::{
    acceleration::Acceleration, alive::Alive, human::Human, location::Location, speed::Speed,
    vision_range::VisionRange,
};

pub struct HumanRepulsion;

impl HumanRepulsion {
    pub fn run(self, world: &World) -> Result<()> {
        let query = world
            .query()
            .with_component::<Location>()
            .with_component::<Speed>()
            .with_component::<Acceleration>()
            .with_component::<VisionRange>()
            .with_component::<Human>()
            .with_component::<Alive>()
            .run()?;
        let other_wrapped_locations = query.1[0].clone();

        for (entity_index, entity_wrapped_location) in query.1[0].iter().enumerate() {
            let borrowed_entity_location = entity_wrapped_location.borrow();
            let borrowed_entity_vision_range = query.1[3][entity_index].borrow();
            let entity_location = borrowed_entity_location.downcast_ref::<Location>().unwrap();
            let entity_vision_range = borrowed_entity_vision_range
                .downcast_ref::<VisionRange>()
                .unwrap();
            let borrowed_entity_speed = query.1[1][entity_index].borrow();
            let entity_speed = borrowed_entity_speed.downcast_ref::<Speed>().unwrap();
            let mut borrowed_entity_acceleration = query.1[2][entity_index].borrow_mut();
            let entity_acceleration = borrowed_entity_acceleration
                .downcast_mut::<Acceleration>()
                .unwrap();

            for (other_entity_index, other_wrapped_location) in
                other_wrapped_locations.iter().enumerate()
            {
                if entity_index == other_entity_index {
                    continue;
                }
                let borrowed_other_location = other_wrapped_location.borrow();
                let other_location = borrowed_other_location.downcast_ref::<Location>().unwrap();

                if entity_location.distance(&**other_location) < **entity_vision_range {
                    let mut force = **entity_location - &**other_location;
                    force.normalize();
                    force *= **entity_speed;
                    **entity_acceleration += force;
                }
            }
        }
        Ok(())
    }
}
