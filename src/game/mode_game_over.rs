use specs::*;
use ggez::*;
use plugins::*;
use DeltaTime;
use super::{reset_game, GameMode, GameModeManager, HeroPlayer};

const RESET_TTL_MAX: f32 = 10.0;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.add_resource(GameOverModeState::new());
    dispatcher.add(GameOverModeSystem, "game_over_mode", &[])
}

pub struct GameOverModeState {
    reset_ttl: f32,
}
impl GameOverModeState {
    pub fn new() -> GameOverModeState {
        GameOverModeState {
            reset_ttl: RESET_TTL_MAX,
        }
    }
    pub fn reset(&mut self) {
        self.reset_ttl = RESET_TTL_MAX;
    }
    pub fn update(&mut self, delta_time: f32) {
        if self.reset_ttl > 0.0 {
            self.reset_ttl -= delta_time;
        }
    }
}

pub struct GameOverModeSystem;
impl<'a> System<'a> for GameOverModeSystem {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, DeltaTime>,
        FetchMut<'a, GameModeManager>,
        FetchMut<'a, GameOverModeState>,
        FetchMut<'a, player_control::Inputs>,
        FetchMut<'a, score::PlayerScore>,
        ReadStorage<'a, HeroPlayer>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            delta,
            mut game_mode,
            mut game_over_state,
            mut inputs,
            mut score,
            hero_players,
        ) = data;
        if game_mode.is_pending(GameMode::GameOver) {
            reset_game(&entities, &mut inputs, false);
            for (entity, _hero_player) in (&*entities, &hero_players).join() {
                if let Err(e) = entities.delete(entity) {
                    println!("Error deleting entity: {:?}", e);
                }
            }
            game_over_state.reset();
            // HACK: Reset displayed score.
            score.reset_displayed();
            game_mode.resolve();
            return;
        }
        if game_mode.is_current(GameMode::GameOver) {
            // HACK: Let displayed score tick back up to the high score before count down to reset.
            if score.get() == score.get_displayed() {
                game_over_state.update(delta.0);
                if game_over_state.reset_ttl <= 0.0 {
                    game_mode.change(GameMode::Attract);
                }
                if inputs.fire {
                    game_mode.change(GameMode::Playing);
                }
            }
        }
    }
}

pub fn draw(world: &mut World, font: &mut fonts::Font, ctx: &mut Context) -> GameResult<()> {
    let game_mode = world.read_resource::<GameModeManager>();
    if !game_mode.is_current(GameMode::GameOver) {
        return Ok(());
    }

    let player_score = world.read_resource::<score::PlayerScore>();
    // let viewport_state = world.read_resource::<viewport::ViewportState>();
    font.draw(
        ctx,
        &format!(
            "Game Over!\n\nFinal score:\n {:07}",
            player_score.get_displayed()
        ),
        fonts::DrawOptions {
            x: -250.0,
            y: -200.0,
            scale: 3.0,
            ..Default::default()
        },
    )?;

    Ok(())
}
