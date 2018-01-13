extern crate ggez;
extern crate invaders;
#[macro_use]
extern crate maplit;
extern crate rand;
extern crate specs;

use std::f32::consts::PI;
use std::fmt;

use ggez::*;
use ggez::event::{Axis, Button, Keycode, Mod};
use ggez::graphics::{DrawParam, Point2};

use specs::{Dispatcher, DispatcherBuilder, Join, World};

use invaders::graphics::meshes;
use invaders::components::*;
use invaders::systems::*;
use invaders::resources::*;

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_setup.title = String::from("Rust Invaders!");
    c.window_setup.samples = conf::NumSamples::Eight;
    c.window_setup.resizable = true;

    let ctx = &mut Context::load_from_conf("invaders", "ggez", c).unwrap();

    ctx.print_resource_stats();

    let projection = graphics::get_projection(ctx);
    println!("PROJECTION {}", projection);

    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}

struct MainState<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    paused: bool,
    zoom: f32,
}

impl<'a, 'b> fmt::Display for MainState<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inputs = self.world.read_resource::<Inputs>();
        write!(f, "paused: {}; inputs: {:?}", self.paused, *inputs)
    }
}

impl<'a, 'b> MainState<'a, 'b> {
    fn new(ctx: &mut Context) -> GameResult<MainState<'a, 'b>> {
        // let resolutions = ggez::graphics::get_fullscreen_modes(ctx, 0)?;

        let mut world = World::new();

        world.add_resource(DeltaTime(0.016));
        world.add_resource(Inputs {
            left: false,
            right: false,
            up: false,
            down: false,
            fire: false,
            special: false,
        });

        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Thruster>();
        world.register::<ThrusterSet>();
        world.register::<Friction>();
        world.register::<SpeedLimit>();
        world.register::<PlayerControl>();
        world.register::<Sprite>();

        let dispatcher = DispatcherBuilder::new()
            .add(MotionSystem, "motion", &[])
            .add(ThrusterSystem, "thruster", &[])
            .add(ThrusterSetSystem, "thruster_set", &[])
            .add(PlayerControlSystem, "player_control", &[])
            .add(SpeedLimitSystem, "speed_limit", &[])
            .add(FrictionSystem, "friction", &[])
            .build();

        spawn_player(ctx, &mut world);

        Ok(MainState {
            world,
            dispatcher,
            paused: false,
            zoom: 2.0,
        })
    }
}

fn spawn_player(ctx: &mut Context, world: &mut World) {
    world
        .create_entity()
        .with(Position {
            x: 0.0,
            y: 0.0,
            r: 0.0,
        })
        .with(Velocity {
            x: 0.0,
            y: 0.0,
            r: 0.0, // PI / 3.0,
        })
        .with(SpeedLimit(800.0))
        .with(Friction(6000.0))
        .with(ThrusterSet(hashmap!{
            "longitudinal" => Thruster {
                thrust: 10000.0,
                throttle: 0.0,
                angle: 0.0,
            },
            "lateral" => Thruster {
                thrust: 12500.0,
                throttle: 0.0,
                angle: PI * 0.5,
            },
        }))
        .with(Sprite {
            offset: Point2::new(0.5, 0.5),
            mesh: meshes::player(ctx, 1.0 / 50.0),
            scale: Point2::new(50.0, 50.0),
        })
        .with(PlayerControl)
        .build();
}

impl<'a, 'b> event::EventHandler for MainState<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        {
            let dt = ggez::timer::get_delta(ctx);
            let mut delta = self.world.write_resource::<DeltaTime>();
            *delta = DeltaTime(dt.as_secs() as f32 + dt.subsec_nanos() as f32 * 1e-9);
        }

        self.dispatcher.dispatch(&mut self.world.res);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_background_color(ctx, graphics::BLACK);
        graphics::clear(ctx);

        let (width, height) = graphics::get_size(ctx);
        let (z_width, z_height) = (width as f32 * self.zoom, height as f32 * self.zoom);
        let new_rect = graphics::Rect::new(
            0.0 - (z_width / 2.0),
            0.0 - (z_height / 2.0),
            z_width,
            z_height,
        );
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();

        // Hacky screen shake:
        // let mut coords = graphics::get_screen_coordinates(ctx);
        // coords.x = 10.0 * rand::random::<f32>();
        // coords.y = 10.0 * rand::random::<f32>();
        // graphics::set_screen_coordinates(ctx, coords).unwrap();

        graphics::set_color(ctx, graphics::WHITE)?;

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

    fn resize_event(&mut self, ctx: &mut Context, width: u32, height: u32) {
        println!("Resized screen to {}, {}", width, height);
        let new_rect = graphics::Rect::new(
            0.0,
            0.0,
            width as f32 * self.zoom,
            height as f32 * self.zoom,
        );
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
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
        let mut inputs = self.world.write_resource::<Inputs>();
        match keycode {
            Keycode::Up => inputs.up = true,
            Keycode::W => inputs.up = true,
            Keycode::Down => inputs.down = true,
            Keycode::S => inputs.down = true,
            Keycode::Left => inputs.left = true,
            Keycode::A => inputs.left = true,
            Keycode::Right => inputs.right = true,
            Keycode::D => inputs.right = true,
            Keycode::Space => inputs.fire = true,
            Keycode::Return => inputs.special = true,
            _ => (),
        };
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        let mut inputs = self.world.write_resource::<Inputs>();
        match keycode {
            Keycode::Up => inputs.up = false,
            Keycode::W => inputs.up = false,
            Keycode::Down => inputs.down = false,
            Keycode::S => inputs.down = false,
            Keycode::Left => inputs.left = false,
            Keycode::A => inputs.left = false,
            Keycode::Right => inputs.right = false,
            Keycode::D => inputs.right = false,
            Keycode::Space => inputs.fire = false,
            Keycode::Return => inputs.special = false,
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
