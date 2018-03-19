use specs::*;
use ggez::graphics::{Point2, Rect};
use DeltaTime;
use plugins::*;
use game::*;
use game::sound_effects::SoundEffectType;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.register::<Gun>();
    dispatcher.add(GunSystem, "gun", &[])
}

#[derive(Component, Debug)]
pub struct Gun {
    pub firing: bool,
    pub period: f32,
    pub cooldown: f32,
}
impl Default for Gun {
    fn default() -> Gun {
        Gun {
            firing: false,
            period: 1.0,
            cooldown: 0.0,
        }
    }
}

pub struct GunSystem;

impl<'a> System<'a> for GunSystem {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, DeltaTime>,
        Fetch<'a, LazyUpdate>,
        FetchMut<'a, sound_effects::SoundEffectQueue>,
        ReadStorage<'a, position_motion::Position>,
        WriteStorage<'a, Gun>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, delta, lazy, mut sounds, positions, mut guns) = data;
        let delta = delta.0;
        for (_entity, position, gun) in (&*entities, &positions, &mut guns).join() {
            if gun.cooldown > 0.0 {
                gun.cooldown -= delta;
                continue;
            }
            if !gun.firing {
                continue;
            }
            gun.cooldown = gun.period;

            sounds.play(SoundEffectType::Shot);

            let bullet = entities.create();
            lazy.insert(
                bullet,
                metadata::Tags::new(vec!["player_bullet", "player_weapon"]),
            );
            lazy.insert(
                bullet,
                position_motion::Position {
                    x: position.x,
                    y: position.y - 50.0,
                    ..Default::default()
                },
            );
            lazy.insert(
                bullet,
                position_motion::Velocity {
                    y: -800.0,
                    ..Default::default()
                },
            );
            lazy.insert(bullet, collision::Collidable { size: 50.0 });
            lazy.insert(bullet, health_damage::Health::new(10.0));
            lazy.insert(
                bullet,
                despawn::DespawnBounds(Rect::new(-800.0, -550.0, 1600.0, 1000.0)),
            );
            lazy.insert(
                bullet,
                sprites::Sprite {
                    shape: sprites::Shape::SimpleBullet,
                    scale: Point2::new(50.0, 50.0),
                    ..Default::default()
                },
            );
        }
    }
}
