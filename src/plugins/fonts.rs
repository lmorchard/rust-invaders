extern crate ggez;

use std::fs::File;
use std::error::Error;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

use ggez::*;
use ggez::graphics::{DrawMode, DrawParam, Mesh, MeshBuilder, Point2};

use plugins::*;

const GLYPH_CHARS: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz 0123456789!?\"$/()|-+=*'#&\\^.,:;`[]{}<>~%@_";

pub struct FontMeta {
    pub filename: &'static str,
    pub glyph_ids: [i32; 95]
}

pub const FUTURA_L: FontMeta = FontMeta {
    filename: "./hershey-fonts/futural.jhf",
    glyph_ids: [
        501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 511, 512, 513, 514, 515, 516, 517, 518,
        519, 520, 521, 522, 523, 524, 525, 526, 601, 602, 603, 604, 605, 606, 607, 608, 609, 610,
        611, 612, 613, 614, 615, 616, 617, 618, 619, 620, 621, 622, 623, 624, 625, 626, 699, 700,
        701, 702, 703, 704, 705, 706, 707, 708, 709, 714, 715, 717, 719, 720, 721, 722, 723, 724,
        725, 726, 728, 731, 733, 734, 804, 832, 1210, 1211, 1212, 1213, 1252, 1405, 1406, 1407,
        1408, 2241, 2242, 2246, 2271, 2273, 12345,
    ]
};

#[derive(Debug)]
pub struct Glyph {
    pub left: f32,
    pub right: f32,
    pub lines: Vec<Vec<Point2>>,
}
impl Glyph {
    pub fn new(left: f32, right: f32) -> Glyph {
        let mut lines = Vec::new();
        lines.push(Vec::new());
        Glyph { left, right, lines }
    }
    pub fn add_line(&mut self) {
        self.lines.push(Vec::new());
    }
    pub fn add_point(&mut self, pt: Point2) {
        let last_idx = { self.lines.len() - 1 };
        self.lines[last_idx].push(pt);
    }
}

fn char_to_coord(c: char) -> f32 {
    0.0 - (('R' as i8) - (c as i8)) as f32
}

fn build_mesh(ctx: &mut Context, scale: f32, lines: &Vec<Vec<Point2>>) -> Mesh {
    let line_width = 1.0 / scale;
    let mut builder = MeshBuilder::new();
    for line in lines {
        if !line.is_empty() {
            builder.polyline(DrawMode::Line(line_width), &line);
        }
    }
    builder.build(ctx).unwrap()
}


pub struct Font {
    meta: &'static FontMeta,
    meshes: HashMap<char, Mesh>,
    glyphs: HashMap<char, Glyph>,
}
impl Font {
    pub fn new(meta: &'static FontMeta) -> Font {
        Font {
            meta,
            meshes: HashMap::new(),
            glyphs: HashMap::new(),
        }
    }

    pub fn get_glyph_margins(&self, c: char) -> (f32, f32) {
        let glyph = self.glyphs.get(&c).unwrap();
        (glyph.left, glyph.right)
    }

    pub fn draw_char(&mut self, ctx: &mut Context, c: char, scale: f32, x: f32, y: f32, rotation: f32) -> GameResult<()> {
        let glyph = self.glyphs.get(&c).unwrap();
        let mesh = self.meshes.entry(c).or_insert_with(|| build_mesh(ctx, scale, &glyph.lines));
        graphics::draw_ex(
            ctx,
            &*mesh,
            DrawParam {
                dest: Point2::new(x, y),
                rotation,
                offset: Point2::new(0.0, 0.0),
                scale: Point2::new(scale, scale),
                ..Default::default()
            },
        )
    }

    pub fn load(&mut self) -> Result<(), Box<Error>> {
        let f = File::open(self.meta.filename)?;

        let ids_to_chars: HashMap<&i32, char> =
            self.meta.glyph_ids.iter().zip(GLYPH_CHARS.chars()).collect();

        for line_result in BufReader::new(f).lines() {
            let line = line_result?;
            let id = line.get(0..5).unwrap().trim().parse::<i32>().unwrap();
            let key = ids_to_chars.get(&id).unwrap();
            let mut data = line.get(8..).unwrap().chars();

            let mut glyph = Glyph::new(
                char_to_coord(data.next().unwrap()),
                char_to_coord(data.next().unwrap()),
            );

            while let Some(cx) = data.next() {
                let cy = data.next().unwrap();
                if cx == ' ' && cy == 'R' {
                    glyph.add_line();
                    continue;
                }
                glyph.add_point(Point2::new(
                    char_to_coord(cx),
                    char_to_coord(cy)
                ));
            }
            self.glyphs.insert(*key, glyph);
        }
        Ok(())
    }
}
