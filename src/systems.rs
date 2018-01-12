use std::f32::consts::PI;

use specs::{Fetch, Join, ReadStorage, System, WriteStorage};

use ggez::graphics::Vector2;

use resources::*;
use components::*;

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
            let m_inertia = (velocity.x * velocity.x + velocity.y * velocity.y).sqrt();
            if m_inertia == 0.0 && thruster.throttle == 0.0 {
                continue;
            }
            let m_thrust = thruster.thrust * thruster.throttle * delta;
            let a_thrust = PI - (position.r + thruster.angle);
            let v_thrust = vec_from_angle(a_thrust) * m_thrust;
            velocity.x += v_thrust.x;
            velocity.y += v_thrust.y;
        }
    }
}

pub struct SpeedLimitSystem;

impl<'a> System<'a> for SpeedLimitSystem {
    type SystemData = (
        ReadStorage<'a, SpeedLimit>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (speed_limit, mut velocity) = data;
        for (speed_limit, velocity) in (&speed_limit, &mut velocity).join() {
            let m_inertia = (velocity.x * velocity.x + velocity.y * velocity.y).sqrt();
            if m_inertia <= speed_limit.max_speed {
                continue;
            }
            let a_inertia = velocity.x.atan2(velocity.y);
            let v_limit = vec_from_angle(a_inertia) * speed_limit.max_speed;
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
                // Skip if we're stopped
                continue;
            }
            let m_inertia = (velocity.x * velocity.x + velocity.y * velocity.y).sqrt();
            if m_inertia < 0.25 {
                // Stop if we're close enough to stopped.
                velocity.x = 0.0;
                velocity.y = 0.0;
                continue;
            }
            let a_inertia = velocity.x.atan2(velocity.y);
            let m_braking = friction.braking * delta;
            let v_braking = vec_from_angle(PI - a_inertia) * m_braking;
            velocity.x += v_braking.x;
            velocity.y += v_braking.y;
        }
    }
}
