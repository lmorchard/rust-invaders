use specs::*;
use ggez::*;
use plugins::*;

pub mod hud;
pub mod mode_attract;
pub mod mode_game_over;
pub mod mode_playing;
pub mod prefabs;
pub mod sound_effects;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.add_resource(GameModeManager::new());

    world.register::<HeroPlanet>();
    world.register::<HeroPlayer>();

    let dispatcher = hud::init(world, dispatcher);
    let dispatcher = sound_effects::init(world, dispatcher);
    let dispatcher = mode_attract::init(world, dispatcher);
    let dispatcher = mode_playing::init(world, dispatcher);
    let dispatcher = mode_game_over::init(world, dispatcher);
    dispatcher
}

pub fn draw(
    world: &mut World,
    ctx: &mut Context,
    font: &mut fonts::Font,
    sound_effects: &mut sound_effects::SoundEffects,
) -> GameResult<()> {
    mode_attract::draw(world, font, ctx)?;
    mode_playing::draw(world, font, ctx)?;
    mode_game_over::draw(world, font, ctx)?;
    hud::draw(world, font, ctx)?;
    sound_effects::play(world, ctx, sound_effects)?;
    Ok(())
}

pub fn reset_game(
    entities: &Entities,
    inputs: &mut FetchMut<player_control::Inputs>,
    delete_entities: bool,
) {
    inputs.reset();
    if delete_entities {
        for entity in entities.join() {
            if let Err(e) = entities.delete(entity) {
                println!("Error deleting entity: {:?}", e);
            }
        }
    }
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
    pub resolved: bool,
}
impl GameModeManager {
    pub fn new() -> GameModeManager {
        GameModeManager {
            current_mode: GameMode::Attract,
            pending_mode: GameMode::Attract,
            resolved: false,
        }
    }
    pub fn change(&mut self, mode: GameMode) {
        self.pending_mode = mode;
        self.resolved = false;
    }
    pub fn resolve(&mut self) {
        self.current_mode = self.pending_mode;
        self.resolved = true;
    }
    pub fn is_pending(&self, mode: GameMode) -> bool {
        self.pending_mode == mode && !self.resolved
    }
    pub fn is_current(&self, mode: GameMode) -> bool {
        self.current_mode == mode && self.resolved
    }
}
