extern crate ggez;

mod assets;
mod actors;
mod animations;
mod controls;

use ggez::conf;
use ggez::timer;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

use assets::Assets;
use actors::player::Player;

// Main state
struct MainState {
    player: Player,
    assets: Assets
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);

        let state = MainState {
            assets: Assets::new(ctx)?,
            player: Player::new(ctx)?,
        };
        Ok(state)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if timer::get_ticks(ctx) % 100 == 0 {
            println!("Delta frame time: {:?} ", timer::get_delta(ctx));
            println!("Average FPS: {}", timer::get_fps(ctx));
        }

        self.player.update(timer::get_delta(ctx))?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        self.player.draw(ctx, &self.assets.warrior)?;

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: event::Keycode,
        _keymod: event::Mod,
        _repeat: bool
    ) {
        match keycode {
            event::Keycode::Up => self.player.controls.direction.y = -1.0,
            event::Keycode::Down => self.player.controls.direction.y = 1.0,
            event::Keycode::Left => self.player.controls.direction.x = -1.0,
            event::Keycode::Right => self.player.controls.direction.x = 1.0,
            _ => {}
        }
    }

    fn key_up_event(
        &mut self,
        _context: &mut Context,
        keycode: event::Keycode,
        _keymod: event::Mod,
        _repeat: bool
    ) {
        match keycode {
            event::Keycode::Up => self.player.controls.direction.y = 0.,
            event::Keycode::Down => self.player.controls.direction.y = 0.,
            event::Keycode::Left => self.player.controls.direction.x = 0.,
            event::Keycode::Right => self.player.controls.direction.x = 0.,
            _ => {}
        }
    }
}

fn main() {
    let c = conf::Conf::new();
    println!("Starting with default config: {:#?}", c);

    let ctx = &mut Context::load_from_conf("TopDown", "ggez", c).unwrap();

    match MainState::new(ctx) {
        Ok(ref mut game) => {
            let result = event::run(ctx, game);
            if let Err(e) = result {
                println!("Error encountered running game: {}", e);
            } else {
                println!("Game exited cleanly.");
            }
        }
        Err(e) => {
            println!("Could not load game!");
            println!("Error: {}", e);
        }
    }
}
