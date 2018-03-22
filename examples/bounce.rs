extern crate ggez;
extern crate invaders;
extern crate rand;
extern crate specs;

use std::f32::consts::PI;

use ggez::*;
use ggez::graphics::*;
use specs::*;
use invaders::*;
use invaders::plugins::*;

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_setup.title = String::from("Bounce - Rust Invaders!");
    c.window_setup.samples = conf::NumSamples::Four;
    c.window_setup.resizable = true;

    let ctx = &mut Context::load_from_conf("bounce", "ggez", c).unwrap();

    ctx.print_resource_stats();
    graphics::set_background_color(ctx, (0, 0, 0, 255).into());

    match MainState::new() {
        Err(e) => {
            println!("Could not load game!");
            println!("Error: {}", e);
        }
        Ok(ref mut state) => {
            {
                let (width, height) = graphics::get_size(ctx);
                let mut viewport = state.world.write_resource::<viewport::ViewportState>();
                viewport.update_screen(width as f32, height as f32);
            }
            event::run(ctx, state).unwrap();
        }
    }
}

struct MainState<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> MainState<'a, 'b> {
    fn new() -> GameResult<MainState<'a, 'b>> {
        let mut world = World::new();

        let mut dispatcher = DispatcherBuilder::new();
        let init_funcs = [
            init,
            viewport::init,
            collision::init,
            bounce::init,
            health_damage::init,
            simple_physics::init,
            position_motion::init,
            sprites::init,
            despawn::init,
        ];
        for init_func in init_funcs.iter() {
            dispatcher = init_func(&mut world, dispatcher);
        }

        for _idx in 0..10 {
            spawn_asteroid(&mut world);
        }

        Ok(MainState {
            world,
            dispatcher: dispatcher.build(),
        })
    }
}

impl<'a, 'b> event::EventHandler for MainState<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        update_delta_time(&mut self.world, ctx);
        if rand::random::<f32>() < 0.3 {
            spawn_asteroid(&mut self.world);
        }
        self.dispatcher.dispatch(&self.world.res);
        self.world.maintain();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_background_color(ctx, graphics::BLACK);
        graphics::clear(ctx);
        graphics::set_color(ctx, graphics::WHITE)?;
        sprites::draw(&mut self.world, ctx)?;
        graphics::present(ctx);
        Ok(())
    }

    fn resize_event(&mut self, _ctx: &mut Context, width: u32, height: u32) {
        let mut viewport = self.world.write_resource::<viewport::ViewportState>();
        viewport.update_screen(width as f32, height as f32);
    }
}

const HW: f32 = viewport::PLAYFIELD_WIDTH / 2.0;
const HH: f32 = viewport::PLAYFIELD_HEIGHT / 2.0;

fn spawn_asteroid(world: &mut World) {
    let x = if rand::random::<f32>() > 0.5 {
        0.0 - HW + 100.0
    } else {
        HW - 100.0
    };
    let xv = if rand::random::<f32>() > 0.5 {
        200.0 + rand::random::<f32>() * 500.0
    } else {
        0.0 - 200.0 - rand::random::<f32>() * 500.0
    };
    let y = (0.0 - HH) + (viewport::PLAYFIELD_HEIGHT / 12.0) * (rand::random::<f32>() * 12.0);

    let size = 25.0 + 150.0 * rand::random::<f32>();

    {
        let positions = world.read::<position_motion::Position>();
        let collidables = world.read::<collision::Collidable>();
        if !collision::is_empty_at(&positions, &collidables, x, y, size) {
            return;
        }
    }

    world
        .create_entity()
        .with(position_motion::Position {
            x,
            y,
            ..Default::default()
        })
        .with(position_motion::Velocity {
            x: xv,
            r: PI * rand::random::<f32>(),
            ..Default::default()
        })
        .with(despawn::Timeout(7.0))
        .with(collision::Collidable { size })
        .with(bounce::BounceOnCollision {
            ..Default::default()
        })
        .with(sprites::Sprite {
            shape: sprites::Shape::Asteroid,
            scale: Point2::new(size, size),
            ..Default::default()
        })
        .with(despawn::DespawnBounds(Rect::new(
            -800.0,
            -450.0,
            1600.0,
            900.0,
        )))
        //.with(health_damage::DamageOnCollision {
        //    damage: 100.0,
        //    ..Default::default()
        //})
        //.with(health_damage::Health(100.0))
        //.with(despawn::Tombstone)
        .build();
}
