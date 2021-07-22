use bbecs::World;
use eyre::Result;
use ggez::{
    graphics::{self, DrawParam},
    Context,
};

use crate::{
    components::{color::Color, location::Location},
    resources::mesh::Mesh,
};

pub struct DrawEntities;

impl DrawEntities {
    pub fn run(self, world: &World, context: &mut Context) -> Result<()> {
        let mesh = world.get_resource::<Mesh>().unwrap();
        let query = world
            .query()
            .with_component::<Location>()
            .with_component::<Color>()
            .run()?;
        for (index, location) in query.first().unwrap().iter().enumerate() {
            let borrowed_location = location.borrow();
            let location = borrowed_location.downcast_ref::<Location>().unwrap();
            let borrowed_color = query[1][index].borrow();
            let color = borrowed_color.downcast_ref::<Color>().unwrap();

            graphics::draw(
                context,
                &mesh.0,
                DrawParam::new()
                    .dest(location.to_mint_vector2())
                    .color(**color),
            )?;
        }
        Ok(())
    }
}
