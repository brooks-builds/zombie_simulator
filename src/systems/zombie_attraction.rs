use bbecs::World;
use eyre::Result;

pub struct ZombieAttraction;

impl ZombieAttraction {
    /// Get the zombies and humans, and then for each zombie have it move towards the closest
    /// human that it can see.
    pub fn run(self, world: &World) -> Result<()> {
        Ok(())
    }
}
