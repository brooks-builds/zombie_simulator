use bbecs::World;
use eyre::{anyhow, Result};

use crate::components::{acceleration::Acceleration, velocity::Velocity};

pub struct UpdateVelocity;

impl UpdateVelocity {
    pub fn run(self, world: &World) -> Result<()> {
        let query = world
            .query()
            .with_component::<Acceleration>()
            .with_component::<Velocity>()
            .run()?;
        let accelerations = query[0].clone();
        let velocities = query[1].clone();
        assert_eq!(accelerations.len(), velocities.len());
        for (index, acceleration) in accelerations.iter().enumerate() {
            let borrowed_acceleration = acceleration.borrow();
            let mut borrowed_velocity = velocities[index].borrow_mut();
            let acceleration = borrowed_acceleration
                .downcast_ref::<Acceleration>()
                .ok_or_else(|| anyhow!("Error downcasting to acceleration"))?;
            let velocity = borrowed_velocity
                .downcast_mut::<Velocity>()
                .ok_or_else(|| anyhow!("Error downcasting to velocity"))?;
            **velocity += **acceleration;
        }
        Ok(())
    }
}
