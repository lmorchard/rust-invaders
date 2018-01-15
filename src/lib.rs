extern crate ggez;
extern crate rand;
extern crate specs;
#[macro_use]
extern crate specs_derive;

use specs::*;
use ggez::*;
use ggez::graphics::{Rect};

pub mod plugins;

pub const PLAYFIELD_WIDTH: f32 = 1600.0;
pub const PLAYFIELD_HEIGHT: f32 = 900.0;
pub const PLAYFIELD_RATIO: f32 = PLAYFIELD_WIDTH / PLAYFIELD_HEIGHT;

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

pub fn update_screen_coordinates(ctx: &mut Context, zoom: f32, width: u32, height: u32) {
    let width = width as f32;
    let height = height as f32;

    let screen_ratio = width / height;
    let fit_ratio = if screen_ratio < PLAYFIELD_RATIO {
        PLAYFIELD_WIDTH / width
    } else {
        PLAYFIELD_HEIGHT / height
    };

    let (visible_width, visible_height) = (
        width as f32 * fit_ratio * (1.0 / zoom),
        height as f32 * fit_ratio * (1.0 / zoom),
    );
    let (visible_x, visible_y) = (0.0 - (visible_width / 2.0), 0.0 - (visible_height / 2.0));

    let coords = Rect::new(visible_x, visible_y, visible_width, visible_height);
    graphics::set_screen_coordinates(ctx, coords).unwrap();
}
