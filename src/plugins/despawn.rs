use ggez::graphics::*;
use specs::*;
use plugins::*;
use DeltaTime;

use std::f32::consts::PI;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.add_resource(DespawnEventQueue::new());
    world.register::<Timeout>();
    world.register::<Tombstone>();
    world.register::<DespawnBounds>();
    world.register::<DespawnOnCollision>();
    dispatcher
        .add(TimeoutSystem, "timeout_system", &[])
        .add(TombstoneSystem, "tombstone_system", &[])
        .add(DespawnBoundsSystem, "despawn_bounds_system", &[])
        .add(
            DespawnOnCollisionSystem,
            "despawn_on_collision_system",
            &["damage_on_collision"],
        )
        .add(
            DespawnRemovalSystem,
            "despawn_removal_system",
            &["damage_on_collision"],
        )
}

#[derive(Debug, Eq, PartialEq)]
pub struct DespawnEvent {
    pub entity: Entity,
}

#[derive(Debug)]
pub struct DespawnEventQueue(pub Vec<DespawnEvent>);
impl DespawnEventQueue {
    pub fn new() -> DespawnEventQueue {
        DespawnEventQueue(Vec::new())
    }
}

#[derive(Component, Debug)]
pub struct Timeout(pub f32);

pub struct TimeoutSystem;

impl<'a> System<'a> for TimeoutSystem {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, DeltaTime>,
        FetchMut<'a, DespawnEventQueue>,
        WriteStorage<'a, Timeout>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, delta, mut despawn_events, mut timeouts) = data;
        let delta = delta.0;
        for (entity, mut timeout) in (&*entities, &mut timeouts).join() {
            timeout.0 -= delta;
            if timeout.0 <= 0.0 {
                despawn_events.0.push(DespawnEvent { entity });
            }
        }
    }
}

#[derive(Component, Debug)]
pub struct Tombstone;

pub struct TombstoneSystem;

impl<'a> System<'a> for TombstoneSystem {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, LazyUpdate>,
        FetchMut<'a, DespawnEventQueue>,
        ReadStorage<'a, sprites::Sprite>,
        ReadStorage<'a, position_motion::Position>,
        ReadStorage<'a, Tombstone>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, lazy, mut despawn_events, sprites, positions, tombstones) = data;
        for (entity, sprite, position, tombstone) in
            (&*entities, &sprites, &positions, &tombstones).join()
        {
            if !despawn_events.0.contains(&DespawnEvent { entity }) {
                continue;
            }
            // TODO: Implement multiple tombstone selections beyond explosions
            let tombstone = entities.create();
            lazy.insert(tombstone, Timeout(0.5));
            lazy.insert(
                tombstone,
                position_motion::Position {
                    x: position.x,
                    y: position.y,
                    ..Default::default()
                },
            );
            lazy.insert(
                tombstone,
                position_motion::Velocity {
                    r: PI * 7.0,
                    ..Default::default()
                },
            );
            lazy.insert(
                tombstone,
                sprites::Sprite {
                    shape: sprites::Shape::Explosion,
                    scale: Point2::new(sprite.scale.x, sprite.scale.y),
                    ..Default::default()
                },
            );
        }
    }
}

#[derive(Component, Debug)]
pub struct DespawnBounds(pub Rect);

pub struct DespawnBoundsSystem;

impl<'a> System<'a> for DespawnBoundsSystem {
    type SystemData = (
        Entities<'a>,
        FetchMut<'a, DespawnEventQueue>,
        ReadStorage<'a, position_motion::Position>,
        ReadStorage<'a, DespawnBounds>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut despawn_events, positions, bounds) = data;
        for (entity, pos, bounds) in (&*entities, &positions, &bounds).join() {
            let bounds = bounds.0;
            if pos.x < bounds.x || pos.x > bounds.x + bounds.w || pos.y < bounds.y
                || pos.y > bounds.y + bounds.h
            {
                despawn_events.0.push(DespawnEvent { entity });
            }
        }
    }
}

#[derive(Component, Debug)]
pub struct DespawnOnCollision;

pub struct DespawnOnCollisionSystem;
impl<'a> System<'a> for DespawnOnCollisionSystem {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, collision::Collisions>,
        FetchMut<'a, DespawnEventQueue>,
        ReadStorage<'a, DespawnOnCollision>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, collisions, mut despawn_events, on_collisions) = data;
        for (entity, _on_collision) in (&*entities, &on_collisions).join() {
            if let Some(_) = collisions.get(&entity) {
                despawn_events.0.push(despawn::DespawnEvent { entity });
            }
        }
    }
}

pub struct DespawnRemovalSystem;
impl<'a> System<'a> for DespawnRemovalSystem {
    type SystemData = (Entities<'a>, FetchMut<'a, DespawnEventQueue>);
    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut despawn_events) = data;
        if despawn_events.0.len() > 0 {}
        for despawn_event in &despawn_events.0 {
            // TODO: Figure out why we're deleting already dead entities
            match entities.delete(despawn_event.entity) {
                _ => (),
            };
        }
        despawn_events.0.clear();
    }
}
