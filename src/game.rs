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

    dispatcher
        .add(DespawnMatchSystem, "despawn_match", &[])
        .add(CollisionMatchSystem, "collision_match", &[])
}

pub fn update(world: &mut World) -> GameResult<()> {
    if rand::random::<f32>() < 0.025 {
        spawn_asteroid(world);
    }
    Ok(())
}

pub fn draw(world: &mut World, font: &mut fonts::Font, ctx: &mut Context) -> GameResult<()> {
    Ok(())
}

pub fn spawn_player(world: &mut World) {
    world
        .create_entity()
        .with(metadata::Name("player"))
        .with(metadata::Tags::new(vec!["player", "friend"]))
        .with(position_motion::Position {
            y: (viewport::PLAYFIELD_HEIGHT / 2.0) - 200.0,
            ..Default::default()
        })
        .with(position_motion::PositionBounds(Rect::new(
            0.0 - viewport::PLAYFIELD_WIDTH / 2.0 + 25.0,
            0.0 - viewport::PLAYFIELD_HEIGHT / 2.0 + 5.0,
            viewport::PLAYFIELD_WIDTH - 50.0,
            viewport::PLAYFIELD_HEIGHT - 10.0,
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
        .with(health_damage::DamageOnCollision {
            damage: 100.0,
            despawn: false,
            ..Default::default()
        })
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
            y: 1800.0,
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
        .with(health_damage::DamageOnCollision {
            damage: 100000.0,
            despawn: false,
            ..Default::default()
        })
        .with(health_damage::Health::new(100000.0))
        .build();
}

const HW: f32 = viewport::PLAYFIELD_WIDTH / 2.0;
const HH: f32 = viewport::PLAYFIELD_HEIGHT / 2.0;

pub fn spawn_asteroid(world: &mut World) {
    let size = 25.0 + 150.0 * rand::random::<f32>();
    let x = 0.0 - HW + (viewport::PLAYFIELD_WIDTH / 8.0) * (rand::random::<f32>() * 8.0);
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
            viewport::PLAYFIELD_WIDTH + 400.0,
            viewport::PLAYFIELD_HEIGHT + 400.0,
        )))
        .with(health_damage::DamageOnCollision {
            damage: 50.0,
            despawn: false,
            ..Default::default()
        })
        .with(health_damage::Health::new(100.0))
        .with(score::PointsOnLastHit(1000))
        .build();
}

pub struct DespawnMatchSystem;
impl<'a> System<'a> for DespawnMatchSystem {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, LazyUpdate>,
        Fetch<'a, despawn::DespawnEventQueue>,
        ReadStorage<'a, metadata::Tags>,
        ReadStorage<'a, position_motion::Position>,
        ReadStorage<'a, sprites::Sprite>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, lazy, despawn_events, tags, positions, sprites) = data;
        for despawn_event in &despawn_events.0 {
            let entity = despawn_event.entity;
            if let (Some(tags), Some(position), Some(sprite)) =
                (tags.get(entity), positions.get(entity), sprites.get(entity))
            {
                for &tag in &tags.0 {
                    self.handle_match(&entities, &lazy, &despawn_event, tag, &position, &sprite);
                }
            }
        }
    }
}
impl DespawnMatchSystem {
    fn handle_match(
        &mut self,
        entities: &Entities,
        lazy: &LazyUpdate,
        despawn_event: &despawn::DespawnEvent,
        tag: &str,
        position: &position_motion::Position,
        sprite: &sprites::Sprite,
    ) {
        if despawn_event.reason == despawn::DespawnReason::Health {
            if tag == "asteroid" {
                let explosion = entities.create();
                lazy.insert(explosion, despawn::Timeout(0.5));
                lazy.insert(
                    explosion,
                    position_motion::Position {
                        x: position.x,
                        y: position.y,
                        ..Default::default()
                    },
                );
                lazy.insert(
                    explosion,
                    position_motion::Velocity {
                        r: PI * 7.0,
                        ..Default::default()
                    },
                );
                lazy.insert(
                    explosion,
                    sprites::Sprite {
                        shape: sprites::Shape::Explosion,
                        scale: Point2::new(sprite.scale.x, sprite.scale.y),
                        ..Default::default()
                    },
                );
            }
        }
    }
}

pub struct CollisionMatchSystem;
impl<'a> System<'a> for CollisionMatchSystem {
    type SystemData = (
        Entities<'a>,
        FetchMut<'a, score::PlayerScore>,
        Fetch<'a, collision::Collisions>,
        FetchMut<'a, health_damage::DamageEventQueue>,
        FetchMut<'a, viewport::ViewportState>,
        ReadStorage<'a, metadata::Tags>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut player_score, collisions, mut damage, mut viewport, tags) = data;
        for (a_entity, a_tags) in (&*entities, &tags).join() {
            for &a_tag in &a_tags.0 {
                if let Some(ent_collisions) = collisions.get(&a_entity) {
                    for b_entity in ent_collisions.iter() {
                        if let Some(b_tags) = tags.get(*b_entity) {
                            for &b_tag in &b_tags.0 {
                                self.handle_match(
                                    &mut player_score,
                                    &mut viewport,
                                    &a_tag,
                                    &b_tag,
                                    &a_entity,
                                    &b_entity,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
impl CollisionMatchSystem {
    fn handle_match(
        &mut self,
        player_score: &mut score::PlayerScore,
        viewport: &mut viewport::ViewportState,
        a_tag: &str,
        b_tag: &str,
        a_entity: &Entity,
        b_entity: &Entity,
    ) {
        match (a_tag, b_tag) {
            ("player", "enemy") | ("asteroid", "planet") => {
                viewport.shake(7.0, 0.15);
            }
            ("player_bullet", "enemy") => {
                // player_score.increment(1000);
            }
            (&_, _) => (),
        }
    }
}
