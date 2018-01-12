extern crate ggez;
extern crate invaders;
extern crate rand;
extern crate specs;

use std::f32::consts::PI;
use std::time::SystemTime;

use ggez::*;
use ggez::graphics::{DrawParam, Point2};

use specs::{Dispatcher, DispatcherBuilder, Join, World};

use invaders::graphics::meshes;
use invaders::components::*;
use invaders::systems::*;
use invaders::resources::*;

struct MainState<'a, 'b> {
    last_time: SystemTime,
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> MainState<'a, 'b> {
    fn new(ctx: &mut Context) -> GameResult<MainState<'a, 'b>> {
        let mut world = World::new();

        world.add_resource(DeltaTime(0.016));
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Sprite>();

        for _idx in 0..25 {
            spawn(ctx, &mut world);
        }

        let dispatcher = DispatcherBuilder::new()
            .add(MotionSystem, "motion", &[])
            .build();

        world
            .create_entity()
            .with(Position {
                x: 400.0,
                y: 500.0,
                r: 0.0,
            })
            .with(Velocity {
                x: 0.0,
                y: 0.0,
                r: 0.0,
            })
            .with(Sprite {
                mesh: meshes::player(ctx, 0.01),
                offset: Point2::new(0.5, 0.5),
                scale: Point2::new(100.0, 100.0),
            })
            .build();

        Ok(MainState {
            world,
            dispatcher,
            last_time: SystemTime::now(),
        })
    }
}

fn spawn(ctx: &mut Context, world: &mut World) {
    let scale = 50.0 + (50.0 * rand::random::<f32>());
    world
        .create_entity()
        .with(Position {
            x: 800.0 * rand::random::<f32>(),
            y: 600.0 * rand::random::<f32>(),
            r: PI * rand::random::<f32>(),
        })
        .with(Velocity {
            x: 100.0 - 200.0 * rand::random::<f32>(),
            y: 100.0 - 200.0 * rand::random::<f32>(),
            r: (PI * 0.5) - PI * rand::random::<f32>(),
        })
        .with(Sprite {
            mesh: if rand::random::<f32>() > 0.5 {
                meshes::player(ctx, 1.0 / scale)
            } else {
                meshes::asteroid(ctx, 1.0 / scale)
            },
            offset: Point2::new(0.5, 0.5),
            scale: Point2::new(scale, scale),
        })
        .build();
}

impl<'a, 'b> event::EventHandler for MainState<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let now = SystemTime::now();
        let dt = now.duration_since(self.last_time).unwrap();
        self.last_time = now;

        {
            let mut delta = self.world.write_resource::<DeltaTime>();
            *delta = DeltaTime(dt.as_secs() as f32 + dt.subsec_nanos() as f32 * 1e-9);
        }

        self.dispatcher.dispatch(&mut self.world.res);

        if rand::random::<f32>() < 0.01 {
            spawn(ctx, &mut self.world);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let entities = self.world.entities();
        let positions = self.world.read::<Position>();
        let sprites = self.world.read::<Sprite>();

        for (_ent, pos, spr) in (&*entities, &positions, &sprites).join() {
            graphics::draw_ex(
                ctx,
                &spr.mesh,
                DrawParam {
                    dest: Point2::new(pos.x, pos.y),
                    rotation: pos.r,
                    offset: spr.offset,
                    scale: spr.scale,
                    ..Default::default()
                },
            )?;
        }

        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("spawn", "ggez", c).unwrap();

    ctx.print_resource_stats();
    graphics::set_background_color(ctx, (0, 0, 0, 255).into());

    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
