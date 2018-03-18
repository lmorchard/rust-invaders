use specs::*;
use ggez::*;
use plugins::*;

pub mod hud;
pub mod mode_attract;
pub mod mode_game_over;
pub mod mode_playing;
pub mod prefabs;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.add_resource(GameModeManager::new());

    world.register::<HeroPlanet>();
    world.register::<HeroPlayer>();

    let dispatcher = hud::init(world, dispatcher);
    let dispatcher = mode_attract::init(world, dispatcher);
    let dispatcher = mode_playing::init(world, dispatcher);
    let dispatcher = mode_game_over::init(world, dispatcher);
    dispatcher
}

pub fn update(_world: &mut World) -> GameResult<()> {
    Ok(())
}

pub fn update_after(world: &mut World) -> GameResult<()> {
    world.write_resource::<GameModeManager>().update()?;
    Ok(())
}

pub fn draw(world: &mut World, font: &mut fonts::Font, ctx: &mut Context) -> GameResult<()> {
    mode_attract::draw(world, font, ctx)?;
    mode_playing::draw(world, font, ctx)?;
    mode_game_over::draw(world, font, ctx)?;
    hud::draw(world, font, ctx)?;
    Ok(())
}

#[derive(Component, Debug)]
pub struct HeroPlayer;

#[derive(Component, Debug)]
pub struct HeroPlanet;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum GameMode {
    Attract,
    Playing,
    GameOver,
}

pub struct GameModeManager {
    pub current_mode: GameMode,
    pub pending_mode: GameMode,
    pub activated: bool,
}
impl GameModeManager {
    pub fn new() -> GameModeManager {
        GameModeManager {
            current_mode: GameMode::Attract,
            pending_mode: GameMode::Attract,
            activated: false,
        }
    }
    pub fn change(&mut self, mode: GameMode) {
        self.pending_mode = mode;
        self.activated = false;
    }
    pub fn update(&mut self) -> GameResult<()> {
        self.current_mode = self.pending_mode;
        self.activated = true;
        Ok(())
    }
    pub fn is_pending(&self, mode: GameMode) -> bool {
        self.pending_mode == mode && !self.activated
    }
    pub fn is_current(&self, mode: GameMode) -> bool {
        self.current_mode == mode && self.activated
    }
}
