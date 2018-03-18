use specs::*;
use ggez::*;
use ggez::graphics::*;
use plugins::*;
use DeltaTime;

use super::{prefabs, GameMode, GameModeManager, HeroPlayer, HeroPlanet};

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.add_resource(PlayingModeState::new());
    dispatcher.add(PlayingModeSystem, "playing_mode", &[])
}

pub struct PlayingModeState {
    ready_msg_ttl: f32
}
impl PlayingModeState {
    pub fn new() -> PlayingModeState {
        PlayingModeState {
            ready_msg_ttl: 3.0
        }
    }
    pub fn reset(&mut self) {
        self.ready_msg_ttl = 3.0;
    }
}

pub struct PlayingModeSystem;
impl<'a> System<'a> for PlayingModeSystem {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, LazyUpdate>,
        Fetch<'a, DeltaTime>,
        FetchMut<'a, GameModeManager>,
        FetchMut<'a, PlayingModeState>,
        Fetch<'a, player_control::Inputs>,
        ReadStorage<'a, HeroPlanet>,
        ReadStorage<'a, HeroPlayer>,
        WriteStorage<'a, thruster::ThrusterSet>,
        WriteStorage<'a, guns::Gun>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, lazy, delta, mut game_mode, mut playing_state, inputs, hero_planets, hero_players, mut thruster_set, mut gun) = data;

        if game_mode.is_pending(GameMode::Playing) {
            // Delete everything, add a new player and planet.
            for entity in entities.join() {
                entities.delete(entity);
            }
            prefabs::player(entities.create(), &lazy);
            prefabs::planet(entities.create(), &lazy);
            playing_state.reset();
        }

        if !game_mode.is_current(GameMode::Playing) {
            return;
        }

        if playing_state.ready_msg_ttl > 0.0 {
            playing_state.ready_msg_ttl -= delta.0;
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
    }
}

pub fn draw(world: &mut World, font: &mut fonts::Font, ctx: &mut Context) -> GameResult<()> {
    let game_mode = world.read_resource::<GameModeManager>();
    if !game_mode.is_current(GameMode::Playing) {
        return Ok(());
    }

    let playing_state = world.read_resource::<PlayingModeState>();
    if playing_state.ready_msg_ttl > 0.0 {
        let viewport_state = world.read_resource::<viewport::ViewportState>();
        font.draw(
            ctx,
            //"Ready Player One!",
            &format!("Ready {:07}", playing_state.ready_msg_ttl),
            fonts::DrawOptions {
                x: -500.0,
                y: -100.0,
                scale: 3.0,
                ..Default::default()
            },
        )?;
    }

    Ok(())
}

