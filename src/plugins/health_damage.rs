use std::ops::{Deref, DerefMut};

use ggez::*;
use specs::*;

use resources::*;
use components::*;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.add_resource(GameEventBuffer::new());
    world.register::<Health>();
    world.register::<DamageOnCollision>();
    dispatcher
        .add(DamageOnCollisionSystem, "damage_on_collision", &[])
        .add(HealthSystem, "health", &["damage_on_collision"])
}

pub fn update(world: &mut World, ctx: &mut Context) {
    let mut events = world.write_resource::<GameEventBuffer>();
    events.clear();
}

pub fn draw(world: &mut World, ctx: &mut Context) {}

#[derive(Component, Debug)]
pub struct Health(pub f32);

#[derive(Component, Debug)]
pub struct DamageOnCollision(pub f32);

pub trait GameEvent {
    fn is_kind(&self, &str) -> bool;
}

pub struct GameEventBuffer(pub Vec<Box<GameEvent + Send + Sync>>);
impl GameEventBuffer {
    pub fn new() -> Self {
        GameEventBuffer(Vec::new())
    }
}
impl Deref for GameEventBuffer {
    type Target = Vec<Box<GameEvent + Send + Sync>>;
    fn deref(&self) -> &Vec<Box<GameEvent + Send + Sync>> {
        &self.0
    }
}
impl DerefMut for GameEventBuffer {
    fn deref_mut(&mut self) -> &mut Vec<Box<GameEvent + Send + Sync>> {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct DamageEvent {
    to: Entity,
    amount: f32,
}
impl GameEvent for DamageEvent {
    fn is_kind(&self, want_kind: &str) -> bool {
        want_kind == "damage"
    }
}

pub struct DamageOnCollisionSystem;
impl<'a> System<'a> for DamageOnCollisionSystem {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, DeltaTime>,
        Fetch<'a, LazyUpdate>,
        Fetch<'a, Collisions>,
        FetchMut<'a, GameEventBuffer>,
        ReadStorage<'a, Collidable>,
        ReadStorage<'a, DamageOnCollision>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, delta, lazy, collisions, mut events, collidables, damage_on_collisions) =
            data;
        let delta = delta.0;
        for (ent, collidable, damage_on_collision) in
            (&*entities, &collidables, &damage_on_collisions).join()
        {
            if let Some(ref ent_collisions) = collisions.get(&ent) {
                for other_ent in ent_collisions.iter() {
                    events.push(Box::new(DamageEvent {
                        to: other_ent.clone(),
                        amount: damage_on_collision.0,
                    }));
                    println!(
                        "ENT {:?} DAMAGES {:?} FOR {:?}",
                        ent, other_ent, damage_on_collision.0
                    );
                }
            }
        }
    }
}

pub struct HealthSystem;
impl<'a> System<'a> for HealthSystem {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, DeltaTime>,
        Fetch<'a, LazyUpdate>,
        FetchMut<'a, GameEventBuffer>,
        ReadStorage<'a, Health>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, delta, lazy, mut events, healths) = data;
        let delta = delta.0;
        for (ent, health) in (&*entities, &healths).join() {}
    }
}
