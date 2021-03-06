extern crate ggez;
extern crate invaders;
extern crate rand;
extern crate specs;

use std::env;
use std::path;
use ggez::*;
use ggez::event::*;
use specs::*;
use invaders::*;
use invaders::plugins::*;

pub fn main() {
    let mut cb = ContextBuilder::new("rustinvaders", "ggez")
        .window_setup(
            conf::WindowSetup::default()
                .title("Rust Invaders")
                .resizable(true)
                .samples(4)
                .unwrap(),
        )
        .window_mode(conf::WindowMode::default().dimensions(800, 600));

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        cb = cb.add_resource_path(path);
    } else {
        println!("Not building from cargo?  Ok.");
    }

    let ctx = &mut cb.build().unwrap();

    ctx.print_resource_stats();

    match MainState::new(ctx) {
        Err(e) => {
            println!("Could not load game!");
            println!("Error: {}", e);
        }
        Ok(ref mut state) => {
            {
                let (width, height) = graphics::get_size(ctx);
                let mut viewport = state.world.write_resource::<viewport::ViewportState>();
                viewport.update_screen(width as f32, height as f32);
            }
            event::run(ctx, state).unwrap();
        }
    }
}

pub struct MainState<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    font: plugins::fonts::Font,
    sound_effects: game::sound_effects::SoundEffects,
}

impl<'a, 'b> MainState<'a, 'b> {
    fn new(ctx: &mut Context) -> GameResult<MainState<'a, 'b>> {
        let mut font = fonts::Font::new(&fonts::FUTURAL);
        if let Err(err) = font.load(ctx) {
            return Err(GameError::FontError(format!(
                "Failed to load font: {:?}",
                err
            )));
        }

        let mut world = World::new();

        let mut dispatcher = DispatcherBuilder::new();
        let init_funcs = [
            init,
            viewport::init,
            metadata::init,
            guns::init,
            thruster::init,
            collision::init,
            bounce::init,
            health_damage::init,
            player_control::init,
            simple_physics::init,
            position_motion::init,
            sprites::init,
            despawn::init,
            score::init,
            game::init,
        ];
        for init_func in init_funcs.iter() {
            dispatcher = init_func(&mut world, dispatcher);
        }

        let sound_effects = game::sound_effects::SoundEffects::new(ctx)?;

        Ok(MainState {
            font,
            world,
            sound_effects,
            dispatcher: dispatcher.build(),
        })
    }
}

impl<'a, 'b> event::EventHandler for MainState<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        update_delta_time(&mut self.world, ctx);
        viewport::update(&mut self.world, ctx)?;
        self.dispatcher.dispatch(&self.world.res);
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
        game::draw(
            &mut self.world,
            ctx,
            &mut self.font,
            &mut self.sound_effects,
        )?;
        graphics::present(ctx);
        timer::yield_now();
        Ok(())
    }

    fn resize_event(&mut self, _ctx: &mut Context, width: u32, height: u32) {
        let mut viewport = self.world.write_resource::<viewport::ViewportState>();
        viewport.update_screen(width as f32, height as f32);
    }

    fn focus_event(&mut self, _ctx: &mut Context, gained: bool) {
        if gained {
        } else {
        }
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: i32, _y: i32) {
        /*
        let mut viewport = self.world.write_resource::<viewport::ViewportState>();
        if y < 0 {
            viewport.decrease_zoom(0.1);
        } else if y > 0 {
            viewport.increase_zoom(0.1);
        }
        */
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
