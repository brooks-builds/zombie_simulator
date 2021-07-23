use bbecs::World;
use eyre::Result;

pub struct RunAwayFromZombies;

impl RunAwayFromZombies {
    /// For every human find the zombies that they can see and move away from them.
    pub fn run(self, world: &World) -> Result<()> {
        Ok(())
    }
}
