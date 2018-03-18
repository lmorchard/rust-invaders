use ggez::*;
use ggez::event::{Axis, Button, Keycode, Mod};
use specs::*;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.register::<PlayerControl>();
    world.add_resource(Inputs::new());
    dispatcher
}

#[derive(Component, Debug)]
pub struct PlayerControl;

#[derive(Debug)]
pub struct Inputs {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub fire: bool,
    pub special: bool,
}
impl Inputs {
    pub fn new() -> Inputs {
        Inputs {
            left: false,
            right: false,
            up: false,
            down: false,
            fire: false,
            special: false,
        }
    }
    pub fn reset(&mut self) {
        self.left = false;
        self.right = false;
        self.up = false;
        self.down = false;
        self.fire = false;
        self.special = false;
    }
}

pub fn key_down_event(
    world: &mut World,
    _ctx: &mut Context,
    keycode: Keycode,
    _keymod: Mod,
    repeat: bool,
) {
    if repeat {
        return;
    }
    let mut inputs = world.write_resource::<Inputs>();
    match keycode {
        Keycode::Up | Keycode::W => inputs.up = true,
        Keycode::Down | Keycode::S => inputs.down = true,
        Keycode::Left | Keycode::A => inputs.left = true,
        Keycode::Right | Keycode::D => inputs.right = true,
        Keycode::Space => inputs.fire = true,
        Keycode::Return => inputs.special = true,
        _ => (),
    };
}

pub fn key_up_event(
    world: &mut World,
    _ctx: &mut Context,
    keycode: Keycode,
    _keymod: Mod,
    repeat: bool,
) {
    if repeat {
        return;
    }
    let mut inputs = world.write_resource::<Inputs>();
    match keycode {
        Keycode::Up | Keycode::W => inputs.up = false,
        Keycode::Down | Keycode::S => inputs.down = false,
        Keycode::Left | Keycode::A => inputs.left = false,
        Keycode::Right | Keycode::D => inputs.right = false,
        Keycode::Space => inputs.fire = false,
        Keycode::Return => inputs.special = false,
        _ => (),
    };
}

pub fn controller_button_down_event(
    _world: &mut World,
    _ctx: &mut Context,
    btn: Button,
    instance_id: i32,
) {
    println!(
        "Controller button pressed: {:?} Controller_Id: {}",
        btn, instance_id
    );
}

pub fn controller_button_up_event(
    _world: &mut World,
    _ctx: &mut Context,
    btn: Button,
    instance_id: i32,
) {
    println!(
        "Controller button released: {:?} Controller_Id: {}",
        btn, instance_id
    );
}

pub fn controller_axis_event(
    _world: &mut World,
    _ctx: &mut Context,
    axis: Axis,
    value: i16,
    instance_id: i32,
) {
    println!(
        "Axis Event: {:?} Value: {} Controller_Id: {}",
        axis, value, instance_id
    );
}
