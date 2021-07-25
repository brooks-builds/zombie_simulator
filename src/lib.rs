use std::ops::Deref;

use bbecs::World;
use components::{
    acceleration::Acceleration, alive::Alive, color::Color, human::Human, location::Location,
    speed::Speed, velocity::Velocity, vision_range::VisionRange,
};
use config::Config;
use data_structures::vector2::Vector2;
use eyre::Result;
use ggez::{
    event::{EventHandler, MouseButton},
    graphics::{self, WHITE},
    timer, Context,
};
use resources::{
    background_color::BackgroundColor, clicked_location::ClickedLocation, dying_color::DyingColor,
    entity_size::EntitySize, mesh::Mesh, target_fps::TargetFps,
};
use systems::{
    add_zombie::AddZombie, attack::Attack, contain_entities_in_arena::ContainEntitiesInArena,
    draw_entities::DrawEntities, human_repulsion::HumanRepulsion, randomly_walk::RandomlyWalk,
    reset_acceleration::ResetAcceleration, run_away_from_zombies::RunAwayFromZombies,
    update_location::UpdateLocation, update_velocity::UpdateVelocity,
    visualize_vision_range::VisualizeVisionRange, zombie_attraction::ZombieAttraction,
};

pub mod components;
pub mod config;
pub mod data_structures;
pub mod resources;
pub mod systems;

#[derive(Debug, Default)]
pub struct MainState {
    world: World,
}

impl MainState {
    pub fn new(config: Config, context: &mut Context) -> Result<Self> {
        let mut world = World::new();
        let entity_size = EntitySize(config.size);
        world.add_resource(BackgroundColor(config.background_color));
        world.add_resource(config.arena_size);
        world.add_resource(Mesh::new(config.size, context)?);
        world.add_resource(TargetFps(config.target_fps));
        world.add_resource(entity_size);
        world.add_resource(ClickedLocation(None));
        world.add_resource(DyingColor(config.dying_color));

        #[allow(unused_variables)]
        for count in 0..config.humans {
            let registering_entity = world
                .register_entity()
                .with_component(Location(Vector2::new_random_range(
                    entity_size.half(),
                    config.arena_size.height - entity_size.half(),
                )))
                .with_component(Velocity(Vector2::new(0.0, 0.0)))
                .with_component(Acceleration(Vector2::new(0.0, 0.0)))
                .with_component(Speed(config.speed))
                .with_component(VisionRange(config.human_vision_range))
                .with_component(Color(WHITE))
                .with_component(Human)
                .with_component(Alive);

            // if count == 1 {
            //     registering_entity.with_component(DrawVisionRange);
            // }
        }
        Ok(Self { world })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> ggez::GameResult {
        let target_fps = **self.world.get_resource::<TargetFps>().unwrap();
        while timer::check_update_time(context, target_fps) {
            let randomly_walk = RandomlyWalk;
            let update_velocity = UpdateVelocity;
            let update_location = UpdateLocation;
            let reset_acceleration = ResetAcceleration;
            let contain_entities_in_arena = ContainEntitiesInArena;
            let human_repulsion = HumanRepulsion;
            let add_zombie = AddZombie;
            let zombie_attraction = ZombieAttraction;
            let run_away_from_zombies = RunAwayFromZombies;
            let attack = Attack;

            randomly_walk.run(&self.world).unwrap();
            update_velocity.run(&self.world).unwrap();
            update_location.run(&self.world).unwrap();
            reset_acceleration.run(&self.world).unwrap();
            contain_entities_in_arena.run(&self.world).unwrap();
            human_repulsion.run(&self.world).unwrap();
            add_zombie.run(&mut self.world);
            zombie_attraction.run(&self.world).unwrap();
            run_away_from_zombies.run(&self.world).unwrap();
            attack.run(&mut self.world).unwrap();
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> ggez::GameResult {
        let background_color = self.world.get_resource::<BackgroundColor>().unwrap();
        graphics::clear(context, *background_color.deref());

        let visualize_vision_range = VisualizeVisionRange;
        visualize_vision_range.run(&self.world, context).unwrap();
        let draw_entities = DrawEntities;
        draw_entities.run(&self.world, context).unwrap();

        graphics::present(context)
    }

    fn mouse_button_down_event(
        &mut self,
        _context: &mut Context,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) {
        if let MouseButton::Left = button {
            let clicked_location = self.world.get_resource_mut::<ClickedLocation>().unwrap();
            clicked_location.set(x, y);
        }
    }
}
