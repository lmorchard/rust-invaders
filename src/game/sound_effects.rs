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
    Explosion,
    Shot,
    Shield,
    PlanetHit,
}

pub struct SoundEffects {
    pub currently_playing: Vec<audio::Source>,
    pub explosions: Vec<audio::SoundData>,
    pub shots: Vec<audio::SoundData>,
    pub shields: Vec<audio::SoundData>,
    pub planethits: Vec<audio::SoundData>,
}
impl SoundEffects {
    pub fn new(ctx: &mut Context) -> GameResult<SoundEffects> {
        Ok(SoundEffects {
            currently_playing: Vec::new(),
            explosions: vec![audio::SoundData::new(ctx, "/boom.ogg")?],
            shots: vec![
                audio::SoundData::new(ctx, "/shot01.wav")?,
                audio::SoundData::new(ctx, "/shot02.wav")?,
                audio::SoundData::new(ctx, "/shot03.wav")?,
                audio::SoundData::new(ctx, "/shot04.wav")?,
                // audio::SoundData::new(ctx, "/pew.ogg")?,
            ],
            shields: vec![audio::SoundData::new(ctx, "/boom.ogg")?],
            planethits: vec![audio::SoundData::new(ctx, "/boom.ogg")?],
        })
    }
    pub fn play(&mut self, ctx: &mut Context, effect_type: &SoundEffectType) -> GameResult<()> {
        let sound_data = thread_rng()
            .choose(match *effect_type {
                SoundEffectType::Explosion => &self.explosions,
                SoundEffectType::Shot => &self.shots,
                SoundEffectType::Shield => &self.shields,
                SoundEffectType::PlanetHit => &self.planethits,
            })
            .unwrap();
        if let Ok(source) = audio::Source::from_data(ctx, sound_data.clone()) {
            source.play()?;
            self.currently_playing.push(source);
        }
        Ok(())
    }
    pub fn maintain(&mut self) {
        self.currently_playing.retain(|ref source| source.playing());
    }
}
