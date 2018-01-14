use std::ops::{Deref, DerefMut};
use std::collections::{HashMap, HashSet};

use specs::Entity;

#[derive(Debug)]
pub struct DeltaTime(pub f32);

#[derive(Debug)]
pub struct Inputs {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub fire: bool,
    pub special: bool,
}

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
