extern crate invaders;
extern crate ggez;

use ggez::*;
use ggez::graphics::{
    BlendMode,
    Drawable,
    DrawMode,
    DrawParam,
    Mesh,
    MeshBuilder,
    Point2,
};

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

struct MainState {
    pos_x: f32,
    player_sprite: PlayerSprite,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        ctx.print_resource_stats();
        graphics::set_background_color(ctx, (0, 0, 0, 255).into());

        let s = MainState {
            pos_x: 0.0,
            player_sprite: PlayerSprite::new(ctx)
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::draw_ex(ctx, &self.player_sprite, DrawParam {
            dest: Point2::new(self.pos_x, 200.0),
            rotation: self.pos_x / 40.0,
            offset: Point2::new(0.5, 0.5),
            scale: Point2::new(100.0, 100.0),
            .. Default::default()
        })?;

        graphics::present(ctx);
        Ok(())
    }
}

struct PlayerSprite {
    shape: Mesh
}

impl PlayerSprite {
    fn new(ctx: &mut Context) -> Self {
        let shape = MeshBuilder::new()
            .polygon(DrawMode::Line(0.01), &points![
                (0.0, 0.0),
                (1.0, 0.0),
                (1.0, 1.0),
                (0.0, 1.0)
            ])
            .polygon(DrawMode::Line(0.01), &points![
                (0.5, 0.0),
                (1.0, 1.0),
                (0.0, 1.0)
            ])
            .line(&points![
                (0.4, 0.5),
                (0.6, 0.5)
            ], 0.01)
            .line(&points![
                (0.5, 0.4),
                (0.5, 0.6)
            ], 0.01)
            .build(ctx)
            .unwrap();

        PlayerSprite { shape }
    }
}

impl Drawable for PlayerSprite {
    fn get_blend_mode(&self) -> Option<BlendMode> { Some(BlendMode::Add) }
    fn set_blend_mode(&mut self, _mode: Option<BlendMode>) { }
    fn draw_ex(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        graphics::draw_ex(ctx, &self.shape, param)
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();

    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
