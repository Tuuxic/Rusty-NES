mod bus_mod;
mod cpu_mod;

use bus_mod::bus::Bus;
use ggez::event;
use ggez::{Context, GameResult, GameError, ContextBuilder};


const GAME_ID : &str = "RustyNes";
const AUTHOR_NAME : &str = "Nikolai Prjanikov";

struct MainState {
    nes: Bus
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s: MainState = MainState {
            nes: Bus::new()
        };
        Ok(s)
    }
}

impl event::EventHandler<GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

}

fn main() -> GameResult {
    let cb: ContextBuilder = ContextBuilder::new(GAME_ID, AUTHOR_NAME);
    let (ctx, event_loop) = cb.build()?;
    let state: MainState = MainState::new()?;

    event::run(ctx, event_loop, state)
}
