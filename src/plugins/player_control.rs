use ggez::*;
use ggez::event::{Axis, Button, Keycode, Mod};
use specs::*;
use plugins::*;
use DeltaTime;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.register::<PlayerControl>();
    world.add_resource(Inputs {
        left: false,
        right: false,
        up: false,
        down: false,
        fire: false,
        special: false,
    });
    dispatcher.add(PlayerControlSystem, "player_control", &[])
}

#[derive(Debug)]
pub struct Inputs {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub fire: bool,
    pub special: bool,
}

pub fn key_down_event(
    world: &mut World,
    _ctx: &mut Context,
    keycode: Keycode,
    _keymod: Mod,
    _repeat: bool,
) {
    let mut inputs = world.write_resource::<Inputs>();
    match keycode {
        Keycode::Up => inputs.up = true,
        Keycode::W => inputs.up = true,
        Keycode::Down => inputs.down = true,
        Keycode::S => inputs.down = true,
        Keycode::Left => inputs.left = true,
        Keycode::A => inputs.left = true,
        Keycode::Right => inputs.right = true,
        Keycode::D => inputs.right = true,
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
    _repeat: bool,
) {
    let mut inputs = world.write_resource::<Inputs>();
    match keycode {
        Keycode::Up => inputs.up = false,
        Keycode::W => inputs.up = false,
        Keycode::Down => inputs.down = false,
        Keycode::S => inputs.down = false,
        Keycode::Left => inputs.left = false,
        Keycode::A => inputs.left = false,
        Keycode::Right => inputs.right = false,
        Keycode::D => inputs.right = false,
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

#[derive(Component, Debug)]
pub struct PlayerControl;

pub struct PlayerControlSystem;

impl<'a> System<'a> for PlayerControlSystem {
    type SystemData = (
        Fetch<'a, DeltaTime>,
        Fetch<'a, Inputs>,
        WriteStorage<'a, thruster::ThrusterSet>,
        WriteStorage<'a, guns::Gun>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (_delta, inputs, mut thruster_set, mut gun) = data;
        for (thruster_set, gun) in (&mut thruster_set, &mut gun).join() {
            gun.firing = inputs.fire;

            if let Some(lat_thruster) = thruster_set.0.get_mut("lateral") {
                lat_thruster.throttle = if inputs.right {
                    1.0
                } else if inputs.left {
                    -1.0
                } else {
                    0.0
                }
            }

            if let Some(long_thruster) = thruster_set.0.get_mut("longitudinal") {
                long_thruster.throttle = if inputs.up {
                    1.0
                } else if inputs.down {
                    -1.0
                } else {
                    0.0
                }
            }
        }
    }
}
