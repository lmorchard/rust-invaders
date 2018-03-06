extern crate ggez;

use std::fs::File;
use std::error::Error;
use std::io::BufReader;
use std::io::prelude::*;
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;

use std::f32;
use std::f32::consts::PI;

use ggez::*;
use ggez::graphics::{MeshBuilder, DrawMode, DrawParam, Mesh, Point2};

static FONT_FILENAME: &'static str = "./hershey-fonts/futural.jhf";

const SPACING: f32 = 60.0;
const ROTATION_SPEED: f32 = PI / 100.0;

fn main() {
    let shapes = load_font(FONT_FILENAME).unwrap();

    {
        let mut k: Vec<&i32> = shapes.keys().collect();
        k.sort();
        println!("SHAPE {:?}", shapes.get(k[0]));
    };

    let mut c = conf::Conf::new();
    c.window_setup.title = String::from("Hershey - Rust Invaders!");
    c.window_setup.samples = conf::NumSamples::Eight;
    c.window_setup.resizable = true;

    let ctx = &mut Context::load_from_conf("example_meshes", "ggez", c).unwrap();

    graphics::set_background_color(ctx, (0, 0, 0, 255).into());

    let state = &mut MainState::new(shapes).unwrap();
    event::run(ctx, state).unwrap();
}

struct MainState {
    rotation: f32,
    shapes: FontShapes,
    meshes: Vec<Option<Mesh>>,
}

impl MainState {
    fn new(shapes: FontShapes) -> GameResult<MainState> {
        Ok(MainState {
            shapes,
            rotation: 0.0,
            meshes: Vec::new(),
        })
    }
}

pub fn build_mesh(ctx: &mut Context, scale: f32, left: &f32, right: &f32, lines: &Vec<Vec<Point2>>) -> Mesh {
    let line_width = 1.0 / scale;
    let mut builder = MeshBuilder::new();
    builder
        .line(&[Point2::new(*left, -8.0), Point2::new(*left, 8.0)], line_width / 2.0)
        .line(&[Point2::new(*right, -8.0), Point2::new(*right, 8.0)], line_width / 2.0)
        .circle(DrawMode::Line(line_width), Point2::new(0.0, 0.0), 0.5, 0.05);

    for line in lines {
        if !line.is_empty() {
            builder.polyline(DrawMode::Line(line_width), line);
        }
    }
    builder.build(ctx).unwrap()
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

        let shape_keys = {
            let mut k: Vec<&i32> = self.shapes.keys().collect();
            k.sort();
            k
        };

        for idx in 0..shape_keys.len() {
            let key = shape_keys[idx];
            let shape = self.shapes.get(&key).unwrap();
            let mesh = build_mesh(ctx, 2.0, &shape.left, &shape.right, &shape.lines);
            // let mesh = &self.meshes[idx].get_or_insert_with(|| build_mesh(ctx, 2.0, &shape.lines));

            //pos_x += (0.0 - shape.left);

            graphics::draw_ex(
                ctx,
                &mesh,
                DrawParam {
                    dest: Point2::new(pos_x, pos_y),
                    rotation: 0.0, //self.rotation,
                    offset: Point2::new(0.0, 0.0),
                    scale: Point2::new(2.0, 2.0),
                    ..Default::default()
                },
            )?;

            pos_x += SPACING;
            // pos_x += (0.0 - shape.left) + shape.right + 3.0;
            // pos_x += shape.right + 15.0;
            if pos_x >= 700.0 {
                pos_x = 25.0;
                pos_y += SPACING;
            }
        }

        graphics::present(ctx);
        Ok(())
    }
}

#[derive(Debug)]
pub struct FontShape {
    pub left: f32,
    pub right: f32,
    pub lines: Vec<Vec<Point2>>
}
impl FontShape {
    pub fn new(left: f32, right: f32) -> FontShape {
        let mut lines = Vec::new();
        lines.push(Vec::new());
        FontShape {
            left,
            right,
            lines
        }
    }
    pub fn add_line(&mut self) {
        self.lines.push(Vec::new());
    }
    pub fn add_point(&mut self, pt: Point2) {
        let last_idx = { self.lines.len() - 1 };
        self.lines[last_idx].push(pt);
    }
}

pub struct FontShapes(pub HashMap<i32, FontShape>);
impl FontShapes {
    pub fn new() -> FontShapes {
        Default::default()
    }
}
impl Default for FontShapes {
    fn default() -> FontShapes {
        FontShapes(HashMap::new())
    }
}
impl Deref for FontShapes {
    type Target = HashMap<i32, FontShape>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for FontShapes {
    fn deref_mut(&mut self) -> &mut HashMap<i32, FontShape> {
        &mut self.0
    }
}

pub fn char_to_coord(c: char) -> f32 {
    0.0 - (('R' as i8) - (c as i8)) as f32
}

pub fn load_font(filename: &str) -> Result<FontShapes, Box<Error>> {
    let f = File::open(filename)?;
    let mut shapes = FontShapes::new();

    for line_result in BufReader::new(f).lines() {
        let line = line_result?;
        let key = line.get(0..5).unwrap().trim().parse::<i32>().unwrap();
        let mut data = line.get(8..).unwrap().chars();

        let mut shape = FontShape::new(
            char_to_coord(data.next().unwrap()),
            char_to_coord(data.next().unwrap())
        );

        while let Some(cx) = data.next() {
            let cy = data.next().unwrap();
            if cx == ' ' && cy == 'R' {
                shape.add_line();
                continue;
            }
            shape.add_point(Point2::new(
                char_to_coord(cx),
                char_to_coord(cy)
            ));
        }
        shapes.insert(key, shape);
    }

    Ok(shapes)
}
