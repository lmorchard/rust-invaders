extern crate ggez;
extern crate invaders;
extern crate rand;
extern crate specs;
#[macro_use]
extern crate specs_derive;

use std::f32::consts::PI;
use std::fmt;
use std::time::SystemTime;

use ggez::*;
use ggez::event::{Axis, Button, Keycode, Mod};
use ggez::graphics::{DrawParam, Mesh, Point2};

use specs::{Dispatcher, DispatcherBuilder, Fetch, Join, ReadStorage, System, World, WriteStorage};

use invaders::graphics::meshes;

#[derive(Debug)]
struct DeltaTime(pub f32);

#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
    r: f32,
}

#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32,
    r: f32,
}

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
        WriteStorage<'a, Position>,
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
    paused: bool,
    input_left: bool,
    input_right: bool,
    input_up: bool,
    input_down: bool,
    input_fire: bool,
    input_special: bool,
}

impl<'a, 'b> fmt::Display for MainState<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "paused: {}; input_left: {}; input_right: {}; input_fire: {}",
            self.paused, self.input_left, self.input_right, self.input_fire
        )
    }
}

impl<'a, 'b> MainState<'a, 'b> {
    fn new(ctx: &mut Context) -> GameResult<MainState<'a, 'b>> {
        let mut world = World::new();

        world.add_resource(DeltaTime(0.016));
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Sprite>();

        for _idx in 0..25 {
            let scale = 100.0 * rand::random::<f32>();
            world
                .create_entity()
                .with(Position {
                    x: 640.0 * rand::random::<f32>(),
                    y: 480.0 * rand::random::<f32>(),
                    r: PI * rand::random::<f32>(),
                })
                .with(Velocity {
                    x: 100.0 - 200.0 * rand::random::<f32>(),
                    y: 100.0 - 200.0 * rand::random::<f32>(),
                    r: (PI * 0.5) - PI * rand::random::<f32>(),
                })
                .with(Sprite {
                    mesh: meshes::player(ctx, 0.01),
                    offset: Point2::new(0.5, 0.5),
                    scale: Point2::new(scale, scale),
                })
                .build();
        }

        let dispatcher = DispatcherBuilder::new()
            .add(MotionSystem, "motion", &[])
            .build();

        Ok(MainState {
            world,
            dispatcher,
            last_time: SystemTime::now(),
            paused: false,
            input_left: false,
            input_right: false,
            input_up: false,
            input_down: false,
            input_fire: false,
            input_special: false,
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

    fn focus_event(&mut self, _ctx: &mut Context, gained: bool) {
        if gained {
            self.paused = false;
        } else {
            self.paused = true;
        }
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool,
    ) {
        match keycode {
            Keycode::Up => self.input_up = true,
            Keycode::W => self.input_up = true,
            Keycode::Down => self.input_down = true,
            Keycode::S => self.input_down = true,
            Keycode::Left => self.input_left = true,
            Keycode::A => self.input_left = true,
            Keycode::Right => self.input_right = true,
            Keycode::D => self.input_right = true,
            Keycode::Space => self.input_fire = true,
            Keycode::Return => self.input_special = true,
            _ => (),
        };
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Up => self.input_up = false,
            Keycode::W => self.input_up = false,
            Keycode::Down => self.input_down = false,
            Keycode::S => self.input_down = false,
            Keycode::Left => self.input_left = false,
            Keycode::A => self.input_left = false,
            Keycode::Right => self.input_right = false,
            Keycode::D => self.input_right = false,
            Keycode::Space => self.input_fire = false,
            Keycode::Return => self.input_special = false,
            _ => (),
        };
    }

    fn controller_button_down_event(&mut self, _ctx: &mut Context, btn: Button, instance_id: i32) {
        println!(
            "Controller button pressed: {:?} Controller_Id: {}",
            btn, instance_id
        );
    }

    fn controller_button_up_event(&mut self, _ctx: &mut Context, btn: Button, instance_id: i32) {
        println!(
            "Controller button released: {:?} Controller_Id: {}",
            btn, instance_id
        );
    }

    fn controller_axis_event(
        &mut self,
        _ctx: &mut Context,
        axis: Axis,
        value: i16,
        instance_id: i32,
    ) {
        println!(
            "Axis Event: {:?} Value: {} Controller_Id: {}",
            axis, value, instance_id
        );
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();

    ctx.print_resource_stats();
    graphics::set_background_color(ctx, (0, 0, 0, 255).into());

    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
