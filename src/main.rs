extern crate ggez;
extern crate specs;

mod game;

use ggez::event;
use ggez::{Context, graphics};
use ggez::conf::{Conf, WindowMode};

use game::Game;

fn main() {
    let window_mode = WindowMode {
        width: 800,
        height: 600,
        borderless: true,
        vsync: false,
        ..Default::default()
    };
    let c = Conf {
        window_mode,
        ..Default::default()
    };
    let ctx = &mut Context::load_from_conf("config", "me", c).unwrap();
    graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);

    let mut state = Game::new(ctx).unwrap();
    println!("aqui!!!!!!!");
    event::run(ctx, &mut state).unwrap();
}
