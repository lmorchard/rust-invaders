use specs::*;
use ggez::*;
use ggez::graphics::*;
use plugins::*;

use super::{GameMode, GameModeManager, HeroPlanet, HeroPlayer};

pub fn init<'a, 'b>(
    _world: &mut World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    dispatcher
}

pub fn draw(world: &mut World, font: &mut fonts::Font, ctx: &mut Context) -> GameResult<()> {
    let game_mode = world.read_resource::<GameModeManager>();
    if !game_mode.is_current(GameMode::Playing) {
        return Ok(());
    }

    let viewport_state = world.read_resource::<viewport::ViewportState>();

    let player_score = world.read_resource::<score::PlayerScore>();
    font.draw(
        ctx,
        &format!("{:07}", player_score.get_displayed()),
        fonts::DrawOptions {
            x: viewport_state.screen.x + viewport_state.screen.w - 50.0,
            y: viewport_state.screen.y + 75.0,
            scale: 3.0,
            reverse: true,
            ..Default::default()
        },
    )?;

    let scale = 50.0;
    let base_x = viewport_state.screen.x + scale * 1.5;
    let base_y = viewport_state.screen.y + scale * 1.5;
    let planet_icon = sprites::Shape::PlanetIcon.build_mesh(ctx, 1.0 / scale);
    let player_icon = sprites::Shape::Player.build_mesh(ctx, 1.0 / scale);

    for (health, _planet) in (
        &world.read::<health_damage::Health>(),
        &world.read::<HeroPlanet>(),
    ).join()
    {
        draw_hud_gauge(
            ctx,
            scale,
            base_x,
            base_y,
            font,
            &planet_icon,
            100.0 * (health.health / health.max_health),
        )?;
    }

    for (health, _player) in (
        &world.read::<health_damage::Health>(),
        &world.read::<HeroPlayer>(),
    ).join()
    {
        draw_hud_gauge(
            ctx,
            scale,
            base_x,
            base_y + (scale * 1.5),
            font,
            &player_icon,
            100.0 * (health.health / health.max_health),
        )?;
    }

    Ok(())
}

pub fn draw_hud_gauge(
    ctx: &mut Context,
    scale: f32,
    base_x: f32,
    base_y: f32,
    _font: &mut fonts::Font,
    icon: &Mesh,
    perc: f32,
) -> GameResult<()> {
    let perc_scale = 4.0;
    graphics::draw_ex(
        ctx,
        &*icon,
        DrawParam {
            dest: Point2::new(base_x, base_y),
            rotation: 0.0,
            offset: Point2::new(0.5, 0.5),
            scale: Point2::new(scale, scale),
            ..Default::default()
        },
    )?;
    graphics::rectangle(
        ctx,
        graphics::DrawMode::Line(1.0),
        Rect::new(
            base_x + (scale * 1.125),
            base_y - (scale * 0.5),
            perc_scale * perc,
            scale,
        ),
    )?;
    Ok(())
}
