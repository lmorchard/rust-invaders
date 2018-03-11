use specs::*;
use ggez::*;
use plugins::*;
use ::*;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.add_resource(PlayerScore::new());
    dispatcher
}

pub fn update(world: &mut World) -> GameResult<()> {
    let mut player_score = world.write_resource::<PlayerScore>();
    player_score.update();
    Ok(())
}

pub fn draw(world: &mut World, font: &mut fonts::Font, ctx: &mut Context) -> GameResult<()> {
    let player_score = world.read_resource::<PlayerScore>();
    font.draw(
        ctx,
        &format!("{:07}", player_score.get_displayed()),
        fonts::DrawOptions {
            x: (PLAYFIELD_WIDTH / 2.0) - 24.0,
            y: 0.0 - (PLAYFIELD_HEIGHT / 2.0) + 24.0,
            scale: 3.0,
            reverse: true,
            ..Default::default()
        },
    )?;
    Ok(())
}

#[derive(Debug)]
pub struct PlayerScore {
    current: i32,
    displayed: i32,
    factor: i32,
}
impl PlayerScore {
    pub fn new() -> PlayerScore {
        PlayerScore {
            current: 0,
            displayed: 0,
            factor: 10
        }
    }
    pub fn get(&self) -> i32 {
        self.current
    }
    pub fn set(&mut self, new_score: i32) {
        self.current = new_score;
    }
    pub fn increment(&mut self, amount: i32) {
        self.current += amount;
    }
    pub fn decrement(&mut self, amount: i32) {
        self.current -= amount;
        if self.current < 0 {
            self.current = 0;
        }
    }
    pub fn get_displayed(&self) -> i32 {
        self.displayed
    }
    pub fn update(&mut self) {
        if self.displayed == self.current {
            return;
        }
        if self.displayed < self.current {
            let incr = 1 + (self.current - self.displayed) / self.factor;
            self.displayed += incr;
            if self.displayed > self.current {
                self.displayed = self.current;
            }
        }
        if self.displayed > self.current {
            let decr = 1 + (self.displayed - self.current) / self.factor;
            self.displayed -= decr;
            if self.displayed < self.current {
                self.displayed = self.current;
            }
        }
    }
}
