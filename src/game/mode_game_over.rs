use specs::*;
use ggez::*;
use plugins::*;
use DeltaTime;
use super::{GameMode, GameModeManager};

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    dispatcher.add(GameOverModeSystem, "game_over_mode", &[])
}

pub struct GameOverModeSystem;
impl<'a> System<'a> for GameOverModeSystem {
    type SystemData = (
        FetchMut<'a, GameModeManager>,
        Fetch<'a, player_control::Inputs>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (mut game_mode, inputs) = data;
        if !game_mode.is_current(GameMode::GameOver) {
            return;
        }
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

    let viewport_state = world.read_resource::<viewport::ViewportState>();
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


