use std::collections::HashMap;

use specs::{Entity};

use ggez::*;
use ggez::graphics::{Mesh, Point2, Rect};

#[derive(Component, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub r: f32,
}

#[derive(Component, Debug)]
pub struct PositionBounds(pub Rect);

#[derive(Component, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub r: f32,
}

#[derive(Component, Debug)]
pub struct Thruster {
    pub thrust: f32,
    pub throttle: f32,
    pub angle: f32,
}

#[derive(Component, Debug)]
pub struct ThrusterSet(pub HashMap<&'static str, Thruster>);

#[derive(Component, Debug)]
pub struct Friction(pub f32);

#[derive(Component, Debug)]
pub struct SpeedLimit(pub f32);

#[derive(Component, Debug)]
pub struct PlayerControl;

#[derive(Component, Debug)]
pub struct Sprite {
    pub scale: Point2,
    pub offset: Point2,
    pub mesh: Mesh,
}

#[derive(Component, Debug)]
pub struct Collidable {
    pub size: f32,
}

#[derive(Component, Debug)]
pub struct Gun {
    pub firing: bool,
    pub period: f32,
    pub cooldown: f32,
}
