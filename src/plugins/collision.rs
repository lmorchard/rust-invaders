use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};

use ggez::*;
use specs::*;

use resources::*;
use components::*;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.add_resource(Collisions::new());
    world.register::<Collidable>();
    dispatcher
        .add(CollisionSystem, "collision", &[])
}

pub fn update_after(world: &mut World, ctx: &mut Context) {
}

pub fn draw_after(world: &mut World, ctx: &mut Context) {}

#[derive(Debug)]
pub struct Collisions(pub HashMap<Entity, HashSet<Entity>>);
impl Collisions {
    pub fn new() -> Self {
        Collisions(HashMap::new())
    }
    pub fn insert(&mut self, e1: Entity, e2: Entity) {
        self.0.entry(e1).or_insert(HashSet::new()).insert(e2);
        self.0.entry(e2).or_insert(HashSet::new()).insert(e1);
    }
    pub fn remove(&mut self, e1: Entity, e2: Entity) {
        self.0.entry(e1).or_insert(HashSet::new()).remove(&e2);
        self.0.entry(e2).or_insert(HashSet::new()).remove(&e1);
    }
}
impl Deref for Collisions {
    type Target = HashMap<Entity, HashSet<Entity>>;
    fn deref(&self) -> &HashMap<Entity, HashSet<Entity>> {
        &self.0
    }
}
impl DerefMut for Collisions {
    fn deref_mut(&mut self) -> &mut HashMap<Entity, HashSet<Entity>> {
        &mut self.0
    }
}

#[derive(Component, Debug)]
pub struct Collidable {
    pub size: f32,
}

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        Entities<'a>,
        FetchMut<'a, Collisions>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Collidable>,
    );

    fn run(&mut self, (entities, mut collisions, positions, collidables): Self::SystemData) {
        collisions.clear();
        // TODO: Replace this compare of all-to-all with a quadtree search
        for (entity, pos, col) in (&*entities, &positions, &collidables).join() {
            for (other_entity, other_pos, other_col) in
                (&*entities, &positions, &collidables).join()
            {
                if entity == other_entity {
                    continue;
                }
                // Simple circular overlap "hitbox" - TODO: implement more complex logic
                let overlap_range = ((col.size / 2.0) + (other_col.size / 2.0)).powf(2.0);
                let distance_sq = (other_pos.x - pos.x).powf(2.0) + (other_pos.y - pos.y).powf(2.0);
                if distance_sq <= overlap_range {
                    collisions.insert(entity, other_entity);
                }
            }
        }
    }
}