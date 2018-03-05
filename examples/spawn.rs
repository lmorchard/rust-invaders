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
    c.window_setup.title = String::from("Spawn - Rust Invaders!");
    c.window_setup.samples = conf::NumSamples::Eight;
    c.window_setup.resizable = true;

    let ctx = &mut Context::load_from_conf("spawn", "ggez", c).unwrap();

    ctx.print_resource_stats();
    graphics::set_background_color(ctx, (0, 0, 0, 255).into());

    let state = &mut MainState::new(ctx).unwrap();
    let (width, height) = graphics::get_size(ctx);
    update_screen_coordinates(ctx, 1.0, width, height);
    event::run(ctx, state).unwrap();
}

struct MainState<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> MainState<'a, 'b> {
    fn new(_ctx: &mut Context) -> GameResult<MainState<'a, 'b>> {
        let mut world = World::new();

        let dispatcher = DispatcherBuilder::new();
        let dispatcher = init(&mut world, dispatcher);
        let dispatcher = position_motion::init(&mut world, dispatcher);
        let dispatcher = sprites::init(&mut world, dispatcher);
        let dispatcher = dispatcher.build();

        for _idx in 0..25 {
            spawn(&mut world);
        }

        Ok(MainState { world, dispatcher })
    }
}

impl<'a, 'b> event::EventHandler for MainState<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        update_delta_time(&mut self.world, ctx);

        self.dispatcher.dispatch(&self.world.res);

        if rand::random::<f32>() < 0.01 {
            spawn(&mut self.world);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        sprites::draw(&mut self.world, ctx)?;
        graphics::present(ctx);
        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut Context, width: u32, height: u32) {
        update_screen_coordinates(ctx, 1.0, width, height);
    }
}

fn spawn(world: &mut World) {
    let scale = 50.0 + (50.0 * rand::random::<f32>());
    world
        .create_entity()
        .with(position_motion::Position {
            x: PLAYFIELD_WIDTH * rand::random::<f32>() - (PLAYFIELD_WIDTH / 2.0),
            y: PLAYFIELD_HEIGHT * rand::random::<f32>() - (PLAYFIELD_HEIGHT / 2.0),
            r: PI * rand::random::<f32>(),
        })
        .with(position_motion::Velocity {
            x: 100.0 - 200.0 * rand::random::<f32>(),
            y: 100.0 - 200.0 * rand::random::<f32>(),
            r: (PI * 0.5) - PI * rand::random::<f32>(),
        })
        .with(sprites::Sprite {
            shape: if rand::random::<f32>() > 0.5 {
                sprites::Shape::Player
            } else {
                sprites::Shape::Asteroid
            },
            offset: Point2::new(0.5, 0.5),
            scale: Point2::new(scale, scale),
        })
        .build();
}
