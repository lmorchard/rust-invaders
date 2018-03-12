use ggez::*;
use ggez::graphics::*;
use specs::*;
use plugins::*;
use std::ops::Deref;
use DeltaTime;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.add_resource(DespawnEventQueue::new());
    world.register::<Timeout>();
    world.register::<DespawnBounds>();
    world.register::<DespawnOnCollision>();
    dispatcher
        .add(TimeoutSystem, "timeout_system", &[])
        .add(DespawnBoundsSystem, "despawn_bounds_system", &[])
        .add(
            DespawnOnCollisionSystem,
            "despawn_on_collision_system",
            &["damage_on_collision"],
        )
}

pub fn update(world: &mut World) -> GameResult<()> {
    let mut entities = world.entities();
    let mut despawn_events = world.write_resource::<DespawnEventQueue>();
    for despawn_event in &despawn_events.0 {
        if let Err(err) = entities.delete(despawn_event.entity) {
            return Err(GameError::UnknownError(format!(
                "Failed to delete despawning entity {:?} - {:?}",
                despawn_event.entity, err
            )));
        }
    }
    despawn_events.0.clear();
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
pub enum DespawnReason {
    Timeout,
    Collision,
    SelfDestruct,
    Health,
    OutOfBounds,
    Other(&'static str),
}

#[derive(Debug, Eq, PartialEq)]
pub struct DespawnEvent {
    pub entity: Entity,
    pub reason: DespawnReason,
}

#[derive(Debug)]
pub struct DespawnEventQueue(pub Vec<DespawnEvent>);
impl Default for DespawnEventQueue {
    fn default() -> DespawnEventQueue {
        DespawnEventQueue(Vec::new())
    }
}
impl Deref for DespawnEventQueue {
    type Target = Vec<DespawnEvent>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DespawnEventQueue {
    pub fn new() -> DespawnEventQueue {
        Default::default()
    }
    pub fn despawn(&mut self, entity: Entity, reason: DespawnReason) {
        self.0.push(DespawnEvent { entity, reason });
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
                despawn_events.despawn(entity, DespawnReason::OutOfBounds);
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
            if collisions.get(&entity).is_some() {
                despawn_events.despawn(entity, DespawnReason::Collision);
            }
        }
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
                despawn_events.despawn(entity, DespawnReason::Timeout);
            }
        }
    }
}
