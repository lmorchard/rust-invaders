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

    match MainState::new(ctx) {
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
    fn new(_ctx: &mut Context) -> GameResult<MainState<'a, 'b>> {
        let mut world = World::new();

        let mut dispatcher = DispatcherBuilder::new();
        let init_funcs = [init, viewport::init, position_motion::init, sprites::init];
        for init_func in init_funcs.iter() {
            dispatcher = init_func(&mut world, dispatcher);
        }

        for _idx in 0..25 {
            spawn(&mut world);
        }

        Ok(MainState {
            world,
            dispatcher: dispatcher.build(),
        })
    }
}

impl<'a, 'b> event::EventHandler for MainState<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        viewport::update(&mut self.world, ctx)?;

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

    fn resize_event(&mut self, _ctx: &mut Context, width: u32, height: u32) {
        let mut viewport = self.world.write_resource::<viewport::ViewportState>();
        viewport.update_screen(width as f32, height as f32);
    }
}

fn spawn(world: &mut World) {
    let scale = 50.0 + (50.0 * rand::random::<f32>());
    world
        .create_entity()
        .with(position_motion::Position {
            x: viewport::PLAYFIELD_WIDTH * rand::random::<f32>()
                - (viewport::PLAYFIELD_WIDTH / 2.0),
            y: viewport::PLAYFIELD_HEIGHT * rand::random::<f32>()
                - (viewport::PLAYFIELD_HEIGHT / 2.0),
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
