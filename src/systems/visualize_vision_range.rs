use std::ops::Deref;

use bbecs::World;
use eyre::Result;
use ggez::{
    graphics::{self, DrawMode, DrawParam, MeshBuilder, WHITE},
    Context,
};

use crate::components::{
    draw_vision_range::DrawVisionRange, location::Location, vision_range::VisionRange,
};

pub struct VisualizeVisionRange;

impl VisualizeVisionRange {
    pub fn run(self, world: &World, context: &mut Context) -> Result<()> {
        let query = world
            .query()
            .with_component::<Location>()
            .with_component::<VisionRange>()
            .with_component::<DrawVisionRange>()
            .run();

        for (index, wrapped_location) in query[0].iter().enumerate() {
            let borrowed_location = wrapped_location.borrow();
            let borrowed_vision_range = query[1][index].borrow();
            let location = borrowed_location.downcast_ref::<Location>().unwrap();
            let vision_range = borrowed_vision_range.downcast_ref::<VisionRange>().unwrap();

            let mesh = MeshBuilder::new()
                .circle(
                    DrawMode::stroke(1.0),
                    location.to_mint_vector2(),
                    **vision_range,
                    0.1,
                    WHITE,
                )
                .build(context)?;

            graphics::draw(context, &mesh, DrawParam::default())?;
        }

        Ok(())
    }
}
