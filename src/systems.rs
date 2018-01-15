use std::f32::consts::PI;

use specs::{Entities, Fetch, FetchMut, Join, LazyUpdate, ReadStorage, System, WriteStorage};

use ggez::graphics::{Point2, Rect, Vector2};

use graphics::meshes::MeshSelection;
use resources::*;
use components::*;
use plugins;

fn vec_from_angle(angle: f32) -> Vector2 {
    let vx = angle.sin();
    let vy = angle.cos();
    Vector2::new(vx, vy)
}

pub struct MotionSystem;

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

pub struct PositionBoundsSystem;

impl<'a> System<'a> for PositionBoundsSystem {
    type SystemData = (ReadStorage<'a, PositionBounds>, WriteStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        let (bounds, mut pos) = data;
        for (bounds, pos) in (&bounds, &mut pos).join() {
            let bounds = bounds.0;
            if pos.x < bounds.x {
                pos.x = bounds.x;
            } else if pos.x > bounds.x + bounds.w {
                pos.x = bounds.x + bounds.w;
            }
            if pos.y < bounds.y {
                pos.y = bounds.y;
            } else if pos.y > bounds.y + bounds.h {
                pos.y = bounds.y + bounds.h;
            }
        }
    }
}

pub struct DespawnBoundsSystem;

impl<'a> System<'a> for DespawnBoundsSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, DespawnBounds>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, positions, bounds) = data;
        for (entity, pos, bounds) in (&*entities, &positions, &bounds).join() {
            let bounds = bounds.0;
            if pos.x < bounds.x || pos.x > bounds.x + bounds.w || pos.y < bounds.y
                || pos.y > bounds.y + bounds.h
            {
                entities.delete(entity).unwrap();
            }
        }
    }
}

fn apply_thrust(delta: f32, thruster: &Thruster, position: &Position, velocity: &mut Velocity) {
    let m_inertia = (velocity.x * velocity.x + velocity.y * velocity.y).sqrt();
    if m_inertia == 0.0 && thruster.throttle == 0.0 {
        return;
    }
    let m_thrust = thruster.thrust * thruster.throttle * delta;
    let a_thrust = PI - (position.r + thruster.angle);
    let v_thrust = vec_from_angle(a_thrust) * m_thrust;

    velocity.x += v_thrust.x;
    velocity.y += v_thrust.y;
}

pub struct ThrusterSystem;

impl<'a> System<'a> for ThrusterSystem {
    type SystemData = (
        Fetch<'a, DeltaTime>,
        ReadStorage<'a, Thruster>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (delta, thruster, position, mut velocity) = data;
        let delta = delta.0;
        for (thruster, position, velocity) in (&thruster, &position, &mut velocity).join() {
            apply_thrust(delta, thruster, position, velocity);
        }
    }
}

pub struct ThrusterSetSystem;

impl<'a> System<'a> for ThrusterSetSystem {
    type SystemData = (
        Fetch<'a, DeltaTime>,
        ReadStorage<'a, ThrusterSet>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (delta, thruster_set, position, mut velocity) = data;
        let delta = delta.0;
        for (thruster_set, position, velocity) in (&thruster_set, &position, &mut velocity).join() {
            // TODO: I thought thruster_set.values() would work here, but alas no
            for (_name, thruster) in &thruster_set.0 {
                apply_thrust(delta, thruster, position, velocity);
            }
        }
    }
}

pub struct SpeedLimitSystem;

impl<'a> System<'a> for SpeedLimitSystem {
    type SystemData = (ReadStorage<'a, SpeedLimit>, WriteStorage<'a, Velocity>);

    fn run(&mut self, data: Self::SystemData) {
        let (speed_limit, mut velocity) = data;
        for (speed_limit, velocity) in (&speed_limit, &mut velocity).join() {
            let speed_limit = speed_limit.0;
            let m_inertia = (velocity.x * velocity.x + velocity.y * velocity.y).sqrt();
            if m_inertia <= speed_limit {
                continue;
            }

            let a_inertia = velocity.x.atan2(velocity.y);
            let v_limit = vec_from_angle(a_inertia) * speed_limit;

            // TODO: Rework so speed limit is applied as thrust, rather than assignment.
            velocity.x = v_limit.x;
            velocity.y = v_limit.y;
        }
    }
}

pub struct FrictionSystem;

impl<'a> System<'a> for FrictionSystem {
    type SystemData = (
        Fetch<'a, DeltaTime>,
        ReadStorage<'a, Friction>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (delta, friction, mut velocity) = data;
        let delta = delta.0;
        for (friction, velocity) in (&friction, &mut velocity).join() {
            if velocity.x == 0.0 && velocity.y == 0.0 {
                continue;
            }

            let friction = friction.0;
            let m_inertia = (velocity.x * velocity.x + velocity.y * velocity.y).sqrt();
            let a_inertia = velocity.x.atan2(velocity.y);
            let m_braking = 0.0 - m_inertia.min((friction * delta));
            let v_braking = vec_from_angle(a_inertia) * m_braking;

            velocity.x += v_braking.x;
            velocity.y += v_braking.y;
        }
    }
}

pub struct PlayerControlSystem;

impl<'a> System<'a> for PlayerControlSystem {
    type SystemData = (
        Fetch<'a, DeltaTime>,
        Fetch<'a, Inputs>,
        WriteStorage<'a, ThrusterSet>,
        WriteStorage<'a, Gun>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (_delta, inputs, mut thruster_set, mut gun) = data;
        for (thruster_set, gun) in (&mut thruster_set, &mut gun).join() {
            gun.firing = inputs.fire;

            if let Some(lat_thruster) = thruster_set.0.get_mut("lateral") {
                lat_thruster.throttle = if inputs.right {
                    1.0
                } else if inputs.left {
                    -1.0
                } else {
                    0.0
                }
            }

            if let Some(long_thruster) = thruster_set.0.get_mut("longitudinal") {
                long_thruster.throttle = if inputs.up {
                    1.0
                } else if inputs.down {
                    -1.0
                } else {
                    0.0
                }
            }
        }
    }
}

pub struct GunSystem;

impl<'a> System<'a> for GunSystem {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, DeltaTime>,
        Fetch<'a, LazyUpdate>,
        ReadStorage<'a, Velocity>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Gun>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, delta, lazy, velocities, positions, mut guns) = data;
        let delta = delta.0;
        for (velocity, position, gun) in (&velocities, &positions, &mut guns).join() {
            if gun.cooldown > 0.0 {
                gun.cooldown -= delta;
                continue;
            }
            if !gun.firing {
                continue;
            }
            gun.cooldown = gun.period;

            let bullet = entities.create();
            lazy.insert(
                bullet,
                Position {
                    x: position.x,
                    y: position.y - 50.0,
                    ..Default::default()
                },
            );
            lazy.insert(
                bullet,
                DespawnBounds(Rect::new(-800.0, -450.0, 1600.0, 900.0)),
            );
            lazy.insert(
                bullet,
                Velocity {
                    y: -800.0,
                    ..Default::default()
                },
            );
            lazy.insert(bullet, plugins::collision::Collidable { size: 50.0 });
            //lazy.insert(bullet, plugins::health_damage::Health(100.0));
            lazy.insert(bullet, plugins::health_damage::DamageOnCollision(100.0));
            lazy.insert(
                bullet,
                Sprite {
                    mesh_selection: MeshSelection::SimpleBullet,
                    scale: Point2::new(50.0, 50.0),
                    ..Default::default()
                },
            );
        }
    }
}
