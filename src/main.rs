extern crate ggez;
extern crate invaders;
extern crate rand;
extern crate specs;

use ggez::*;
use ggez::event::*;

use specs::*;

use invaders::*;
use invaders::plugins::*;

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_setup.title = String::from("Rust Invaders!");
    c.window_setup.samples = conf::NumSamples::Four;
    c.window_setup.resizable = true;

    let ctx = &mut Context::load_from_conf("invaders", "ggez", c).unwrap();
    ctx.print_resource_stats();

    match MainState::new(ctx) {
        Err(e) => {
            println!("Could not load game!");
            println!("Error: {}", e);
        }
        Ok(ref mut state) => {
            {
                let (width, height) = graphics::get_size(ctx);
                let mut viewport_state = state.world.write_resource::<viewport::ViewportState>();
                viewport_state.update_screen(width as f32, height as f32);
            }
            event::run(ctx, state).unwrap();
        }
    }
}

pub struct MainState<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    font: plugins::fonts::Font,
}

impl<'a, 'b> MainState<'a, 'b> {
    fn new(_ctx: &mut Context) -> GameResult<MainState<'a, 'b>> {
        let mut font = fonts::Font::new(&fonts::FUTURAL);
        if let Err(err) = font.load() {
            return Err(GameError::FontError(format!(
                "Failed to load font: {:?}",
                err
            )));
        }

        let mut world = World::new();

        // TODO: This seems ugly, find a better pattern?
        let dispatcher = DispatcherBuilder::new();
        let dispatcher = init(&mut world, dispatcher);
        let dispatcher = viewport::init(&mut world, dispatcher);
        let dispatcher = metadata::init(&mut world, dispatcher);
        let dispatcher = guns::init(&mut world, dispatcher);
        let dispatcher = thruster::init(&mut world, dispatcher);
        let dispatcher = collision::init(&mut world, dispatcher);
        let dispatcher = bounce::init(&mut world, dispatcher);
        let dispatcher = health_damage::init(&mut world, dispatcher);
        let dispatcher = player_control::init(&mut world, dispatcher);
        let dispatcher = simple_physics::init(&mut world, dispatcher);
        let dispatcher = position_motion::init(&mut world, dispatcher);
        let dispatcher = sprites::init(&mut world, dispatcher);
        let dispatcher = despawn::init(&mut world, dispatcher);
        let dispatcher = score::init(&mut world, dispatcher);
        let dispatcher = game::init(&mut world, dispatcher);
        let dispatcher = dispatcher.build();

        Ok(MainState {
            font,
            world,
            dispatcher,
        })
    }
}

impl<'a, 'b> event::EventHandler for MainState<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        update_delta_time(&mut self.world, ctx);
        viewport::update(&mut self.world, ctx)?;
        game::update(&mut self.world)?;
        self.dispatcher.dispatch(&self.world.res);
        score::update(&mut self.world)?;
        despawn::update(&mut self.world)?;
        self.world.maintain();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_background_color(ctx, graphics::BLACK);
        graphics::clear(ctx);
        graphics::set_color(ctx, graphics::WHITE)?;
        viewport::draw(&mut self.world, ctx)?;
        sprites::draw(&mut self.world, ctx)?;
        score::draw(&mut self.world, &mut self.font, ctx)?;
        game::draw(&mut self.world, &mut self.font, ctx)?;
        graphics::present(ctx);
        Ok(())
    }

    fn resize_event(&mut self, _ctx: &mut Context, width: u32, height: u32) {
        let mut viewport_state = self.world.write_resource::<viewport::ViewportState>();
        viewport_state.update_screen(width as f32, height as f32);
    }

    fn focus_event(&mut self, _ctx: &mut Context, gained: bool) {
        if gained {
        } else {
        }
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: i32, y: i32) {
        let mut viewport_state = self.world.write_resource::<viewport::ViewportState>();
        if y < 0 {
            viewport_state.decrease_zoom(0.1);
        } else if y > 0 {
            viewport_state.increase_zoom(0.1);
        }
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        player_control::key_down_event(&mut self.world, ctx, keycode, keymod, repeat);
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        player_control::key_up_event(&mut self.world, ctx, keycode, keymod, repeat);
    }

    fn controller_button_down_event(&mut self, ctx: &mut Context, btn: Button, instance_id: i32) {
        player_control::controller_button_down_event(&mut self.world, ctx, btn, instance_id);
    }

    fn controller_button_up_event(&mut self, ctx: &mut Context, btn: Button, instance_id: i32) {
        player_control::controller_button_up_event(&mut self.world, ctx, btn, instance_id);
    }

    fn controller_axis_event(
        &mut self,
        ctx: &mut Context,
        axis: Axis,
        value: i16,
        instance_id: i32,
    ) {
        player_control::controller_axis_event(&mut self.world, ctx, axis, value, instance_id);
    }
}
