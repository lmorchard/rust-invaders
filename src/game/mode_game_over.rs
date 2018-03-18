use specs::*;
use ggez::*;
use plugins::*;
use DeltaTime;
use super::{GameMode, GameModeManager};

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.add_resource(GameOverModeState::new());
    dispatcher.add(GameOverModeSystem, "game_over_mode", &[])
}

pub struct GameOverModeState {
    attract_reset_ttl: f32,
}
impl GameOverModeState {
    pub fn new() -> GameOverModeState {
        GameOverModeState {
            attract_reset_ttl: 5.0,
        }
    }
    pub fn reset(&mut self) {
        self.attract_reset_ttl = 5.0;
    }
    pub fn update(&mut self, delta_time: f32) {
        if self.attract_reset_ttl > 0.0 {
            self.attract_reset_ttl -= delta_time;
        }
    }
}

pub struct GameOverModeSystem;
impl<'a> System<'a> for GameOverModeSystem {
    type SystemData = (
        Fetch<'a, DeltaTime>,
        FetchMut<'a, GameModeManager>,
        FetchMut<'a, GameOverModeState>,
        Fetch<'a, player_control::Inputs>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (delta, mut game_mode, mut game_over_state, inputs) = data;
        if !game_mode.is_current(GameMode::GameOver) {
            return;
        }
        game_over_state.update(delta.0);
        if inputs.fire {
            game_mode.change(GameMode::Playing);
        }
    }
}

pub fn draw(world: &mut World, font: &mut fonts::Font, ctx: &mut Context) -> GameResult<()> {
    let game_mode = world.read_resource::<GameModeManager>();
    if !game_mode.is_current(GameMode::GameOver) {
        return Ok(());
    }

    // let viewport_state = world.read_resource::<viewport::ViewportState>();
    font.draw(
        ctx,
        "Game Over!",
        fonts::DrawOptions {
            x: -500.0,
            y: -100.0,
            scale: 3.0,
            ..Default::default()
        },
    )?;

    Ok(())
}
