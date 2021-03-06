use specs::*;
use ggez::*;
use ggez::graphics::*;
use ::*;
use rand;

pub const PLAYFIELD_WIDTH: f32 = 1600.0;
pub const PLAYFIELD_HEIGHT: f32 = 900.0;
pub const PLAYFIELD_RATIO: f32 = PLAYFIELD_WIDTH / PLAYFIELD_HEIGHT;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.add_resource(ViewportState::new());
    dispatcher
}

pub fn update(world: &mut World, _ctx: &mut Context) -> GameResult<()> {
    let mut viewport_state = world.write_resource::<ViewportState>();
    let delta = world.read_resource::<DeltaTime>();
    if viewport_state.shake_duration > 0.0 {
        viewport_state.shake_duration -= delta.0;
    }
    Ok(())
}

pub fn draw(world: &mut World, ctx: &mut Context) -> GameResult<()> {
    let viewport_state = world.read_resource::<ViewportState>();
    let screen = viewport_state.screen;
    let mut coords = Rect::new(screen.x, screen.y, screen.w, screen.h);
    if viewport_state.shake_duration > 0.0 {
        coords.x += (0.5 - rand::random::<f32>()) * viewport_state.shake;
        coords.y += (0.5 - rand::random::<f32>()) * viewport_state.shake;
    }
    graphics::set_screen_coordinates(ctx, coords)?;
    Ok(())
}

#[derive(Debug)]
pub struct ViewportState {
    pub screen: Rect,
    pub zoom: f32,
    pub shake: f32,
    pub shake_duration: f32,
}
impl ViewportState {
    pub fn new() -> ViewportState {
        ViewportState {
            screen: Rect::new(0.0, 0.0, 100.0, 100.0),
            zoom: 1.0,
            shake: 0.0,
            shake_duration: 0.0,
        }
    }
    pub fn set_screen(&mut self, screen: Rect) {
        self.screen = screen;
    }
    pub fn shake(&mut self, shake: f32, shake_duration: f32) {
        self.shake = shake;
        self.shake_duration = shake_duration;
    }
    pub fn increase_zoom(&mut self, amount: f32) {
        self.zoom += amount;
        let (w, h) = {
            let screen = self.screen;
            (screen.w, screen.h)
        };
        self.update_screen(w, h);
    }
    pub fn decrease_zoom(&mut self, amount: f32) {
        self.zoom -= amount;
        if self.zoom <= 0.0 {
            self.zoom = 0.1;
        }
        let (w, h) = {
            let screen = self.screen;
            (screen.w, screen.h)
        };
        self.update_screen(w, h);
    }
    pub fn update_screen(&mut self, width: f32, height: f32) {
        let screen_ratio = width / height;
        let fit_ratio = if screen_ratio < PLAYFIELD_RATIO {
            PLAYFIELD_WIDTH / width
        } else {
            PLAYFIELD_HEIGHT / height
        };

        let (visible_width, visible_height) = (
            width * fit_ratio * (1.0 / self.zoom),
            height * fit_ratio * (1.0 / self.zoom),
        );
        let (visible_x, visible_y) = (0.0 - (visible_width / 2.0), 0.0 - (visible_height / 2.0));

        self.screen = Rect::new(visible_x, visible_y, visible_width, visible_height);
    }
}
