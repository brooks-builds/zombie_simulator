use bbecs::World;
use eyre::Result;

use crate::{
    components::{
        alive::Alive, color::Color, human::Human, location::Location, velocity::Velocity,
        zombie::Zombie,
    },
    resources::{dying_color::DyingColor, entity_size::EntitySize},
};

pub struct Attack;

impl Attack {
    pub fn run(self, world: &mut World) -> Result<()> {
        let entity_size = world.get_resource::<EntitySize>().unwrap();
        let dying_color = world.get_resource::<DyingColor>().unwrap();
        let zombies_query = match world
            .query()
            .with_component::<Location>()
            .with_component::<Zombie>()
            .with_component::<Velocity>()
            .run()
        {
            Ok(query) => query,
            Err(error) => {
                return match error.downcast_ref::<bbecs::errors::Errors>() {
                    Some(bbecs::errors::Errors::ComponentNotFound) => Ok(()),
                    _ => Err(error),
                }
            }
        };

        let humans_query = world
            .query()
            .with_component::<Location>()
            .with_component::<Velocity>()
            .with_component::<Human>()
            .with_component::<Alive>()
            .with_component::<Color>()
            .run()?;

        for (zombie_index, wrapped_zombie_location) in zombies_query.1[0].iter().enumerate() {
            let borrowed_zombie_location = wrapped_zombie_location.borrow();
            let mut borrowed_zombie_velocity = zombies_query.1[2][zombie_index].borrow_mut();
            let zombie_location = borrowed_zombie_location.downcast_ref::<Location>().unwrap();
            let zombie_velocity = borrowed_zombie_velocity.downcast_mut::<Velocity>().unwrap();

            for (human_index, wrapped_human_location) in humans_query.1[0].iter().enumerate() {
                let borrowed_human_location = wrapped_human_location.borrow();
                let mut borrowed_human_velocity = humans_query.1[1][human_index].borrow_mut();
                let human_location = borrowed_human_location.downcast_ref::<Location>().unwrap();
                let human_velocity = borrowed_human_velocity.downcast_mut::<Velocity>().unwrap();

                if zombie_location.distance(&**human_location) < **entity_size {
                    let mut borrowed_human_color = humans_query.1[4][human_index].borrow_mut();
                    let human_color = borrowed_human_color.downcast_mut::<Color>().unwrap();
                    human_color.set(**dying_color);
                    zombie_velocity.clear();
                    human_velocity.clear();
                    world.remove_component::<Alive>(humans_query.0[human_index]);
                    return Ok(());
                }
            }
        }
        Ok(())
    }
}
