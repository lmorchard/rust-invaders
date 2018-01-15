use std::collections::HashMap;

use ggez::graphics::{Mesh, Point2, Rect};

use graphics::meshes::MeshSelection;

#[derive(Component, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub r: f32,
}
impl Default for Position {
    fn default() -> Position {
        Position {
            x: 0.0,
            y: 0.0,
            r: 0.0,
        }
    }
}

#[derive(Component, Debug)]
pub struct PositionBounds(pub Rect);

#[derive(Component, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub r: f32,
}
impl Velocity {
    pub fn new() -> Velocity {
        Velocity {
            ..Default::default()
        }
    }
}
impl Default for Velocity {
    fn default() -> Velocity {
        Velocity {
            x: 0.0,
            y: 0.0,
            r: 0.0,
        }
    }
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
    pub mesh_selection: MeshSelection,
    pub mesh: Option<Mesh>,
}
impl Default for Sprite {
    fn default() -> Sprite {
        Sprite {
            scale: Point2::new(100.0, 100.0),
            offset: Point2::new(0.5, 0.5),
            mesh_selection: MeshSelection::Test,
            mesh: None,
        }
    }
}

#[derive(Component, Debug)]
pub struct Gun {
    pub firing: bool,
    pub period: f32,
    pub cooldown: f32,
}
impl Default for Gun {
    fn default() -> Gun {
        Gun {
            firing: false,
            period: 1.0,
            cooldown: 0.0,
        }
    }
}
