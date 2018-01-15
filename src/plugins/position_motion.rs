use specs::*;
use ggez::graphics::Rect;
use DeltaTime;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.register::<Position>();
    world.register::<PositionBounds>();
    world.register::<Velocity>();
    dispatcher
        .add(MotionSystem, "motion", &[])
        .add(PositionBoundsSystem, "position_bounds", &[])
}

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

pub struct MotionSystem;

impl<'a> System<'a> for MotionSystem {
    type SystemData = (
        Fetch<'a, DeltaTime>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (delta, vel, mut pos) = data;
        let delta = delta.0;
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * delta;
            pos.y += vel.y * delta;
            pos.r += vel.r * delta;
        }
    }
}

pub struct PositionBoundsSystem;

impl<'a> System<'a> for PositionBoundsSystem {
    type SystemData = (ReadStorage<'a, PositionBounds>, WriteStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        let (bounds, mut pos) = data;
        for (bounds, pos) in (&bounds, &mut pos).join() {
            let bounds = bounds.0;
            if pos.x < bounds.x {
                pos.x = bounds.x;
            } else if pos.x > bounds.x + bounds.w {
                pos.x = bounds.x + bounds.w;
            }
            if pos.y < bounds.y {
                pos.y = bounds.y;
            } else if pos.y > bounds.y + bounds.h {
                pos.y = bounds.y + bounds.h;
            }
        }
    }
}
