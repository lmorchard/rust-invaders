extern crate ggez;
extern crate invaders;

use ggez::*;
use ggez::graphics::{DrawParam, Mesh, Point2};

use std::f32;
use std::f32::consts::PI;

use invaders::graphics::meshes;

const SPACING: f32 = 150.0;
const ROTATION_SPEED: f32 = PI / 150.0;
const LINE_WIDTH: f32 = 0.01;

struct MainState {
    rotation: f32,
    meshes: Vec<Mesh>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let meshes = vec![
            meshes::test(ctx, LINE_WIDTH),
            meshes::player(ctx, LINE_WIDTH),
            meshes::asteroid(ctx, LINE_WIDTH),
            meshes::asteroid(ctx, LINE_WIDTH),
            meshes::asteroid(ctx, LINE_WIDTH),
            meshes::asteroid(ctx, LINE_WIDTH),
            meshes::asteroid(ctx, LINE_WIDTH),
        ];
        Ok(MainState {
            rotation: 0.0,
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

        for ref mesh in &self.meshes {
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

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("example_meshes", "ggez", c).unwrap();

    ctx.print_resource_stats();
    graphics::set_background_color(ctx, (0, 0, 0, 255).into());

    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
