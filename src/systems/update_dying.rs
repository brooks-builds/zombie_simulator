use bbecs::World;
use eyre::Result;

use crate::components::dying::Dying;

#[derive(Debug)]
pub struct UpdateDying;

impl UpdateDying {
    pub fn run(self, world: &World) -> Result<()> {
        let query = &world.query().with_component::<Dying>().run()?.1[0];

        for wrapped_dying in query {
            let mut borrowed_dying = wrapped_dying.borrow_mut();
            let dying = borrowed_dying.downcast_mut::<Dying>().unwrap();
            dying.decrement();
        }

        Ok(())
    }
}
