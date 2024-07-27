use std::env;

use ggez::{
    event::EventHandler,
    glam::Vec2,
    graphics::{Canvas, Color, Text},
    input::keyboard::KeyCode,
    Context, GameError, GameResult,
};

use crate::nes::Nes;

use super::cpu_debug::CpuDebug;

pub struct CpuView {
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

impl CpuView {
    pub fn new() -> GameResult<CpuView> {
        let mut nes = Nes::new();
        let args: Vec<String> = env::args().collect();
        let rom_file = &args[1];
        nes.insert_cartridge(rom_file);

        let debug = CpuDebug::new(&mut nes);

        let s: CpuView = CpuView {
            nes,
            debug,
            exec_state: ExecState::STEPPING,
        };
        Ok(s)
    }
}

impl EventHandler<GameError> for CpuView {
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
