use ggez::graphics::*;
use specs::*;
use plugins::*;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.add_resource(DespawnEventQueue::new());
    world.register::<DespawnBounds>();
    world.register::<DespawnOnCollision>();
    dispatcher
        .add(DespawnBoundsSystem, "despawn_bounds_system", &[])
        .add(DespawnOnCollisionSystem, "despawn_on_collision_system", &[])
        .add(DespawnRemovalSystem, "despawn_removal_system", &[])
}

#[derive(Debug)]
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
pub struct DespawnBounds(pub Rect);

pub struct DespawnBoundsSystem;

impl<'a> System<'a> for DespawnBoundsSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, position_motion::Position>,
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
        for despawn_event in &despawn_events.0 {
            // TODO: Figure out why we're deleting already dead entities
            match entities.delete(despawn_event.entity) {
                _ => (),
            };
        }
        despawn_events.0.clear();
    }
}
