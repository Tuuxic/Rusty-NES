use std::time::Duration;

use crate::{bus_mod::bus::Bus, cpu_mod::{cpu::Cpu, cpu6502::Cpu6502}};

pub const FRAME_LENGTH: Duration = Duration::new(0, 16_666_666);

pub struct Nes {    
    bus: Bus,
    cpu: Cpu6502,
    //clock: &Clock

    frame_delta_time: f64
}

pub fn write_to_bus(_addr: u16, _data: u8) {

}

pub fn read_from_bus(_addr: u16) -> u8 {
    0
}

impl Nes {
    pub fn new() -> Nes{
        let bus: Bus = Bus::new();
        let cpu: Cpu6502 = Cpu6502::new(write_to_bus, read_from_bus);
        Nes {
            bus,
            cpu,
            frame_delta_time: 0.0
        }
    }


    pub fn update(&mut self, dt: Duration) {
        self.frame_delta_time += dt.as_secs_f64();
        if self.frame_delta_time > FRAME_LENGTH.as_secs_f64() {
            self.cpu.clock();
            self.frame_delta_time = 0.0;
        }
    }


    fn clock(&self) {
        //self.cpu.clock();
    }
}