use bbecs::World;
use eyre::Result;

use crate::{
    components::{acceleration::Acceleration, human::Human, speed::Speed},
    data_structures::vector2::Vector2,
};

pub struct RandomlyWalk;

impl RandomlyWalk {
    pub fn run(self, world: &World) -> Result<()> {
        let query = world
            .query()
            .with_component::<Acceleration>()
            .with_component::<Speed>()
            .with_component::<Human>()
            .run();
        let accelerations = &query[0];
        let speeds = &query[1];
        for (index, wrapped_acceleration) in accelerations.iter().enumerate() {
            let mut borrowed_acceleration = wrapped_acceleration.borrow_mut();
            let borrowed_speed = speeds[index].borrow();
            let acceleration = borrowed_acceleration
                .downcast_mut::<Acceleration>()
                .ok_or_else(|| eyre::anyhow!("error downcasting into acceleration"))?;
            let speed = borrowed_speed
                .downcast_ref::<Speed>()
                .ok_or_else(|| eyre::anyhow!("error downcasting into speed"))?;
            let random_direction = Vector2::new_random_range(-**speed, **speed);

            **acceleration += random_direction;
        }
        Ok(())
    }
}
