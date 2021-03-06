use specs::*;
use plugins::*;
use DeltaTime;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.add_resource(PlayerScore::new());
    world.register::<PointsOnLastHit>();
    dispatcher.add(ScoreSystem, "score_system", &[]).add(
        PointsOnLastHitSystem,
        "points_on_last_hit",
        &[],
    )
}

pub struct ScoreSystem;
impl<'a> System<'a> for ScoreSystem {
    type SystemData = (Fetch<'a, DeltaTime>, FetchMut<'a, PlayerScore>);
    fn run(&mut self, data: Self::SystemData) {
        let (_delta, mut player_score) = data;
        player_score.update();
    }
}

#[derive(Component, Debug)]
pub struct PointsOnLastHit(pub i32);
pub struct PointsOnLastHitSystem;
impl<'a> System<'a> for PointsOnLastHitSystem {
    type SystemData = (
        Entities<'a>,
        FetchMut<'a, PlayerScore>,
        Fetch<'a, despawn::DespawnEventQueue>,
        ReadStorage<'a, PointsOnLastHit>,
        ReadStorage<'a, health_damage::Health>,
        ReadStorage<'a, metadata::Tags>,
        ReadStorage<'a, sprites::Sprite>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (
            _entities,
            mut player_score,
            despawn_events,
            points_on_last_hits,
            healths,
            tags,
            _sprites,
        ) = data;
        for despawn_event in &despawn_events.0 {
            let entity = despawn_event.entity;
            if let (Some(points), Some(health)) =
                (points_on_last_hits.get(entity), healths.get(entity))
            {
                if let Some(last_hurt_by) = health.last_hurt_by {
                    if let Some(tags) = tags.get(last_hurt_by) {
                        // TODO: stop hardcoding these strings, use Enum for tags
                        if tags.0.contains(&"player") || tags.0.contains(&"player_weapon") {
                            player_score.increment(points.0);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct PlayerScore {
    current: i32,
    displayed: i32,
    factor: i32,
}
impl PlayerScore {
    pub fn new() -> PlayerScore {
        PlayerScore {
            current: 0,
            displayed: 0,
            factor: 10,
        }
    }
    pub fn reset(&mut self) {
        self.current = 0;
        self.displayed = 0;
    }
    pub fn reset_displayed(&mut self) {
        self.displayed = 0;
    }
    pub fn get(&self) -> i32 {
        self.current
    }
    pub fn set(&mut self, new_score: i32) {
        self.current = new_score;
    }
    pub fn increment(&mut self, amount: i32) {
        self.current += amount;
    }
    pub fn decrement(&mut self, amount: i32) {
        self.current -= amount;
        if self.current < 0 {
            self.current = 0;
        }
    }
    pub fn get_displayed(&self) -> i32 {
        self.displayed
    }
    pub fn update(&mut self) {
        if self.displayed == self.current {
            return;
        }
        if self.displayed < self.current {
            let incr = 1 + (self.current - self.displayed) / self.factor;
            self.displayed += incr;
            if self.displayed > self.current {
                self.displayed = self.current;
            }
        }
        if self.displayed > self.current {
            let decr = 1 + (self.displayed - self.current) / self.factor;
            self.displayed -= decr;
            if self.displayed < self.current {
                self.displayed = self.current;
            }
        }
    }
}
