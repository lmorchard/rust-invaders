use std::collections::HashMap;
use std::f32::consts::PI;
use ggez::graphics::Vector2;
use specs::*;
use plugins::*;
use DeltaTime;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.register::<Thruster>();
    world.register::<ThrusterSet>();
    dispatcher
        .add(ThrusterSystem, "thruster", &[])
        .add(ThrusterSetSystem, "thruster_set", &[])
}

#[derive(Component, Debug)]
pub struct Thruster {
    pub thrust: f32,
    pub throttle: f32,
    pub angle: f32,
}

#[derive(Component, Debug)]
pub struct ThrusterSet(pub HashMap<&'static str, Thruster>);

// TODO: Use nalgebra or move this to a util module
fn vec_from_angle(angle: f32) -> Vector2 {
    let vx = angle.sin();
    let vy = angle.cos();
    Vector2::new(vx, vy)
}

fn apply_thrust(
    delta: f32,
    thruster: &Thruster,
    position: &position_motion::Position,
    velocity: &mut position_motion::Velocity,
) {
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
        ReadStorage<'a, position_motion::Position>,
        WriteStorage<'a, position_motion::Velocity>,
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
        ReadStorage<'a, position_motion::Position>,
        WriteStorage<'a, position_motion::Velocity>,
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
