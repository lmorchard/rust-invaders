use ggez::*;
use specs::*;

use resources::*;
use components::*;
use plugins::*;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.add_resource(DespawnEventQueue::new());
    world.register::<DespawnOnCollision>();
    dispatcher
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
