extern crate ggez;
extern crate invaders;

// TODO
//
// - Copy over scaling logic from main.rs
// - reflow wrapping sprites based on screen dimensions

use ggez::*;
use ggez::graphics::{DrawParam, Mesh, Point2};

use std::f32;
use std::f32::consts::PI;

use invaders::plugins::sprites::*;

const SPACING: f32 = 150.0;
const ROTATION_SPEED: f32 = PI / 150.0;

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_setup.title = String::from("Meshes - Rust Invaders!");
    c.window_setup.samples = conf::NumSamples::Eight;
    c.window_setup.resizable = true;

    let ctx = &mut Context::load_from_conf("example_meshes", "ggez", c).unwrap();

    graphics::set_background_color(ctx, (0, 0, 0, 255).into());

    let state = &mut MainState::new().unwrap();
    event::run(ctx, state).unwrap();
}

struct MainState {
    rotation: f32,
    shapes: Vec<Shape>,
    meshes: Vec<Option<Mesh>>,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let shapes = vec![
            Shape::Test,
            Shape::Explosion,
            Shape::Player,
            Shape::SimpleBullet,
            Shape::Asteroid,
            Shape::Asteroid,
            Shape::Asteroid,
            Shape::Asteroid,
        ];
        let mut meshes = Vec::new();
        for _idx in 0..shapes.len() {
            meshes.push(None);
        }
        Ok(MainState {
            rotation: 0.0,
            shapes,
            meshes,
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

        let mut pos_x = 75.0;
        let mut pos_y = 75.0;

        for idx in 0..self.meshes.len() {
            let shape = &self.shapes[idx];
            let mesh = &self.meshes[idx].get_or_insert_with(|| shape.build_mesh(ctx, 1.0 / 100.0));
            graphics::draw_ex(
                ctx,
                *mesh,
                DrawParam {
                    dest: Point2::new(pos_x, pos_y),
                    rotation: self.rotation,
                    offset: Point2::new(0.5, 0.5),
                    scale: Point2::new(100.0, 100.0),
                    ..Default::default()
                },
            )?;
            pos_x += SPACING;
            if pos_x >= 800.0 {
                pos_x = 75.0;
                pos_y += SPACING;
            }
        }

        graphics::present(ctx);
        Ok(())
    }
}
