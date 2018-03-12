use std::f32::consts::PI;
use std::collections::{HashMap, HashSet};
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
        Default::default()
    }
}
impl Default for SpriteCache {
    fn default() -> SpriteCache {
        SpriteCache(HashMap::new())
    }
}

pub fn draw(world: &mut World, ctx: &mut Context) -> GameResult<()> {
    let entities = world.entities();
    let positions = world.read::<position_motion::Position>();
    let sprites = world.read::<Sprite>();
    let viewport_state = world.read_resource::<viewport::ViewportState>();
    let mut sprite_cache = world.write_resource::<SpriteCache>();

    let mut seen_entities: HashSet<Entity> = HashSet::new();

    for (ent, pos, spr) in (&*entities, &positions, &sprites).join() {
        let shape = &spr.shape;
        // TODO: Figure out a way to change line_width on zoom change without rebuilding all meshes
        let line_width = 1.0 / spr.scale.x; // / viewport_state.zoom;

        let mesh = sprite_cache
            .0
            .entry(ent)
            .or_insert_with(|| shape.build_mesh(ctx, line_width));
        seen_entities.insert(ent);

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

    // Clean up cached renderings for entities we didn't see during rendering
    let keys: Vec<Entity> = sprite_cache.0.keys().cloned().collect();
    for ent in keys {
        if !seen_entities.contains(&ent) {
            sprite_cache.0.remove(&ent);
        }
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
    Explosion,
    SimpleBullet,
    Planet,
    PlanetIcon,
}
impl Shape {
    pub fn build_mesh(&self, ctx: &mut Context, line_width: f32) -> Mesh {
        match *self {
            Shape::Explosion => explosion(ctx, line_width),
            Shape::Player => player(ctx, line_width),
            Shape::Asteroid => asteroid(ctx, line_width),
            Shape::SimpleBullet => simple_bullet(ctx, line_width),
            Shape::Planet => planet(ctx, line_width),
            Shape::PlanetIcon => planet_icon(ctx, line_width),
            _ => test(ctx, line_width),
        }
    }
}
impl Default for Shape {
    fn default() -> Shape {
        Shape::Test
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

pub fn planet_icon(ctx: &mut Context, line_width: f32) -> Mesh {
    MeshBuilder::new()
        .circle(
            DrawMode::Line(line_width),
            Point2::new(0.5, 0.5),
            0.5,
            0.001,
        )
        .build(ctx)
        .unwrap()
}

pub fn planet(ctx: &mut Context, line_width: f32) -> Mesh {
    let mut num_points = 100.0;
    let max_radius = 0.4975;
    let min_radius = 0.45;
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
        .circle(
            DrawMode::Line(line_width),
            Point2::new(0.5, 0.5),
            0.5,
            0.001,
        )
        .build(ctx)
        .unwrap()
}

pub fn explosion(ctx: &mut Context, line_width: f32) -> Mesh {
    MeshBuilder::new()
        .polygon(
            DrawMode::Line(line_width),
            &points![
                (0.5, 0.0),
                (0.55, 0.45),
                (1.0, 0.5),
                (0.55, 0.55),
                (0.5, 1.0),
                (0.45, 0.55),
                (0.0, 0.5),
                (0.45, 0.45)
            ],
        )
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
