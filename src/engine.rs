use std::env;

use ggez::event::{run, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, Text};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, ContextBuilder, GameError, GameResult};

use crate::debug::cpu_debug::CpuDebug;
use crate::nes::Nes;

const GAME_ID: &str = "RustyNes";
const AUTHOR_NAME: &str = "Nikolai Prjanikov";

struct MainState {
    nes: Nes,
    debug: CpuDebug,
    exec_state: ExecState,
}

#[derive(PartialEq)]
enum ExecState {
    STEPPING,
    RUN,
    UPDATE,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let mut nes = Nes::new();
        let args: Vec<String> = env::args().collect();
        let rom_file = &args[1];
        nes.insert_cartridge(rom_file);

        let debug = CpuDebug::new(&mut nes);

        let s: MainState = MainState {
            nes,
            debug,
            exec_state: ExecState::STEPPING,
        };
        Ok(s)
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let keyboard = &ctx.keyboard;
        if keyboard.is_key_just_pressed(KeyCode::M) {
            self.exec_state = if self.exec_state != ExecState::UPDATE {
                ExecState::UPDATE
            } else {
                ExecState::STEPPING
            };
        }
        if keyboard.is_key_just_pressed(KeyCode::B) {
            self.exec_state = if self.exec_state != ExecState::RUN {
                ExecState::RUN
            } else {
                ExecState::STEPPING
            };
        }
        if keyboard.is_key_just_pressed(KeyCode::N) {
            self.exec_state = ExecState::STEPPING;
        }

        match self.exec_state {
            ExecState::RUN => {
                self.nes.step();
            }
            ExecState::UPDATE => {
                self.nes.update(ctx.time.delta());
            }
            ExecState::STEPPING => {
                if keyboard.is_key_just_pressed(KeyCode::N) {
                    self.nes.step();
                }
            }
        };

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = Canvas::from_frame(_ctx, Color::BLACK);

        let code_txt = Text::new(self.debug.get_debug_code(&mut self.nes));
        let regs_txt = Text::new(self.debug.get_debug_registers(&mut self.nes));
        let ram1_txt = Text::new(self.debug.get_debug_ram(&mut self.nes, 0x0000, 16, 16));
        let cart_txt = Text::new(self.debug.get_debug_ram(&mut self.nes, 0x8000, 16, 16));

        canvas.draw(&code_txt, Vec2::new(500.0, 140.0));
        canvas.draw(&regs_txt, Vec2::new(500.0, 10.0));
        canvas.draw(&ram1_txt, Vec2::new(10.0, 10.0));
        canvas.draw(&cart_txt, Vec2::new(10.0, 285.0));
        canvas.finish(_ctx)?;
        Ok(())
    }
}

pub fn start() -> GameResult {
    let cb: ContextBuilder = ContextBuilder::new(GAME_ID, AUTHOR_NAME);
    let (ctx, event_loop) = cb.build()?;
    let state: MainState = MainState::new()?;

    run(ctx, event_loop, state);
}
