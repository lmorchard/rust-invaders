use specs::*;
use plugins::*;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.register::<BounceOnCollision>();
    dispatcher.add(BounceOnCollisionSystem, "bounce_on_collision", &[])
}

#[derive(Component, Debug)]
pub struct BounceOnCollision {
    pub mass: f32,
}
impl Default for BounceOnCollision {
    fn default() -> BounceOnCollision {
        BounceOnCollision { mass: 100.0 }
    }
}

pub struct BounceOnCollisionSystem;
impl<'a> System<'a> for BounceOnCollisionSystem {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, collision::Collisions>,
        FetchMut<'a, health_damage::DamageEventQueue>,
        ReadStorage<'a, collision::Collidable>,
        ReadStorage<'a, BounceOnCollision>,
        WriteStorage<'a, position_motion::Position>,
        WriteStorage<'a, position_motion::Velocity>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, collisions, mut damage, cols, bounces, mut positions, mut vels) = data;

        // TODO: Track seen entity pairs, skip those already processed
        for (a_entity, a_collidable, a_bounce) in (&*entities, &cols, &bounces).join() {
            if let Some(ref ent_collisions) = collisions.get(&a_entity) {
                for b_entity in ent_collisions.iter() {
                    let result;
                    {
                        let a_position = positions.get(a_entity);
                        let a_velocity = vels.get(a_entity);
                        let b_collidable = cols.get(*b_entity);
                        let b_bounce = bounces.get(*b_entity);
                        let b_position = positions.get(*b_entity);
                        let b_velocity = vels.get(*b_entity);

                        if a_position.is_none() || a_velocity.is_none() || b_collidable.is_none()
                            || b_bounce.is_none() || b_position.is_none()
                            || b_velocity.is_none()
                        {
                            continue;
                        }

                        result = resolve_elastic_collision(
                            &a_bounce,
                            &a_collidable,
                            a_position.unwrap(),
                            a_velocity.unwrap(),
                            b_bounce.unwrap(),
                            b_collidable.unwrap(),
                            b_position.unwrap(),
                            b_velocity.unwrap(),
                        );
                    }
                    if let Some((ax, ay, bx, by)) = result {
                        {
                            let mut a_velocity = vels.get_mut(a_entity).unwrap();
                            a_velocity.x = ax;
                            a_velocity.y = ay;
                        }
                        {
                            let mut b_velocity = vels.get_mut(*b_entity).unwrap();
                            b_velocity.x = bx;
                            b_velocity.y = by;
                        }
                    }
                }
            }
        }
    }
}

fn rotate(vx: f32, vy: f32, angle: f32) -> (f32, f32) {
    (
        vx * angle.cos() - vy * angle.sin(),
        vx * angle.sin() + vy * angle.cos(),
    )
}

// Elastic collision math transliterated from JavaScript
// https://github.com/lmorchard/panic-ranger/blob/master/src/plugins/bounce.js
// https://gist.github.com/christopher4lis/f9ccb589ee8ecf751481f05a8e59b1dc
fn resolve_elastic_collision(
    a_bounce: &BounceOnCollision,
    a_collidable: &collision::Collidable,
    a_position: &position_motion::Position,
    a_velocity: &position_motion::Velocity,
    b_bounce: &BounceOnCollision,
    b_collidable: &collision::Collidable,
    b_position: &position_motion::Position,
    b_velocity: &position_motion::Velocity,
) -> Option<(f32, f32, f32, f32)> {
    let x_velocity_diff = a_velocity.x - b_velocity.x;
    let y_velocity_diff = a_velocity.y - b_velocity.y;

    let x_dist = b_position.x - a_position.x;
    let y_dist = b_position.y - a_position.y;

    if x_velocity_diff * x_dist + y_velocity_diff * y_dist <= 0.0 {
        // Prevent accidental overlap of particles
        return None;
    }

    let angle = 0.0 - (b_position.y - a_position.y).atan2(b_position.x - a_position.x);

    let m1 = a_bounce.mass;
    let m2 = b_bounce.mass;

    let (u1x, u1y) = rotate(a_velocity.x, a_velocity.y, angle);
    let (u2x, u2y) = rotate(b_velocity.x, b_velocity.y, angle);

    let v1x = u1x * (m1 - m2) / (m1 + m2) + u2x * 2.0 * m2 / (m1 + m2);
    let v1y = u1y;

    let v2x = u2x * (m1 - m2) / (m1 + m2) + u1x * 2.0 * m2 / (m1 + m2);
    let v2y = u2y;

    let (vf1x, vf1y) = rotate(v1x, v1y, 0.0 - angle);
    let (vf2x, vf2y) = rotate(v2x, v2y, 0.0 - angle);

    // TODO: Calculate some damage based on mass & velocity

    Some((vf1x, vf1y, vf2x, vf2y))
}
