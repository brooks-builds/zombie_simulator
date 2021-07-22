use crate::components::{location::Location, velocity::Velocity};
use bbecs::World;
use eyre::{anyhow, Result};

pub struct UpdateLocation;

impl UpdateLocation {
    pub fn run(self, world: &World) -> Result<()> {
        let query = world
            .query()
            .with_component::<Velocity>()
            .with_component::<Location>()
            .run()?;
        let velocities = query[0].clone();
        let locations = query[1].clone();
        for (index, wrapped_velocity) in velocities.iter().enumerate() {
            let borrowed_velocity = wrapped_velocity.borrow();
            let mut borrowed_location = locations[index].borrow_mut();
            let velocity = borrowed_velocity
                .downcast_ref::<Velocity>()
                .ok_or_else(|| anyhow!("Error downcasting to velocity"))?;
            let location = borrowed_location
                .downcast_mut::<Location>()
                .ok_or_else(|| anyhow!("Error downcasting to location"))?;

            **location += **velocity;
        }
        Ok(())
    }
}
