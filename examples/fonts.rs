extern crate ggez;
extern crate invaders;

use std::env;
use std::path;
use ggez::*;
use invaders::plugins::*;

fn main() {
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

    graphics::set_background_color(ctx, (0, 0, 0, 255).into());

    let mut font = fonts::Font::new(&fonts::FUTURAL);
    font.load(ctx).unwrap();

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
                x: 800.0,
                reverse: true,
                ..Default::default()
            },
        )?;

        graphics::present(ctx);
        Ok(())
    }
}
