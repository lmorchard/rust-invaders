extern crate ggez;
#[macro_use]
extern crate maplit;
extern crate rand;
extern crate specs;
#[macro_use]
extern crate specs_derive;

use specs::*;
use ggez::*;

pub mod game;
pub mod plugins;

pub fn init<'a, 'b>(
    world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.add_resource(DeltaTime(0.016));
    dispatcher
}

#[derive(Debug)]
pub struct DeltaTime(pub f32);

pub fn update_delta_time(world: &mut World, ctx: &mut Context) {
    let dt = ggez::timer::get_delta(ctx);
    let mut delta = world.write_resource::<DeltaTime>();
    *delta = DeltaTime(dt.as_secs() as f32 + dt.subsec_nanos() as f32 * 1e-9);
}
