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

    let ctx = &mut Context::load_from_conf("invaders", "ggez", c).unwrap();

    ctx.print_resource_stats();

    let state = &mut MainState::new().unwrap();
    let (width, height) = graphics::get_size(ctx);
    update_screen_coordinates(ctx, state.zoom, width, height);

    event::run(ctx, state).unwrap();
}

struct MainState<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    zoom: f32,
}

impl<'a, 'b> MainState<'a, 'b> {
    fn new() -> GameResult<MainState<'a, 'b>> {
        let mut world = World::new();

        let dispatcher = DispatcherBuilder::new();
        let dispatcher = init(&mut world, dispatcher);
        let dispatcher = collision::init(&mut world, dispatcher);
        let dispatcher = bounce::init(&mut world, dispatcher);
        let dispatcher = health_damage::init(&mut world, dispatcher);
        let dispatcher = simple_physics::init(&mut world, dispatcher);
        let dispatcher = position_motion::init(&mut world, dispatcher);
        let dispatcher = sprites::init(&mut world, dispatcher);
        let dispatcher = despawn::init(&mut world, dispatcher);
        let dispatcher = dispatcher.build();

        for _idx in 0..10 {
            spawn_asteroid(&mut world);
        }

        Ok(MainState {
            world,
            dispatcher,
            zoom: 1.0,
        })
    }
}

impl<'a, 'b> event::EventHandler for MainState<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        update_delta_time(&mut self.world, ctx);
        if rand::random::<f32>() < 0.3 {
            spawn_asteroid(&mut self.world);
        }
        self.dispatcher.dispatch(&mut self.world.res);
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

    fn resize_event(&mut self, ctx: &mut Context, width: u32, height: u32) {
        update_screen_coordinates(ctx, self.zoom, width, height);
    }
}

const HW: f32 = PLAYFIELD_WIDTH / 2.0;
const HH: f32 = PLAYFIELD_HEIGHT / 2.0;

fn spawn_asteroid(world: &mut World) {
    let x;
    let xv;
    if rand::random::<f32>() > 0.5 {
        x = (0.0 - HW + 100.0);
        xv = 200.0 + rand::random::<f32>() * 500.0;
    } else {
        x = HW - 100.0;
        xv = 200.0 - rand::random::<f32>() * 500.0;
    }
    let y = (0.0 - HH) + (PLAYFIELD_HEIGHT / 12.0) * (rand::random::<f32>() * 12.0);

    let size = 25.0 + 150.0 * rand::random::<f32>();
    world
        .create_entity()
        .with(position_motion::Position {
            x: x,
            y: y,
            ..Default::default()
        })
        .with(position_motion::Velocity {
            x: xv,
            r: PI * rand::random::<f32>(),
            ..Default::default()
        })
        .with(despawn::Timeout(7.0))
        .with(collision::Collidable { size: size })
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
        .with(despawn::Tombstone)
        .build();
}
