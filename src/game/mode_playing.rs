use std::f32::consts::PI;
use rand;

use specs::*;
use ggez::*;
use ggez::graphics::*;
use plugins::*;
use game::*;
use game::sound_effects::SoundEffectType;
use DeltaTime;

use super::{prefabs, reset_game, GameMode, GameModeManager, HeroPlanet, HeroPlayer};

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.add_resource(PlayingModeState::new());
    dispatcher.add(PlayingModeSystem, "playing_mode", &[])
}

pub fn draw(world: &mut World, font: &mut fonts::Font, ctx: &mut Context) -> GameResult<()> {
    let game_mode = world.read_resource::<GameModeManager>();
    if !game_mode.is_current(GameMode::Playing) {
        return Ok(());
    }

    let playing_state = world.read_resource::<PlayingModeState>();
    if playing_state.ready_delay > 0.0 {
        // let viewport_state = world.read_resource::<viewport::ViewportState>();
        font.draw(
            ctx,
            &format!("Ready {:1.2}", playing_state.ready_delay),
            fonts::DrawOptions {
                x: -300.0,
                y: -100.0,
                scale: 3.0,
                ..Default::default()
            },
        )?;
    }

    Ok(())
}

pub struct PlayingModeState {
    ready_delay: f32,
}
impl PlayingModeState {
    pub fn new() -> PlayingModeState {
        PlayingModeState { ready_delay: 1.0 }
    }
    pub fn reset(&mut self) {
        self.ready_delay = 1.0;
    }
    pub fn update(&mut self, delta_time: f32) {
        if self.ready_delay > 0.0 {
            self.ready_delay -= delta_time;
        }
    }
}

