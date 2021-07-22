use bbecs::World;
use eyre::{anyhow, Result};

use crate::components::acceleration::Acceleration;

pub struct ResetAcceleration;

impl ResetAcceleration {
    pub fn run(self, world: &World) -> Result<()> {
        let query = world.query().with_component::<Acceleration>().run();
        let accelerations = query[0].clone();
        for wrapped_acceleration in accelerations {
            let mut borrowed_acceleration = wrapped_acceleration.borrow_mut();
            let acceleration = borrowed_acceleration
                .downcast_mut::<Acceleration>()
                .ok_or_else(|| anyhow!("Error downcasting to acceleration"))?;

            acceleration.clear();
        }
        Ok(())
    }
}
