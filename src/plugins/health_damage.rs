use specs::*;
use plugins::*;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.add_resource(DamageEventQueue::new());
    world.register::<Health>();
    world.register::<DamageOnCollision>();
    dispatcher
        .add(DamageOnCollisionSystem, "damage_on_collision", &[])
        .add(HealthSystem, "health", &["damage_on_collision"])
}

#[derive(Component, Debug)]
pub struct Health(pub f32);

#[derive(Component, Debug)]
pub struct DamageOnCollision(pub f32);

#[derive(Debug)]
pub struct DamageEvent {
    from: Entity,
    to: Entity,
    amount: f32,
}

#[derive(Debug)]
pub struct DamageEventQueue(pub Vec<DamageEvent>);
impl DamageEventQueue {
    pub fn new() -> DamageEventQueue {
        DamageEventQueue(Vec::new())
    }
}

pub struct DamageOnCollisionSystem;
impl<'a> System<'a> for DamageOnCollisionSystem {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, collision::Collisions>,
        FetchMut<'a, DamageEventQueue>,
        ReadStorage<'a, DamageOnCollision>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, collisions, mut damage_events, damages) = data;
        // TODO: Set a timer to only send damage once every so often, rather than for every frame
        // entities are in collision
        for (ent, damage) in (&*entities, &damages).join() {
            if let Some(ref ent_collisions) = collisions.get(&ent) {
                for other_ent in ent_collisions.iter() {
                    damage_events.0.push(DamageEvent {
                        from: ent.clone(),
                        to: other_ent.clone(),
                        amount: damage.0,
                    });
                }
            }
        }
    }
}

pub struct HealthSystem;
impl<'a> System<'a> for HealthSystem {
    type SystemData = (
        Entities<'a>,
        FetchMut<'a, DamageEventQueue>,
        FetchMut<'a, despawn::DespawnEventQueue>,
        WriteStorage<'a, Health>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut damage_events, mut despawn_events, mut healths) = data;
        // TODO: Maintain a timer to ignore repeated damage from a source for a period of time
        for damage_event in &damage_events.0 {
            if let Some(ref mut health) = healths.get_mut(damage_event.to) {
                health.0 -= damage_event.amount;
            }
        }
        damage_events.0.clear();
        for (entity, health) in (&*entities, &mut healths).join() {
            if health.0 <= 0.0 {
                despawn_events
                    .0
                    .push(despawn::DespawnEvent { entity: entity });
            }
        }
    }
}
