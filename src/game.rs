use specs::*;
use ggez::*;
use rand;

use plugins::*;
use ::*;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    spawn_planet(world);
    spawn_player(world);
    for _idx in 0..10 {
        spawn_asteroid(world);
    }

    world.add_resource(PlayerScore::new());
    dispatcher
        .add(CollisionMatchSystem, "collision_match", &[])
}

#[derive(Debug)]
pub struct PlayerScore {
    current: i32,
    displayed: i32,
    factor: i32,
}
impl PlayerScore {
    pub fn new() -> PlayerScore {
        PlayerScore {
            current: 0,
            displayed: 0,
            factor: 10
        }
    }
    pub fn get(&self) -> i32 {
        self.current
    }
    pub fn set(&mut self, new_score: i32) {
        self.current = new_score;
    }
    pub fn increment(&mut self, amount: i32) {
        self.current += amount;
    }
    pub fn decrement(&mut self, amount: i32) {
        self.current -= amount;
        if self.current < 0 {
            self.current = 0;
        }
    }
    pub fn get_displayed(&self) -> i32 {
        self.displayed
    }
    pub fn update_displayed(&mut self) {
        if self.displayed == self.current {
            return;
        }
        if self.displayed < self.current {
            let incr = 1 + (self.current - self.displayed) / self.factor;
            self.displayed += incr;
            if self.displayed > self.current {
                self.displayed = self.current;
            }
        }
        if self.displayed > self.current {
            let decr = 1 + (self.displayed - self.current) / self.factor;
            self.displayed -= decr;
            if self.displayed < self.current {
                self.displayed = self.current;
            }
        }
    }
}

pub fn update(world: &mut World) -> GameResult<()> {
    if rand::random::<f32>() < 0.025 {
        spawn_asteroid(world);
    }
    Ok(())
}

pub fn draw(world: &mut World, font: &mut fonts::Font, ctx: &mut Context) -> GameResult<()> {
    let mut player_score = world.write_resource::<PlayerScore>();
    player_score.update_displayed();
    font.draw(
        ctx,
        &format!("{:07}", player_score.get_displayed()),
        fonts::DrawOptions {
            x: (PLAYFIELD_WIDTH / 2.0),
            y: 0.0 - (PLAYFIELD_HEIGHT / 2.0) + 24.0,
            scale: 3.0,
            reverse: true,
            ..Default::default()
        },
    )?;

    Ok(())
}

pub fn spawn_player(world: &mut World) {
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

pub fn spawn_planet(world: &mut World) {
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

pub fn spawn_asteroid(world: &mut World) {
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
        FetchMut<'a, PlayerScore>,
        Fetch<'a, collision::Collisions>,
        FetchMut<'a, health_damage::DamageEventQueue>,
        ReadStorage<'a, metadata::Tags>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut player_score, collisions, mut damage, tags) = data;
        for (a_entity, a_tags) in (&*entities, &tags).join() {
            for &a_tag in &a_tags.0 {
                if let Some(ent_collisions) = collisions.get(&a_entity) {
                    for b_entity in ent_collisions.iter() {
                        if let Some(b_tags) = tags.get(*b_entity) {
                            for &b_tag in &b_tags.0 {
                                self.handle_collision(&mut player_score, &a_tag, &b_tag, &a_entity, &b_entity);
                            }
                        }
                    }
                }
            }
        }
    }
}
impl CollisionMatchSystem {
    fn handle_collision(&mut self, player_score: &mut PlayerScore, a_tag: &str, b_tag: &str, a_entity: &Entity, b_entity: &Entity) {
        match (a_tag, b_tag) {
            ("player", "enemy") => {
                println!("PLAYER HIT ENEMY!");
            }
            ("player_bullet", "enemy") => {
                println!("PLAYER BULLET HIT ENEMY!");
                player_score.increment(1000);
            }
            ("asteroid", "asteroid") => {
                println!("ASTEROID HIT ASTEROID");
                player_score.increment(100);
            }
            ("asteroid", "planet") => {
                println!("ASTEROID HIT PLANET");
                player_score.decrement(1000);
            }
            (&_, _) => (),
        }
    }
}
