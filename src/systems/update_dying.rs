use bbecs::{errors::Errors, World};
use eyre::Result;

use crate::components::dying::Dying;

#[derive(Debug)]
pub struct UpdateDying;

impl UpdateDying {
    pub fn run(self, world: &World) -> Result<()> {
        let query = match world.query().with_component::<Dying>().run() {
            Ok(query) => query,
            Err(error) => {
                return match error.downcast_ref::<Errors>() {
                    Some(Errors::ComponentNotFound) => Ok(()),
                    _ => Err(error),
                }
            }
        };

        for wrapped_dying in query.1[0].iter() {
            let mut borrowed_dying = wrapped_dying.borrow_mut();
            let dying = borrowed_dying.downcast_mut::<Dying>().unwrap();
            dying.decrement_if_possible();
        }

        Ok(())
    }
}
