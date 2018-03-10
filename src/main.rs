extern crate ggez;
extern crate invaders;
#[macro_use]
extern crate maplit;
extern crate rand;
extern crate specs;

use std::f32::consts::PI;

use ggez::*;
use ggez::event::*;
use ggez::graphics::*;

use specs::*;

use invaders::*;
use invaders::plugins::*;

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_setup.title = String::from("Rust Invaders!");
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
    paused: bool,
    zoom: f32,
}

impl<'a, 'b> MainState<'a, 'b> {
    fn new() -> GameResult<MainState<'a, 'b>> {
        let mut world = World::new();

        let dispatcher = DispatcherBuilder::new();

        let dispatcher = init(&mut world, dispatcher);
        let dispatcher = metadata::init(&mut world, dispatcher);
        let dispatcher = guns::init(&mut world, dispatcher);
        let dispatcher = thruster::init(&mut world, dispatcher);
        let dispatcher = collision::init(&mut world, dispatcher);
        let dispatcher = bounce::init(&mut world, dispatcher);
        let dispatcher = health_damage::init(&mut world, dispatcher);
        let dispatcher = player_control::init(&mut world, dispatcher);
        let dispatcher = simple_physics::init(&mut world, dispatcher);
        let dispatcher = position_motion::init(&mut world, dispatcher);
        let dispatcher = sprites::init(&mut world, dispatcher);
        let dispatcher = despawn::init(&mut world, dispatcher);

        let dispatcher = dispatcher.add(CollisionMatchSystem, "collision_match", &[]);

        let dispatcher = dispatcher.build();

        spawn_planet(&mut world);

        spawn_player(&mut world);

        for _idx in 0..10 {
            spawn_asteroid(&mut world);
        }

        Ok(MainState {
            world,
            dispatcher,
            paused: false,
            zoom: 1.0,
        })
    }

    fn draw_backdrop(&mut self, ctx: &mut Context) {
        /*
        graphics::circle(
            ctx,
            DrawMode::Line(1.0),
            Point2::new(0.0, 2975.0),
            2600.0,
            3.0
        ).unwrap();

        graphics::rectangle(
            ctx,
            DrawMode::Line(1.0),
            Rect::new(
                0.0 - (PLAYFIELD_WIDTH / 2.0) - 1.0,
                0.0 - (PLAYFIELD_HEIGHT / 2.0) - 1.0,
                PLAYFIELD_WIDTH + 2.0,
                PLAYFIELD_HEIGHT + 2.0,
            ),
        ).unwrap();
        */
    }
}

impl<'a, 'b> event::EventHandler for MainState<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        update_delta_time(&mut self.world, ctx);

        if rand::random::<f32>() < 0.025 {
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

        self.draw_backdrop(ctx);

        sprites::draw(&mut self.world, ctx)?;

        graphics::present(ctx);
        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut Context, width: u32, height: u32) {
        update_screen_coordinates(ctx, self.zoom, width, height);
    }

    fn focus_event(&mut self, _ctx: &mut Context, gained: bool) {
        if gained {
            self.paused = false;
        } else {
            self.paused = true;
        }
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        player_control::key_down_event(&mut self.world, ctx, keycode, keymod, repeat);
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        player_control::key_up_event(&mut self.world, ctx, keycode, keymod, repeat);
    }

    fn controller_button_down_event(&mut self, ctx: &mut Context, btn: Button, instance_id: i32) {
        player_control::controller_button_down_event(&mut self.world, ctx, btn, instance_id);
    }

    fn controller_button_up_event(&mut self, ctx: &mut Context, btn: Button, instance_id: i32) {
        player_control::controller_button_up_event(&mut self.world, ctx, btn, instance_id);
    }

    fn controller_axis_event(
        &mut self,
        ctx: &mut Context,
        axis: Axis,
        value: i16,
        instance_id: i32,
    ) {
        player_control::controller_axis_event(&mut self.world, ctx, axis, value, instance_id);
    }
}

