use eyre::Result;
use ggez::{
    graphics::{DrawMode, MeshBuilder, WHITE},
    Context,
};
#[derive(Debug)]
pub struct Mesh(pub ggez::graphics::Mesh);

impl Mesh {
    pub fn new(radius: f32, context: &mut Context) -> Result<Mesh> {
        let mesh = MeshBuilder::new()
            .circle(DrawMode::fill(), [0.0, 0.0], radius, 0.1, WHITE)
            .build(context)?;
        Ok(Self(mesh))
    }
}
