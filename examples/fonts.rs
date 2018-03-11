extern crate ggez;
extern crate invaders;

use ggez::*;

use invaders::plugins::*;

fn main() {
    let mut c = conf::Conf::new();
    c.window_setup.title = String::from("Fonts - Rust Invaders!");
    c.window_setup.samples = conf::NumSamples::Eight;
    c.window_setup.resizable = true;

    let ctx = &mut Context::load_from_conf("fonts", "ggez", c).unwrap();

    graphics::set_background_color(ctx, (0, 0, 0, 255).into());

    let mut font = fonts::Font::new(&fonts::FUTURAL);
    font.load().unwrap();

    let state = &mut MainState::new(font).unwrap();
    event::run(ctx, state).unwrap();
}

struct MainState {
    font: fonts::Font,
}

impl MainState {
    fn new(font: fonts::Font) -> GameResult<MainState> {
        Ok(MainState { font })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        self.font.draw(
            ctx,
            "Rust Invaders! DANGER!\n<me@lmorchard.com>",
            fonts::DrawOptions {
                width: 600.0,
                ..Default::default()
            },
        )?;

        self.font.draw(
            ctx,
            "Whoo yay!\nI like pie so much.\nLet's BBQ some things",
            fonts::DrawOptions {
                x: 700.0,
                y: 300.0,
                scale: 1.5,
                reverse: true,
                ..Default::default()
            },
        )?;

        graphics::present(ctx);
        Ok(())
    }
}
