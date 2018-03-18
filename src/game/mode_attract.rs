use rand;

use specs::*;
use ggez::*;
use plugins::*;
use super::{prefabs, GameMode, GameModeManager};

pub fn init<'a, 'b>(
    _world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    dispatcher.add(AttractModeSystem, "attract_mode", &[])
}

pub struct AttractModeSystem;
impl<'a> System<'a> for AttractModeSystem {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, LazyUpdate>,
        FetchMut<'a, GameModeManager>,
        Fetch<'a, player_control::Inputs>,
        ReadStorage<'a, position_motion::Position>,
        ReadStorage<'a, collision::Collidable>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, lazy, mut game_mode, inputs, positions, collidables) = data;

        if game_mode.is_pending(GameMode::Attract) {
            for entity in entities.join() {
                entities.delete(entity);
            }
            prefabs::planet(entities.create(), &lazy);
        }

        if !game_mode.is_current(GameMode::Attract) {
            return;
        }

        if rand::random::<f32>() < 0.07 {
            prefabs::asteroid(&positions, &collidables, entities.create(), &lazy);
        }

        if inputs.fire {
            game_mode.change(GameMode::Playing);
        }
    }
}

pub fn draw(world: &mut World, font: &mut fonts::Font, ctx: &mut Context) -> GameResult<()> {
    let game_mode = world.read_resource::<GameModeManager>();
    if !game_mode.is_current(GameMode::Attract) {
        return Ok(());
    }

    // let viewport_state = world.read_resource::<viewport::ViewportState>();
    font.draw(
        ctx,
        "Rust Invaders v0.1\n<me@lmorchard.com>",
        fonts::DrawOptions {
            x: -500.0,
            y: -100.0,
            scale: 3.0,
            ..Default::default()
        },
    )?;

    Ok(())
}