fn spawn_player(world: &mut World) {
    world
        .create_entity()
        .with(metadata::Name("player"))
        .with(metadata::Tags::new(vec!["player", "friend"]))
        .with(position_motion::Position {
            y: (PLAYFIELD_HEIGHT / 2.0) - 200.0,
            ..Default::default()
        })
        .with(position_motion::PositionBounds(Rect::new(
            0.0 - PLAYFIELD_WIDTH / 2.0 + 25.0,
            0.0 - PLAYFIELD_HEIGHT / 2.0 + 5.0,
            PLAYFIELD_WIDTH - 50.0,
            PLAYFIELD_HEIGHT - 10.0,
        )))
        .with(position_motion::Velocity {
            ..Default::default()
        })
        .with(simple_physics::SpeedLimit(800.0))
        .with(simple_physics::Friction(6000.0))
        .with(thruster::ThrusterSet(hashmap!{
            "longitudinal" => thruster::Thruster {
                thrust: 10000.0,
                throttle: 0.0,
                angle: 0.0,
            },
            "lateral" => thruster::Thruster {
                thrust: 12500.0,
                throttle: 0.0,
                angle: PI * 0.5,
            },
        }))
        .with(guns::Gun {
            period: 0.3,
            ..Default::default()
        })
        .with(collision::Collidable { size: 50.0 })
        .with(bounce::BounceOnCollision { mass: 5.0 })
        .with(sprites::Sprite {
            shape: sprites::Shape::Player,
            scale: Point2::new(50.0, 50.0),
            ..Default::default()
        })
        .with(player_control::PlayerControl)
        .build();
}

fn spawn_planet(world: &mut World) {
    world
        .create_entity()
        .with(metadata::Tags::new(vec!["planet", "friend"]))
        .with(position_motion::Position {
            x: 0.0,
            y: 1850.0,
            ..Default::default()
        })
        .with(position_motion::Velocity {
            r: PI * 0.015,
            ..Default::default()
        })
        .with(sprites::Sprite {
            shape: sprites::Shape::Planet,
            scale: Point2::new(3000.0, 3000.0),
            ..Default::default()
        })
        .with(collision::Collidable { size: 3000.0 })
        .with(simple_physics::SpeedLimit(0.0))
        .with(simple_physics::Friction(100000.0))
        .with(bounce::BounceOnCollision { mass: 100000.0 })
        .with(health_damage::Health(100000.0))
        .build();
}

const HW: f32 = PLAYFIELD_WIDTH / 2.0;
const HH: f32 = PLAYFIELD_HEIGHT / 2.0;

fn spawn_asteroid(world: &mut World) {
    let size = 25.0 + 150.0 * rand::random::<f32>();
    let x = 0.0 - HW + (PLAYFIELD_WIDTH / 8.0) * (rand::random::<f32>() * 8.0);
    let y = 0.0 - HH - size;

    if !collision::is_empty_at(world, x, y, size) {
        return;
    }

    world
        .create_entity()
        .with(metadata::Tags::new(vec!["asteroid", "enemy"]))
        .with(position_motion::Position { x, y, ..Default::default() })
        .with(position_motion::Velocity {
            x: 50.0 - 100.0 * rand::random::<f32>(),
            y: 50.0 + 100.0 * rand::random::<f32>(),
            r: PI * rand::random::<f32>()
        })
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
            0.0 - HW - 200.0,
            0.0 - HH - 200.0,
            PLAYFIELD_WIDTH + 400.0,
            PLAYFIELD_HEIGHT + 400.0,
        )))
        //.with(health_damage::DamageOnCollision {
        //    damage: 100.0,
        //    ..Default::default()
        //})
        .with(health_damage::Health(100.0))
        .with(despawn::Tombstone)
        .build();
}

pub struct CollisionMatchSystem;
impl<'a> System<'a> for CollisionMatchSystem {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, collision::Collisions>,
        FetchMut<'a, health_damage::DamageEventQueue>,
        ReadStorage<'a, metadata::Tags>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, collisions, mut damage, tags) = data;
        for (a_entity, a_tags) in (&*entities, &tags).join() {
            for &a_tag in &a_tags.0 {
                if let Some(ent_collisions) = collisions.get(&a_entity) {
                    for b_entity in ent_collisions.iter() {
                        if let Some(b_tags) = tags.get(*b_entity) {
                            for &b_tag in &b_tags.0 {
                                self.handle_collision(&a_tag, &b_tag, &a_entity, &b_entity);
                            }
                        }
                    }
                }
            }
        }
    }
}
impl CollisionMatchSystem {
    fn handle_collision(&mut self, a_tag: &str, b_tag: &str, a_entity: &Entity, b_entity: &Entity) {
        match (a_tag, b_tag) {
            ("player", "enemy") => {
                println!("PLAYER HIT ENEMY!");
            }
            ("player_bullet", "enemy") => {
                println!("PLAYER BULLET HIT ENEMY!");
            }
            ("asteroid", "asteroid") => {
                println!("ASTEROID HIT ASTEROID");
            }
            (&_, _) => (),
        }
    }
}