pub struct PlayingModeSystem;
impl<'a> System<'a> for PlayingModeSystem {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, LazyUpdate>,
        Fetch<'a, DeltaTime>,
        FetchMut<'a, health_damage::DamageEventQueue>,
        FetchMut<'a, despawn::DespawnEventQueue>,
        FetchMut<'a, sound_effects::SoundEffectQueue>,
        Fetch<'a, collision::Collisions>,
        FetchMut<'a, viewport::ViewportState>,
        FetchMut<'a, GameModeManager>,
        FetchMut<'a, PlayingModeState>,
        FetchMut<'a, score::PlayerScore>,
        FetchMut<'a, player_control::Inputs>,
        ReadStorage<'a, HeroPlanet>,
        ReadStorage<'a, HeroPlayer>,
        WriteStorage<'a, thruster::ThrusterSet>,
        WriteStorage<'a, guns::Gun>,
        ReadStorage<'a, position_motion::Position>,
        ReadStorage<'a, collision::Collidable>,
        ReadStorage<'a, sprites::Sprite>,
        ReadStorage<'a, metadata::Tags>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            lazy,
            delta,
            mut damages,
            mut despawns,
            mut sounds,
            collisions,
            mut viewport,
            mut game_mode,
            mut playing_state,
            mut player_score,
            mut inputs,
            hero_planets,
            hero_players,
            mut thruster_set,
            mut gun,
            positions,
            collidables,
            sprites,
            tags,
        ) = data;

        if game_mode.is_pending(GameMode::Playing) {
            reset_game(&entities, &mut inputs, true);
            playing_state.reset();
            player_score.reset();
            prefabs::player(entities.create(), &lazy);
            prefabs::planet(entities.create(), &lazy);
            sounds.play(SoundEffectType::Ready);
            game_mode.resolve();
            return;
        }

        if !game_mode.is_current(GameMode::Playing) {
            return;
        }

        playing_state.update(delta.0);

        if playing_state.ready_delay <= 0.0 {
            // Quick & dirty ramp up of difficulty relative to current score
            let asteroid_spawn_chance = 0.025 + (player_score.get() as f32 / 1500000.0);
            if rand::random::<f32>() < asteroid_spawn_chance {
                prefabs::asteroid(&positions, &collidables, entities.create(), &lazy);
            }
        }

        // TODO: Should be able to just count the entities here rather than looping.
        let mut hero_planet_alive = false;
        for (_entity, _hero_planet) in (&*entities, &hero_planets).join() {
            hero_planet_alive = true;
        }
        let mut hero_player_alive = false;
        for (_entity, _hero_player) in (&*entities, &hero_players).join() {
            hero_player_alive = true;
        }
        if !hero_planet_alive || !hero_player_alive {
            game_mode.change(GameMode::GameOver);
        }

        for (thruster_set, gun) in (&mut thruster_set, &mut gun).join() {
            gun.firing = inputs.fire;

            if let Some(lat_thruster) = thruster_set.0.get_mut("lateral") {
                lat_thruster.throttle = if inputs.right {
                    1.0
                } else if inputs.left {
                    -1.0
                } else {
                    0.0
                }
            }

            if let Some(long_thruster) = thruster_set.0.get_mut("longitudinal") {
                long_thruster.throttle = if inputs.up {
                    1.0
                } else if inputs.down {
                    -1.0
                } else {
                    0.0
                }
            }
        }

        for despawn_event in &despawns.0 {
            let entity = despawn_event.entity;
            if let (Some(tags), Some(position), Some(sprite)) =
                (tags.get(entity), positions.get(entity), sprites.get(entity))
            {
                for &tag in &tags.0 {
                    self.handle_despawn(
                        &entities,
                        &lazy,
                        &mut sounds,
                        &despawn_event,
                        tag,
                        &position,
                        &sprite,
                    );
                }
            }
        }

        for (a_entity, a_tags) in (&*entities, &tags).join() {
            for &a_tag in &a_tags.0 {
                if let Some(ent_collisions) = collisions.get(&a_entity) {
                    for b_entity in ent_collisions.iter() {
                        if let Some(b_tags) = tags.get(*b_entity) {
                            for &b_tag in &b_tags.0 {
                                self.handle_collision(
                                    &lazy,
                                    &mut player_score,
                                    &mut damages,
                                    &mut despawns,
                                    &mut sounds,
                                    &mut viewport,
                                    &a_tag,
                                    &b_tag,
                                    &a_entity,
                                    &b_entity,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

impl PlayingModeSystem {
    fn handle_despawn(
        &mut self,
        entities: &Entities,
        lazy: &LazyUpdate,
        sounds: &mut sound_effects::SoundEffectQueue,
        despawn_event: &despawn::DespawnEvent,
        tag: &str,
        position: &position_motion::Position,
        sprite: &sprites::Sprite,
    ) {
        if despawn_event.reason == despawn::DespawnReason::Health {
            if tag == "asteroid" {
                sounds.play(SoundEffectType::Explosion);

                let explosion = entities.create();
                lazy.insert(explosion, despawn::Timeout(0.5));
                lazy.insert(
                    explosion,
                    position_motion::Position {
                        x: position.x,
                        y: position.y,
                        ..Default::default()
                    },
                );
                lazy.insert(
                    explosion,
                    position_motion::Velocity {
                        r: PI * 7.0,
                        ..Default::default()
                    },
                );
                lazy.insert(
                    explosion,
                    sprites::Sprite {
                        shape: sprites::Shape::Explosion,
                        scale: Point2::new(sprite.scale.x, sprite.scale.y),
                        ..Default::default()
                    },
                );
            }
        }
    }

    fn handle_collision(
        &mut self,
        _lazy: &LazyUpdate,
        _player_score: &mut score::PlayerScore,
        damages: &mut health_damage::DamageEventQueue,
        _despawns: &mut despawn::DespawnEventQueue,
        sounds: &mut sound_effects::SoundEffectQueue,
        viewport: &mut viewport::ViewportState,
        a_tag: &str,
        b_tag: &str,
        a_entity: &Entity,
        b_entity: &Entity,
    ) {
        match (a_tag, b_tag) {
            ("asteroid", "player") => {
                damages.hurt_mutual(*a_entity, *b_entity, 100.0);
                viewport.shake(16.0, 0.3);
                sounds.play(SoundEffectType::Shield);
            }
            ("asteroid", "planet") => {
                damages.hurt_mutual(*a_entity, *b_entity, 100.0);
                viewport.shake(16.0, 0.3);
                sounds.play(SoundEffectType::PlanetHit);
            }
            ("asteroid", "asteroid") => {
                // damages.hurt_mutual(*a_entity, *b_entity, 10.0);
            }
            ("player_bullet", "enemy") => {
                damages.hurt_mutual(*a_entity, *b_entity, 100.0);
            }
            (&_, _) => (),
        }
    }
}
