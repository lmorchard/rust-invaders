use ggez::graphics::{Mesh, Point2};

#[derive(Component, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub r: f32,
}

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
pub struct Friction {
    pub braking: f32,
}

#[derive(Component, Debug)]
pub struct SpeedLimit {
    pub max_speed: f32,
}

#[derive(Component, Debug)]
pub struct PlayerControlledThruster;

#[derive(Component, Debug)]
pub struct Sprite {
    pub scale: Point2,
    pub offset: Point2,
    pub mesh: Mesh,
}
