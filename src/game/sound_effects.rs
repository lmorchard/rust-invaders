use rand::{thread_rng, Rng};
use specs::*;
use ggez::*;
use ggez::audio;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.add_resource(SoundEffectQueue::new());
    dispatcher
}

pub fn play(
    world: &mut World,
    ctx: &mut Context,
    sound_effects: &mut SoundEffects,
) -> GameResult<()> {
    let mut queue = world.write_resource::<SoundEffectQueue>();
    sound_effects.maintain();
    queue.execute(sound_effects, ctx)
}

pub struct SoundEffectEvent {
    effect_type: SoundEffectType,
}

pub struct SoundEffectQueue(pub Vec<SoundEffectEvent>);
impl SoundEffectQueue {
    pub fn new() -> SoundEffectQueue {
        SoundEffectQueue(Vec::new())
    }
    pub fn play(&mut self, effect_type: SoundEffectType) -> &mut Self {
        self.0.push(SoundEffectEvent { effect_type });
        self
    }
    pub fn execute(
        &mut self,
        sound_effects: &mut SoundEffects,
        ctx: &mut Context,
    ) -> GameResult<()> {
        for ref event in &self.0 {
            sound_effects.play(ctx, &event.effect_type)?;
        }
        self.0.clear();
        Ok(())
    }
}

#[derive(Debug)]
pub enum SoundEffectType {
    Ready,
    GameOver,
    Explosion,
    Shot,
    Shield,
    PlanetHit,
}

pub struct SoundEffects {
    pub currently_playing: Vec<audio::Source>,
    pub ready: Vec<audio::SoundData>,
    pub game_over: Vec<audio::SoundData>,
    pub explosions: Vec<audio::SoundData>,
    pub shots: Vec<audio::SoundData>,
    pub shields: Vec<audio::SoundData>,
    pub planethits: Vec<audio::SoundData>,
}
impl SoundEffects {
    pub fn new(ctx: &mut Context) -> GameResult<SoundEffects> {
        Ok(SoundEffects {
            currently_playing: Vec::new(),
            ready: vec![audio::SoundData::new(ctx, "/ready01.wav")?],
            game_over: vec![audio::SoundData::new(ctx, "/gameover01.wav")?],
            explosions: vec![
                audio::SoundData::new(ctx, "/explosion01.wav")?,
                audio::SoundData::new(ctx, "/explosion02.wav")?,
                audio::SoundData::new(ctx, "/explosion03.wav")?,
                audio::SoundData::new(ctx, "/explosion04.wav")?,
                audio::SoundData::new(ctx, "/explosion05.wav")?,
                audio::SoundData::new(ctx, "/explosion06.wav")?,
                audio::SoundData::new(ctx, "/explosion07.wav")?,
                audio::SoundData::new(ctx, "/explosion08.wav")?,
                audio::SoundData::new(ctx, "/explosion09.wav")?,
                audio::SoundData::new(ctx, "/explosion10.wav")?,
            ],
            shots: vec![
                audio::SoundData::new(ctx, "/shot01.wav")?,
                audio::SoundData::new(ctx, "/shot02.wav")?,
                audio::SoundData::new(ctx, "/shot03.wav")?,
                audio::SoundData::new(ctx, "/shot04.wav")?,
                audio::SoundData::new(ctx, "/shot05.wav")?,
                audio::SoundData::new(ctx, "/shot06.wav")?,
                audio::SoundData::new(ctx, "/shot07.wav")?,
                audio::SoundData::new(ctx, "/shot08.wav")?,
                audio::SoundData::new(ctx, "/shot09.wav")?,
                audio::SoundData::new(ctx, "/shot10.wav")?,
            ],
            shields: vec![
                audio::SoundData::new(ctx, "/shields01.wav")?,
                audio::SoundData::new(ctx, "/shields02.wav")?,
                audio::SoundData::new(ctx, "/shields03.wav")?,
                audio::SoundData::new(ctx, "/shields04.wav")?,
                audio::SoundData::new(ctx, "/shields05.wav")?,
                audio::SoundData::new(ctx, "/shields06.wav")?,
                audio::SoundData::new(ctx, "/shields07.wav")?,
                audio::SoundData::new(ctx, "/shields08.wav")?,
                audio::SoundData::new(ctx, "/shields09.wav")?,
                audio::SoundData::new(ctx, "/shields10.wav")?,
            ],
            planethits: vec![
                audio::SoundData::new(ctx, "/planethit01.wav")?,
                audio::SoundData::new(ctx, "/planethit02.wav")?,
                audio::SoundData::new(ctx, "/planethit03.wav")?,
                audio::SoundData::new(ctx, "/planethit04.wav")?,
                audio::SoundData::new(ctx, "/planethit05.wav")?,
                audio::SoundData::new(ctx, "/planethit06.wav")?,
                audio::SoundData::new(ctx, "/planethit07.wav")?,
                audio::SoundData::new(ctx, "/planethit08.wav")?,
                audio::SoundData::new(ctx, "/planethit09.wav")?,
                audio::SoundData::new(ctx, "/planethit10.wav")?,
            ],
        })
    }
    pub fn play(&mut self, ctx: &mut Context, effect_type: &SoundEffectType) -> GameResult<()> {
        let sound_data = thread_rng()
            .choose(match *effect_type {
                SoundEffectType::Ready => &self.ready,
                SoundEffectType::GameOver => &self.game_over,
                SoundEffectType::Explosion => &self.explosions,
                SoundEffectType::Shot => &self.shots,
                SoundEffectType::Shield => &self.shields,
                SoundEffectType::PlanetHit => &self.planethits,
            })
            .unwrap();
        if let Ok(mut source) = audio::Source::from_data(ctx, sound_data.clone()) {
            source.set_volume(0.25);
            source.play()?;
            self.currently_playing.push(source);
        }
        Ok(())
    }
    pub fn maintain(&mut self) {
        self.currently_playing.retain(|ref source| source.playing());
    }
}
