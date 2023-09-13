use std::
    time::Duration
;

use crate::{
    bus_mod::bus::CpuRAM,
    cpu_mod::{cpu::Cpu, cpu6502::Cpu6502}, iodevice::IODevice,
};

pub const FRAME_LENGTH: Duration = Duration::from_millis(500); // Duration::new(0, 16_666_666);

pub struct Nes {
    ram: CpuRAM,
    cpu: Cpu6502,
    //clock: &Clock
    frame_delta_time: f64,
}

impl Nes {
    pub fn new() -> Nes {
        let bus: CpuRAM = CpuRAM::new();
        let cpu: Cpu6502 = Cpu6502::new();
        Nes {
            ram: bus,
            cpu,
            frame_delta_time: 0.0,
        }
    }

    pub fn update(&mut self, dt: Duration) {
        self.frame_delta_time += dt.as_secs_f64();
        if self.frame_delta_time > FRAME_LENGTH.as_secs_f64() {
            self.clock();
            self.frame_delta_time = 0.0;
        }
    }

    pub fn get_debug(&self) -> String {
        let mut str: String = String::from(self.cpu.opcode.to_string());
        str.push_str(" a = ");
        str.push_str(&self.cpu.a.to_string());

        str.push_str(" x = ");
        str.push_str(&self.cpu.x.to_string());

        str.push_str(" y = ");
        str.push_str(&self.cpu.y.to_string());

        str.push_str(" cycle = ");
        str.push_str(&self.cpu.cycles.to_string());

        str
    }

    fn clock(&mut self) {
        let mut io = IODevice::new(&mut self.ram); 
        // self.cpu.clock(&mut self.ram);
        self.cpu.clock(&mut io);
    }
}
