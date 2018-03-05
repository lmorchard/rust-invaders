use specs::*;
use ggez::graphics::Vector2;
use DeltaTime;
use plugins::*;

#[derive(Component, Debug)]
pub struct Friction(pub f32);

#[derive(Component, Debug)]
pub struct SpeedLimit(pub f32);

pub struct SpeedLimitSystem;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.register::<Friction>();
    world.register::<SpeedLimit>();
    dispatcher
        .add(SpeedLimitSystem, "speed_limit", &[])
        .add(FrictionSystem, "friction", &[])
}

// TODO: Use nalgebra or move this to a util module
fn vec_from_angle(angle: f32) -> Vector2 {
    let vx = angle.sin();
    let vy = angle.cos();
    Vector2::new(vx, vy)
}

impl<'a> System<'a> for SpeedLimitSystem {
    type SystemData = (
        ReadStorage<'a, SpeedLimit>,
        WriteStorage<'a, position_motion::Velocity>,
    );

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
        WriteStorage<'a, position_motion::Velocity>,
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
            let m_braking = 0.0 - m_inertia.min(friction * delta);
            let v_braking = vec_from_angle(a_inertia) * m_braking;

            velocity.x += v_braking.x;
            velocity.y += v_braking.y;
        }
    }
}
