extern crate ggez;
extern crate invaders;

use std::fs::File;
use std::error::Error;
use std::io::BufReader;
use std::io::prelude::*;
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;

use std::f32;
use std::f32::consts::PI;

use ggez::*;
use ggez::graphics::{DrawMode, DrawParam, Mesh, MeshBuilder, Point2};

use invaders::*;
use invaders::plugins::*;

const SPACING: f32 = 80.0;
const ROTATION_SPEED: f32 = PI / 100.0;

fn main() {
    let mut c = conf::Conf::new();
    c.window_setup.title = String::from("Hershey - Rust Invaders!");
    c.window_setup.samples = conf::NumSamples::Eight;
    c.window_setup.resizable = true;

    let ctx = &mut Context::load_from_conf("example_meshes", "ggez", c).unwrap();

    graphics::set_background_color(ctx, (0, 0, 0, 255).into());

    let mut font = fonts::Font::new(&fonts::FUTURA_L);
    font.load();

    let state = &mut MainState::new(font).unwrap();
    event::run(ctx, state).unwrap();
}

struct MainState {
    rotation: f32,
    font: fonts::Font,
}

impl MainState {
    fn new(font: fonts::Font) -> GameResult<MainState> {
        Ok(MainState {
            rotation: 0.0,
            font
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.rotation = (self.rotation + ROTATION_SPEED) % (PI * 2.0);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let mut pos_x = 25.0;
        let mut pos_y = 50.0;

        let title: Vec<char> = "Rust Invaders! DANGER! 2112 <me@lmorchard.com>"
            .chars()
            .collect();

        let scale = 1.5;

        for idx in 0..title.len() {
            let key = title[idx];
            let (left, right) = self.font.get_glyph_margins(key);

            pos_x += (0.0 - left) * scale;

            self.font.draw_char(ctx, key, scale, pos_x, pos_y, 0.0);

            pos_x += right * scale;
            if pos_x >= 700.0 {
                pos_x = 25.0;
                pos_y += SPACING;
            }
        }

        graphics::present(ctx);
        Ok(())
    }
}
