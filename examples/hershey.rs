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
use ggez::graphics::{DrawMode, DrawParam, Mesh, MeshBuilder, Point2};

static FONT_FILENAME: &'static str = "./hershey-fonts/futural.jhf";

const SPACING: f32 = 80.0;
const ROTATION_SPEED: f32 = PI / 100.0;

fn main() {
    let shapes = load_font(FONT_FILENAME).unwrap();

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
    meshes: HashMap<char, Mesh>,
}

impl MainState {
    fn new(shapes: FontShapes) -> GameResult<MainState> {
        Ok(MainState {
            shapes,
            rotation: 0.0,
            meshes: HashMap::new(),
        })
    }
}

pub fn build_mesh(ctx: &mut Context, scale: f32, lines: &Vec<Vec<Point2>>) -> Mesh {
    let line_width = 1.0 / scale;
    let mut builder = MeshBuilder::new();

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

        let title: Vec<char> = "Rust Invaders! DANGER! 0024 <me@lmorchard.com>"
            .chars()
            .collect();

        let _shape_keys = {
            let mut k: Vec<&char> = self.shapes.keys().collect();
            k.sort();
            k
        };

        let scale = 1.5;

        for idx in 0..title.len() {
            let key = title[idx];
            let shape = self.shapes.get(&key).unwrap();
            let mesh = self.meshes
                .entry(key)
                .or_insert_with(|| build_mesh(ctx, scale, &shape.lines));

            pos_x += (0.0 - shape.left) * scale;

            graphics::draw_ex(
                ctx,
                &*mesh,
                DrawParam {
                    dest: Point2::new(pos_x, pos_y),
                    rotation: 0.0,
                    // rotation: self.rotation,
                    offset: Point2::new(0.0, 0.0),
                    scale: Point2::new(scale, scale),
                    ..Default::default()
                },
            )?;

            pos_x += shape.right * scale;
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
    pub lines: Vec<Vec<Point2>>,
}
impl FontShape {
    pub fn new(left: f32, right: f32) -> FontShape {
        let mut lines = Vec::new();
        lines.push(Vec::new());
        FontShape { left, right, lines }
    }
    pub fn add_line(&mut self) {
        self.lines.push(Vec::new());
    }
    pub fn add_point(&mut self, pt: Point2) {
        let last_idx = { self.lines.len() - 1 };
        self.lines[last_idx].push(pt);
    }
}

pub struct FontShapes(pub HashMap<char, FontShape>);
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
    type Target = HashMap<char, FontShape>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for FontShapes {
    fn deref_mut(&mut self) -> &mut HashMap<char, FontShape> {
        &mut self.0
    }
}

pub fn char_to_coord(c: char) -> f32 {
    0.0 - (('R' as i8) - (c as i8)) as f32
}

pub fn load_font(filename: &str) -> Result<FontShapes, Box<Error>> {
    let f = File::open(filename)?;
    let mut shapes = FontShapes::new();

    let ids: [i32; 95] = [
        501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 511, 512, 513, 514, 515, 516, 517, 518,
        519, 520, 521, 522, 523, 524, 525, 526, 601, 602, 603, 604, 605, 606, 607, 608, 609, 610,
        611, 612, 613, 614, 615, 616, 617, 618, 619, 620, 621, 622, 623, 624, 625, 626, 699, 700,
        701, 702, 703, 704, 705, 706, 707, 708, 709, 714, 715, 717, 719, 720, 721, 722, 723, 724,
        725, 726, 728, 731, 733, 734, 804, 832, 1210, 1211, 1212, 1213, 1252, 1405, 1406, 1407,
        1408, 2241, 2242, 2246, 2271, 2273, 12345,
    ];

    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz 0123456789!?\"$/()|-+=*'#&\\^.,:;`[]{}<>~%@_";

    let ids_to_chars: HashMap<&i32, char> = ids.iter().zip(chars.chars()).collect();

    for line_result in BufReader::new(f).lines() {
        let line = line_result?;
        let id = line.get(0..5).unwrap().trim().parse::<i32>().unwrap();
        let key = ids_to_chars.get(&id).unwrap();
        let mut data = line.get(8..).unwrap().chars();

        let mut shape = FontShape::new(
            char_to_coord(data.next().unwrap()),
            char_to_coord(data.next().unwrap()),
        );

        while let Some(cx) = data.next() {
            let cy = data.next().unwrap();
            if cx == ' ' && cy == 'R' {
                shape.add_line();
                continue;
            }
            shape.add_point(Point2::new(char_to_coord(cx), char_to_coord(cy)));
        }
        shapes.insert(*key, shape);
    }

    Ok(shapes)
}
