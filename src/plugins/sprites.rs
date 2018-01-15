use std::f32::consts::PI;
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;
use specs::*;
use ggez::*;
use ggez::graphics::{DrawMode, DrawParam, Mesh, MeshBuilder, Point2};
use rand;
use rand::Rng;

use plugins::*;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.add_resource(SpriteCache::new());
    world.register::<Sprite>();
    dispatcher
}

pub struct SpriteCache(pub HashMap<Entity, Mesh>);
impl SpriteCache {
    pub fn new() -> Self {
        SpriteCache(HashMap::new())
    }
}
impl Deref for SpriteCache {
    type Target = HashMap<Entity, Mesh>;
    fn deref(&self) -> &HashMap<Entity, Mesh> {
        &self.0
    }
}
impl DerefMut for SpriteCache {
    fn deref_mut(&mut self) -> &mut HashMap<Entity, Mesh> {
        &mut self.0
    }
}

pub fn draw(world: &mut World, ctx: &mut Context) -> GameResult<()> {
    let entities = world.entities();
    let positions = world.read::<position_motion::Position>();
    let sprites = world.read::<Sprite>();
    let mut sprite_cache = world.write_resource::<SpriteCache>();

    // TODO: cache these per-sprite component! stateful asteroids
    for (ent, pos, spr) in (&*entities, &positions, &sprites).join() {
        let shape = &spr.shape;
        let line_width = 1.0 / spr.scale.x;
        let mesh = sprite_cache
            .entry(ent)
            .or_insert_with(|| sprites::build_mesh(shape, ctx, line_width));
        graphics::draw_ex(
            ctx,
            &*mesh,
            DrawParam {
                dest: Point2::new(pos.x, pos.y),
                rotation: pos.r,
                offset: spr.offset,
                scale: spr.scale,
                ..Default::default()
            },
        )?;
    }

    Ok(())
}

#[derive(Component, Debug)]
pub struct Sprite {
    pub scale: Point2,
    pub offset: Point2,
    pub shape: Shape,
}
impl Default for Sprite {
    fn default() -> Sprite {
        Sprite {
            scale: Point2::new(100.0, 100.0),
            offset: Point2::new(0.5, 0.5),
            shape: Shape::Test,
        }
    }
}

#[derive(Debug)]
pub enum Shape {
    Test,
    Player,
    Asteroid,
    SimpleBullet,
}

impl Default for Shape {
    fn default() -> Shape {
        Shape::Test
    }
}

pub fn build_mesh(shape: &Shape, ctx: &mut Context, line_width: f32) -> Mesh {
    match shape {
        &Shape::Player => player(ctx, line_width),
        &Shape::Asteroid => asteroid(ctx, line_width),
        &Shape::SimpleBullet => simple_bullet(ctx, line_width),
        _ => test(ctx, line_width),
    }
}

// TODO: Figure out if there's a better way to write this macro
macro_rules! points {
    ( $( $x:expr ), * ) => {
        {
            let mut temp_vec = Vec::new();
            $( temp_vec.push(Point2::new($x.0, $x.1)); )*
            temp_vec
        }
    };
}

pub fn test(ctx: &mut Context, line_width: f32) -> Mesh {
    MeshBuilder::new()
        .polygon(
            DrawMode::Line(line_width),
            &points![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        )
        .polygon(
            DrawMode::Line(line_width),
            &points![(0.5, 0.0), (1.0, 1.0), (0.0, 1.0)],
        )
        .circle(DrawMode::Line(line_width), Point2::new(0.5, 0.5), 0.5, 0.05)
        .line(&points![(0.4, 0.5), (0.6, 0.5)], line_width)
        .line(&points![(0.5, 0.4), (0.5, 0.6)], line_width)
        .build(ctx)
        .unwrap()
}

pub fn player(ctx: &mut Context, line_width: f32) -> Mesh {
    MeshBuilder::new()
        .polygon(
            DrawMode::Line(line_width),
            &points![
                (0.5, 0.0),
                (0.4375, 0.0),
                (0.25, 0.5),
                (0.125, 0.67),
                (0.25, 1.0),
                (0.375, 1.0),
                (0.4375, 0.75),
                (0.5625, 0.75),
                (0.625, 1.0),
                (0.75, 1.0),
                (0.875, 0.67),
                (0.75, 0.5),
                (0.5625, 0.0),
                (0.5, 0.0)
            ],
        )
        .build(ctx)
        .unwrap()
}

pub fn simple_bullet(ctx: &mut Context, line_width: f32) -> Mesh {
    MeshBuilder::new()
        .polygon(
            DrawMode::Line(line_width),
            &points![(0.5, 0.0), (0.6, 0.25), (0.5, 1.0), (0.4, 0.25), (0.5, 0.0)],
        )
        .build(ctx)
        .unwrap()
}

pub fn asteroid(ctx: &mut Context, line_width: f32) -> Mesh {
    let mut num_points = 7.0 + rand::thread_rng().gen_range(0.0, 12.0);
    let max_radius = 0.5;
    let min_radius = 0.3;
    let rotation_step = (PI * 2.0) / num_points;

    let mut points = Vec::new();
    let mut rotation: f32 = 0.0;
    loop {
        let distance = rand::thread_rng().gen_range(min_radius, max_radius);
        points.push(Point2::new(
            0.5 - distance * rotation.cos(),
            0.5 - distance * rotation.sin(),
        ));
        rotation += rotation_step;
        num_points -= 1.0;
        if num_points <= 0.0 {
            break;
        }
    }

    MeshBuilder::new()
        .polygon(DrawMode::Line(line_width), &points)
        .build(ctx)
        .unwrap()
}
