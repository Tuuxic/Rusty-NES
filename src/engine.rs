use ggez::glam::Vec2;
use ggez::graphics::Color;
use ggez::input::keyboard::KeyCode;
use ggez::{event, graphics};
use ggez::{Context, ContextBuilder, GameError, GameResult};

use crate::nes::Nes;

const GAME_ID: &str = "RustyNes";
const AUTHOR_NAME: &str = "Nikolai Prjanikov";

struct MainState {
    nes: Nes,
    stepping_state: bool,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let mut nes = Nes::new();
        nes.init();
        let s: MainState = MainState {
            nes,
            stepping_state: true,
        };
        Ok(s)
    }
}

impl event::EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        while ctx.time.check_update_time(60) {}
        let keyboard = &ctx.keyboard;
        if keyboard.is_key_just_pressed(KeyCode::M) {
            self.stepping_state = !self.stepping_state;
        }

        if self.stepping_state {
            if keyboard.is_key_just_pressed(KeyCode::N) {
                self.nes.step();
            }
        } else {
            self.nes.update(ctx.time.delta());
        }
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        while _ctx.time.check_update_time(60) {}
        let mut canvas = graphics::Canvas::from_frame(_ctx, Color::BLACK);
        let str: String = self.nes.get_debug_code();
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

pub fn start() -> GameResult {
    let cb: ContextBuilder = ContextBuilder::new(GAME_ID, AUTHOR_NAME);
    let (ctx, event_loop) = cb.build()?;
    let state: MainState = MainState::new()?;

    event::run(ctx, event_loop, state);
}
