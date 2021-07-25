use bbecs::World;
use eyre::{anyhow, Result};

use crate::{
    components::{location::Location, velocity::Velocity},
    resources::{arena_size::ArenaSize, entity_size::EntitySize},
};

pub struct ContainEntitiesInArena;

impl ContainEntitiesInArena {
    pub fn run(self, world: &World) -> Result<()> {
        let arena_size = world
            .get_resource::<ArenaSize>()
            .ok_or_else(|| anyhow!("Error getting arena size from world"))?;
        let entity_size = world
            .get_resource::<EntitySize>()
            .ok_or_else(|| anyhow!("Error getting entity size from world"))?;
        let query = world
            .query()
            .with_component::<Velocity>()
            .with_component::<Location>()
            .run()?;
        let velocities = query.1[0].clone();
        let locations = query.1[1].clone();

        for (index, wrapped_velocity) in velocities.iter().enumerate() {
            let mut borrowed_velocity = wrapped_velocity.borrow_mut();
            let mut borrowed_location = locations[index].borrow_mut();
            let velocity = borrowed_velocity
                .downcast_mut::<Velocity>()
                .ok_or_else(|| anyhow!("Error downcasting velocity"))?;
            let location = borrowed_location
                .downcast_mut::<Location>()
                .ok_or_else(|| anyhow!("Error downcasting location"))?;

            if location.x + entity_size.half() > arena_size.width {
                location.x = arena_size.width - entity_size.half();
                velocity.clear_x();
            } else if location.x - entity_size.half() < 0.0 {
                location.x = 0.0 + entity_size.half();
                velocity.clear_x();
            }

            if location.y + entity_size.half() > arena_size.height {
                location.y = arena_size.height - entity_size.half();
                velocity.clear_y();
            } else if location.y - entity_size.half() < 0.0 {
                location.y = 0.0 + entity_size.half();
                velocity.clear_y();
            }
        }
        Ok(())
    }
}
