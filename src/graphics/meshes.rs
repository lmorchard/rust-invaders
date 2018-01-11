extern crate rand;

use std::f32::consts::PI;

use ggez::*;
use ggez::graphics::{DrawMode, Mesh, MeshBuilder, Point2};

use rand::Rng;

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
        .circle(
            DrawMode::Line(line_width),
            Point2::new(0.5, 0.5),
            0.5,
            0.05,
        )
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
            ]
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
