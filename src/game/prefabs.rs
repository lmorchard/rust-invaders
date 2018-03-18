use std::f32::consts::PI;
use rand;

use specs::*;
use ggez::graphics::*;
use plugins::*;
use super::{HeroPlanet, HeroPlayer};

pub fn player(entity: Entity, lazy: &LazyUpdate) {
    LazyBuilder { entity, lazy }
        .with(metadata::Name("player"))
        .with(metadata::Tags::new(vec!["player", "friend"]))
        .with(HeroPlayer)
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
            period: 0.2,
            ..Default::default()
        })
        .with(collision::Collidable { size: 50.0 })
        .with(bounce::BounceOnCollision { mass: 5.0 })
        .with(health_damage::Health::new(1000.0))
        .with(sprites::Sprite {
            shape: sprites::Shape::Player,
            scale: Point2::new(50.0, 50.0),
            ..Default::default()
        })
        .with(player_control::PlayerControl)
        .build();
}

pub fn planet(entity: Entity, lazy: &LazyUpdate) {
    LazyBuilder { entity, lazy }
        .with(HeroPlanet)
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
        .with(health_damage::Health::new(5000.0))
        .build();
}

const HW: f32 = viewport::PLAYFIELD_WIDTH / 2.0;
const HH: f32 = viewport::PLAYFIELD_HEIGHT / 2.0;

pub fn asteroid(
    positions: &ReadStorage<position_motion::Position>,
    collidables: &ReadStorage<collision::Collidable>,
    entity: Entity,
    lazy: &LazyUpdate,
) {
    let size = 25.0 + 150.0 * rand::random::<f32>();
    let x = 0.0 - HW + (viewport::PLAYFIELD_WIDTH / 8.0) * (rand::random::<f32>() * 8.0);
    let y = 0.0 - HH - size;

    if !collision::is_empty_at(&positions, &collidables, x, y, size) {
        return;
    }

    LazyBuilder { entity, lazy }
        .with(metadata::Tags::new(vec!["asteroid", "enemy"]))
        .with(position_motion::Position {
            x,
            y,
            ..Default::default()
        })
        .with(position_motion::Velocity {
            x: 50.0 - 100.0 * rand::random::<f32>(),
            y: 50.0 + 100.0 * rand::random::<f32>(),
            r: PI * rand::random::<f32>(),
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
        .with(health_damage::Health::new(100.0))
        .with(score::PointsOnLastHit(1000))
        .build();
}

// Borrowed from https://github.com/slide-rs/specs/blob/dd81261d829ed3e424dbd8c4e5c6d61608ee356b/src/world/lazy.rs#L5
pub struct LazyBuilder<'a> {
    pub entity: Entity,
    pub lazy: &'a LazyUpdate,
}

impl<'a> LazyBuilder<'a> {
    pub fn with<C>(self, component: C) -> Self
    where
        C: Component + Send + Sync,
    {
        let entity = self.entity;
        self.lazy.execute(move |world| {
            world.write::<C>().insert(entity, component);
        });

        self
    }
    pub fn build(self) -> Entity {
        self.entity
    }
}
