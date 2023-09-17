mod bus_mod;
mod cpu_mod;
mod iodevice;
mod nes;

use ggez::glam::Vec2;
use ggez::{event, graphics};
use ggez::{Context, ContextBuilder, GameError, GameResult};
use nes::Nes;

const GAME_ID: &str = "RustyNes";
const AUTHOR_NAME: &str = "Nikolai Prjanikov";

struct MainState {
    nes: Nes,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let mut nes = Nes::new();
        nes.init();
        let s: MainState = MainState { nes };
        Ok(s)
    }
}

impl event::EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        self.nes.update(ctx.time.delta());
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas =
            graphics::Canvas::from_frame(_ctx, graphics::Color::from([0.1, 0.2, 0.3, 0.1]));
        let str: String = self.nes.get_debug();
        let code_txt = graphics::Text::new(str);
        let register_txt = graphics::Text::new(self.nes.get_debug_registers());
        let ram1_txt = graphics::Text::new(self.nes.get_debug_ram(0x0000, 16, 16));
        let ram2_txt = graphics::Text::new(self.nes.get_debug_ram(0x8000, 16, 16));

        canvas.draw(&code_txt, Vec2::new(500.0, 140.0));
        canvas.draw(&register_txt, Vec2::new(500.0, 10.0));
        canvas.draw(&ram1_txt, Vec2::new(10.0, 10.0));
        canvas.draw(&ram2_txt, Vec2::new(10.0, 285.0));
        canvas.finish(_ctx)?;
        Ok(())
    }
}

#[allow(arithmetic_overflow)]
fn main() -> GameResult {
    let cb: ContextBuilder = ContextBuilder::new(GAME_ID, AUTHOR_NAME);
    let (ctx, event_loop) = cb.build()?;
    let state: MainState = MainState::new()?;

    event::run(ctx, event_loop, state)
}
