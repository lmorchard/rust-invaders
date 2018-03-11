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
pub struct Health {
    pub health: f32,
    pub last_hurt_by: Option<Entity>,
    pub last_healed_by: Option<Entity>,
}
impl Health {
    pub fn new(health: f32) -> Health {
        Health {
            health,
            ..Default::default()
        }
    }
    pub fn hurt(&mut self, amount: f32, from: Entity) {
        self.health -= amount;
        self.last_hurt_by = Some(from);
    }
    pub fn heal(&mut self, amount: f32, from: Entity) {
        self.health += amount;
        self.last_healed_by = Some(from);
    }
}
impl Default for Health {
    fn default() -> Health {
        Health {
            health: 100.0,
            last_hurt_by: None,
            last_healed_by: None,
        }
    }
}

#[derive(Component, Debug)]
pub struct DamageOnCollision {
    pub damage: f32,
    pub despawn: bool,
    pub exclude: Vec<Entity>,
}
impl Default for DamageOnCollision {
    fn default() -> DamageOnCollision {
        DamageOnCollision {
            damage: 0.0,
            despawn: true,
            exclude: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct DamageEvent {
    pub from: Entity,
    pub to: Entity,
    pub amount: f32,
}

#[derive(Debug)]
pub struct DamageEventQueue(pub Vec<DamageEvent>);
impl DamageEventQueue {
    pub fn new() -> DamageEventQueue {
        Default::default()
    }
}
impl Default for DamageEventQueue {
    fn default() -> DamageEventQueue {
        DamageEventQueue(Vec::new())
    }
}

pub struct DamageOnCollisionSystem;
impl<'a> System<'a> for DamageOnCollisionSystem {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, collision::Collisions>,
        FetchMut<'a, DamageEventQueue>,
        FetchMut<'a, despawn::DespawnEventQueue>,
        ReadStorage<'a, DamageOnCollision>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, collisions, mut damage_events, mut despawn_events, damages) = data;
        // TODO: Set a timer to only send damage once every so often, rather than for every frame
        // entities are in collision
        for (ent, damage) in (&*entities, &damages).join() {
            if let Some(ent_collisions) = collisions.get(&ent) {
                for other_ent in ent_collisions.iter() {
                    // TODO: Find a better way to identify exclusions - we won't always know
                    // literal entities. Maybe use some sort of marker component
                    if damage.exclude.contains(other_ent) {
                        continue;
                    }
                    if damage.despawn {
                        despawn_events.0.push(despawn::DespawnEvent { entity: ent });
                    }
                    damage_events.0.push(DamageEvent {
                        from: ent,
                        to: *other_ent,
                        amount: damage.damage,
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
                health.hurt(damage_event.amount, damage_event.from);
            }
        }
        damage_events.0.clear();
        for (entity, health) in (&*entities, &mut healths).join() {
            if health.health <= 0.0 {
                despawn_events.0.push(despawn::DespawnEvent { entity });
            }
        }
    }
}
