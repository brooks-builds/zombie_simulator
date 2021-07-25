use bbecs::{errors::Errors, World};
use eyre::Result;

use crate::{
    components::{
        color::Color, dying::Dying, human::Human, speed::Speed, vision_range::VisionRange,
        zombie::Zombie,
    },
    resources::{
        zombie_color::ZombieColor, zombie_speed::ZombieSpeed,
        zombie_vision_range::ZombieVisionRange,
    },
};

#[derive(Debug)]
pub struct TurnIntoZombie;

impl TurnIntoZombie {
    pub fn run(self, world: &mut World) -> Result<()> {
        let (entity_indexes, query) = match world
            .query()
            .with_component::<Dying>()
            .with_component::<Color>()
            .with_component::<Speed>()
            .with_component::<VisionRange>()
            .run()
        {
            Ok(query) => query,
            Err(error) => {
                return match error.downcast_ref::<Errors>() {
                    Some(Errors::ComponentNotFound) => Ok(()),
                    _ => Err(error),
                }
            }
        };
        let zombie_color = world
            .get_resource::<ZombieColor>()
            .unwrap()
            .get_ggez_color();
        let zombie_vision_range = world.get_resource::<ZombieVisionRange>().unwrap().get();
        let zombie_speed = world.get_resource::<ZombieSpeed>().unwrap().get();

        for (index, wrapped_dying) in query[0].iter().enumerate() {
            let borrowed_dying = wrapped_dying.borrow();
            let dying = borrowed_dying.downcast_ref::<Dying>().unwrap();

            if !dying.is_dead() {
                continue;
            }

            world.remove_component::<Human>(entity_indexes[index]);

            let mut borrowed_color = query[1][index].borrow_mut();
            let color = borrowed_color.downcast_mut::<Color>().unwrap();
            color.set(zombie_color);

            let mut borrowed_speed = query[2][index].borrow_mut();
            let speed = borrowed_speed.downcast_mut::<Speed>().unwrap();
            speed.set(zombie_speed);

            let mut borrowed_vision_range = query[3][index].borrow_mut();
            let vision_range = borrowed_vision_range.downcast_mut::<VisionRange>().unwrap();
            vision_range.set(zombie_vision_range);

            world.add_component_to_entity(Zombie, entity_indexes[index]);
        }
        Ok(())
    }
}
