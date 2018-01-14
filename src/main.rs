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
use ggez::graphics::{DrawMode, DrawParam, Point2, Rect};

use specs::{Dispatcher, DispatcherBuilder, Join, World};

use invaders::graphics::meshes;
use invaders::components::*;
use invaders::systems::*;
use invaders::resources::*;

const PLAYFIELD_WIDTH: f32 = 1600.0;
const PLAYFIELD_HEIGHT: f32 = 900.0;
const PLAYFIELD_RATIO: f32 = PLAYFIELD_WIDTH / PLAYFIELD_HEIGHT;

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_setup.title = String::from("Rust Invaders!");
    c.window_setup.samples = conf::NumSamples::Four;
    c.window_setup.resizable = true;

    let ctx = &mut Context::load_from_conf("invaders", "ggez", c).unwrap();

    ctx.print_resource_stats();

    let state = &mut MainState::new(ctx).unwrap();
    let (width, height) = graphics::get_size(ctx);
    state.update_screen_coordinates(ctx, width, height);

    event::run(ctx, state).unwrap();
}

struct MainState<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    coords: Rect,
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
        world.register::<PositionBounds>();
        world.register::<Velocity>();
        world.register::<Thruster>();
        world.register::<ThrusterSet>();
        world.register::<Friction>();
        world.register::<SpeedLimit>();
        world.register::<PlayerControl>();
        world.register::<Sprite>();

        let dispatcher = DispatcherBuilder::new()
            .add(MotionSystem, "motion", &[])
            .add(PositionBoundsSystem, "position_bounds", &[])
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
            zoom: 1.0,
            coords: Rect::new(0.0, 0.0, PLAYFIELD_WIDTH, PLAYFIELD_HEIGHT),
        })
    }

    fn update_screen_coordinates(&mut self, ctx: &mut Context, width: u32, height: u32) {
        let width = width as f32;
        let height = height as f32;

        let screen_ratio = width / height;
        let fit_ratio = if screen_ratio < PLAYFIELD_RATIO {
            PLAYFIELD_WIDTH / width
        } else {
            PLAYFIELD_HEIGHT / height
        };

        let (visible_width, visible_height) = (
            width as f32 * fit_ratio * (1.0 / self.zoom),
            height as f32 * fit_ratio * (1.0 / self.zoom),
        );
        let (visible_x, visible_y) = (0.0 - (visible_width / 2.0), 0.0 - (visible_height / 2.0));

        self.coords = Rect::new(visible_x, visible_y, visible_width, visible_height);

        graphics::set_screen_coordinates(ctx, self.coords.clone()).unwrap();
    }
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

        graphics::set_color(ctx, graphics::WHITE)?;

        graphics::rectangle(
            ctx,
            DrawMode::Line(1.0),
            Rect::new(
                0.0 - (PLAYFIELD_WIDTH / 2.0),
                0.0 - (PLAYFIELD_HEIGHT / 2.0),
                PLAYFIELD_WIDTH,
                PLAYFIELD_HEIGHT,
            ),
        ).unwrap();

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

        // Hacky screen shake (for future reference):
        //let mut coords = graphics::get_screen_coordinates(ctx);
        //coords.x = self.coords.x + (5.0 - 10.0 * rand::random::<f32>());
        //coords.y = self.coords.y + (5.0 - 10.0 * rand::random::<f32>());
        //graphics::set_screen_coordinates(ctx, coords).unwrap();

        graphics::present(ctx);

        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut Context, width: u32, height: u32) {
        println!("Resized screen to {}, {}", width, height);
        self.update_screen_coordinates(ctx, width, height);
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

fn spawn_player(ctx: &mut Context, world: &mut World) {
    world
        .create_entity()
        .with(Position {
            x: 0.0,
            y: (PLAYFIELD_HEIGHT / 2.0) - 100.0,
            r: 0.0,
        })
        .with(PositionBounds(Rect::new(
            0.0 - PLAYFIELD_WIDTH / 2.0 + 50.0,
            0.0 - PLAYFIELD_HEIGHT / 2.0 + 50.0,
            PLAYFIELD_WIDTH - 100.0,
            PLAYFIELD_HEIGHT - 100.0,
        )))
        .with(Velocity {
            x: 0.0,
            y: 0.0,
            r: 0.0,
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
