extern crate invaders;
extern crate ggez;
extern crate specs;
#[macro_use]
extern crate specs_derive;

use std::f32::consts::PI;
use std::time::{
    SystemTime,
};
use ggez::*;
use ggez::graphics::{
    DrawMode,
    DrawParam,
    Mesh,
    MeshBuilder,
    Point2,
};
use specs::{
    Dispatcher,
    DispatcherBuilder,
    Fetch,
    Join,
    ReadStorage,
    System,
    World,
    WriteStorage,
};

// TODO: Figure out if there's a better way to write this macro
macro_rules! points {
    ( $( $x:expr ), * ) => {
        {
            let mut temp_vec = Vec::new();
            $( temp_vec.push(Point2::new($x.0, $x.1)); )*
            temp_vec
        }
    };
}

#[derive(Debug)]
struct DeltaTime(pub f32);

#[derive(Component, Debug)]
struct Position { x: f32, y: f32, r: f32, }

#[derive(Component, Debug)]
struct Velocity { x: f32, y: f32, r: f32, }

#[derive(Component, Debug)]
struct Sprite {
    scale: Point2,
    offset: Point2,
    mesh: Mesh,
}

struct MotionSystem;

impl<'a> System<'a> for MotionSystem {
    type SystemData = (
        Fetch<'a, DeltaTime>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (delta, vel, mut pos) = data;
        let delta = delta.0;
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * delta;
            pos.y += vel.y * delta;
            pos.r += vel.r * delta;
        }
    }
}

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

        world.create_entity()
            .with(Position { x: 4.0, y: 200.0, r: 0.0 })
            .with(Velocity { x: 100.0, y: 0.0, r: PI / 2.0 })
            .with(Sprite {
                mesh: build_player_mesh(ctx, 0.01),
                offset: Point2::new(0.5, 0.5),
                scale: Point2::new(100.0, 100.0),
            })
            .build();

        world.create_entity()
            .with(Position { x: 4.0, y: 300.0, r: 0.0 })
            .with(Velocity { x: 150.0, y: 0.0, r: PI / 3.0 })
            .with(Sprite {
                mesh: build_player_mesh(ctx, 0.02),
                offset: Point2::new(0.5, 0.5),
                scale: Point2::new(50.0, 50.0),
            })
            .build();

        world.create_entity()
            .with(Position { x: 4.0, y: 100.0, r: 0.0 })
            .with(Velocity { x: 50.0, y: 0.0, r: PI / 3.0 })
            .with(Sprite {
                mesh: build_player_mesh(ctx, 0.02),
                offset: Point2::new(0.5, 0.5),
                scale: Point2::new(50.0, 50.0),
            })
            .build();

        let dispatcher = DispatcherBuilder::new()
            .add(MotionSystem, "motion", &[])
            .build();

        Ok(MainState {
            world,
            dispatcher,
            last_time: SystemTime::now(),
        })
    }
}

impl<'a, 'b> event::EventHandler for MainState<'a, 'b> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let now = SystemTime::now();
        let dt = now.duration_since(self.last_time).unwrap();
        self.last_time = now;

        {
            let mut delta = self.world.write_resource::<DeltaTime>();
            *delta = DeltaTime(dt.as_secs() as f32 + dt.subsec_nanos() as f32 * 1e-9);
        }

        self.dispatcher.dispatch(&mut self.world.res);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let entities = self.world.entities();
        let positions = self.world.read::<Position>();
        let sprites = self.world.read::<Sprite>();

        for (_ent, pos, spr) in (&*entities, &positions, &sprites).join() {
            graphics::draw_ex(ctx, &spr.mesh, DrawParam {
                dest: Point2::new(pos.x, pos.y),
                rotation: pos.r,
                offset: spr.offset,
                scale: spr.scale,
                .. Default::default()
            })?;
        }

        graphics::present(ctx);
        Ok(())
    }
}

fn build_player_mesh (ctx: &mut Context, line_width: f32) -> Mesh {
    MeshBuilder::new()
        .polygon(DrawMode::Line(line_width), &points![
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            (0.0, 1.0)
        ])
        .polygon(DrawMode::Line(line_width), &points![
            (0.5, 0.0),
            (1.0, 1.0),
            (0.0, 1.0)
        ])
        .line(&points![
            (0.4, 0.5),
            (0.6, 0.5)
        ], line_width)
        .line(&points![
            (0.5, 0.4),
            (0.5, 0.6)
        ], line_width)
        .build(ctx)
        .unwrap()
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();

    ctx.print_resource_stats();
    graphics::set_background_color(ctx, (0, 0, 0, 255).into());

    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
