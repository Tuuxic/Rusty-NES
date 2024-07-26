use crate::{bus::bus::Bus, cpu::cpu::Cpu};

use super::instruction::Operation;

#[derive(Clone, Copy)]
pub enum AddrMode {
    IMP,
    IMM,
    ZP0,
    ZPX,
    ZPY,
    REL,
    ABS,
    ABX,
    ABY,
    IND,
    IZX,
    IZY,
}

// Addressing Modes
pub struct IMP;
impl Operation for IMP {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        cpu.fetched = cpu.a;
        0
    }
}

pub struct IMM;
impl Operation for IMM {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        cpu.addr_abs = cpu.pc;
        cpu.pc += 1;
        0
    }
}

pub struct ZP0;
impl Operation for ZP0 {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.addr_abs = bus.cpu_read(cpu.pc) as u16;

        cpu.pc += 1;

        cpu.addr_abs &= 0x00FF;

        0
    }
}

pub struct ZPX;
impl Operation for ZPX {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.addr_abs = (bus.cpu_read(cpu.pc) + cpu.x) as u16;

        cpu.pc += 1;

        cpu.addr_abs &= 0x00FF;

        0
    }
}
pub struct ZPY;
impl Operation for ZPY {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.addr_abs = (bus.cpu_read(cpu.pc) + cpu.y) as u16;

        cpu.pc += 1;

        cpu.addr_abs &= 0x00FF;

        0
    }
}
pub struct REL;
impl Operation for REL {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.addr_rel = bus.cpu_read(cpu.pc) as u16;

        cpu.pc += 1;

        if (cpu.addr_rel & 0x80) != 0 {
            cpu.addr_rel |= 0xFF00;
        }

        0
    }
}
pub struct ABS;
impl Operation for ABS {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        let lo: u16 = bus.cpu_read(cpu.pc) as u16;

        cpu.pc += 1;

        let hi: u16 = bus.cpu_read(cpu.pc) as u16;

        cpu.pc += 1;

        cpu.addr_abs = (hi << 8) | lo;

        0
    }
}
pub struct ABX;
impl Operation for ABX {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        let lo: u16 = bus.cpu_read(cpu.pc) as u16;

        cpu.pc += 1;

        let hi: u16 = bus.cpu_read(cpu.pc) as u16;

        cpu.pc += 1;

        cpu.addr_abs = (hi << 8) | lo;

        cpu.addr_abs += cpu.x as u16;

        if (hi << 8) != (cpu.addr_abs & 0xFF00) {
            1
        } else {
            0
        }
    }
}
pub struct ABY;
impl Operation for ABY {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        let lo: u16 = bus.cpu_read(cpu.pc) as u16;

        cpu.pc += 1;

        let hi: u16 = bus.cpu_read(cpu.pc) as u16;

        cpu.pc += 1;

        cpu.addr_abs = (hi << 8) | lo;

        cpu.addr_abs += cpu.y as u16;

        if (hi << 8) != (cpu.addr_abs & 0xFF00) {
            1
        } else {
            0
        }
    }
}
pub struct IND;
impl Operation for IND {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        let ptr_lo: u16 = bus.cpu_read(cpu.pc) as u16;

        cpu.pc += 1;

        let ptr_hi: u16 = bus.cpu_read(cpu.pc) as u16;

        cpu.pc += 1;

        let ptr: u16 = (ptr_hi << 8) | ptr_lo;

        if ptr_lo == 0x00FF {
            let hi: u16 = bus.cpu_read(ptr & 0xFF00) as u16;

            let lo: u16 = bus.cpu_read(ptr) as u16;

            cpu.addr_abs = (hi << 8) | lo;
        } else {
            let hi: u16 = bus.cpu_read(ptr + 1) as u16;

            let lo: u16 = bus.cpu_read(ptr) as u16;

            cpu.addr_abs = (hi << 8) | lo;
        }

        0
    }
}
pub struct IZX;
impl Operation for IZX {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        let t: u16 = bus.cpu_read(cpu.pc) as u16;

        cpu.pc += 1;

        let lo: u16 = bus.cpu_read(((t + (cpu.x as u16)) as u16) & 0x00FF) as u16;

        let hi: u16 = bus.cpu_read(((t + (cpu.x as u16) + 1) as u16) & 0x00FF) as u16;

        cpu.addr_abs = (hi << 8) | lo;

        0
    }
}

pub struct IZY;
impl Operation for IZY {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        let t: u16 = bus.cpu_read(cpu.pc) as u16;

        cpu.pc += 1;

        let lo: u16 = bus.cpu_read(t & 0x00FF) as u16;

        let hi: u16 = bus.cpu_read((t + 1) & 0x00FF) as u16;

        cpu.addr_abs = (hi << 8) | lo;

        cpu.addr_abs += cpu.y as u16;

        if (cpu.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }
}
