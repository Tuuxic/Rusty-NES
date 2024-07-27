use std::{cell::RefCell, rc::Rc, time::Duration};

use crate::{cartridge::cartridge::Cartridge, cpu::cpu::Cpu};

pub const FRAME_LENGTH: Duration = Duration::from_millis(100);

pub struct Nes {
    pub cpu: Cpu,
    clock_counter: u64,
    frame_delta_time: f64,
}

impl Nes {
    pub fn new() -> Nes {
        let cpu = Cpu::new();
        Nes {
            cpu,
            clock_counter: 0,
            frame_delta_time: 0.0,
        }
    }

    pub fn insert_cartridge(&mut self, path: &str) {
        let cartridge: Rc<RefCell<Cartridge>> = Rc::new(RefCell::new(Cartridge::from_file(path)));
        self.cpu.bus.change_cartridge(cartridge);
        self.reset();
    }

    pub fn reset(&mut self) {
        self.cpu.reset()
    }

    pub fn update(&mut self, dt: Duration) {
        self.frame_delta_time += dt.as_secs_f64();
        if self.frame_delta_time > FRAME_LENGTH.as_secs_f64() {
            self.step();
            self.frame_delta_time = 0.0;
        }
    }

    pub fn step(&mut self) {
        while self.cpu.cycles == 0 {
            self.clock();
        }
        while self.cpu.cycles != 0 {
            self.clock();
        }
    }

    pub fn clock(&mut self) {
        self.cpu.clock_ppu();
        if self.clock_counter % 3 == 0 {
            self.cpu.clock();
        }
        self.clock_counter += 1;
    }
}
