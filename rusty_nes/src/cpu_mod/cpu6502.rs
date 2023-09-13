use crate::iodevice::IODevice;

use super::{
    cpu::Cpu,
    flags6502::Flags6502,
    instruction::{Instruction, AddrMode},
};

pub struct Cpu6502 {
    pub a: u8,

    pub x: u8,

    pub y: u8,

    pub stkp: u8,

    pub pc: u16,

    pub status: u8,

    // internal helper

    pub fetched: u8,

    pub temp: u16,

    pub addr_abs: u16,

    pub addr_rel: u16,

    pub opcode: u8,

    pub cycles: u8,

    pub clock_count: u32,
}

impl Cpu6502 {
    pub fn new() -> Cpu6502 {
        Cpu6502 {
            a: 0x00,

            x: 0x00,

            y: 0x00,

            stkp: 0x00,

            pc: 0x0000,

            status: 0x00,

            fetched: 0x00,

            temp: 0x0000,

            addr_abs: 0x0000,

            addr_rel: 0x0000,

            opcode: 0x00,

            cycles: 0,

            clock_count: 0,
        }
    }
}

impl Cpu for Cpu6502 {
    fn reset(&mut self, io: &mut IODevice) {
        self.addr_abs = 0xFFFC;

        let lo: u16 = io.read(self.addr_abs + 0) as u16;

        let hi: u16 = io.read(self.addr_abs + 1) as u16;

        self.pc = (hi << 8) | lo;

        self.a = 0;

        self.x = 0;

        self.y = 0;

        self.stkp = 0xFD;

        self.status = (0x00 | lo) as u8;

        self.addr_rel = 0x0000;

        self.addr_abs = 0x0000;

        self.fetched = 0x00;

        self.cycles = 0;
    }

    fn irq(&mut self, io: &mut IODevice) {
        if self.get_flag(Flags6502::I) != 0 {
            return;
        }

        // Save PC on stack

        io.write(0x0100 + (self.stkp as u16), ((self.pc >> 8) & 0x00FF) as u8);

        self.stkp -= 1;

        io.write(0x0100 + (self.stkp as u16), (self.pc & 0x00FF) as u8);

        self.stkp -= 1;

        self.set_flag(Flags6502::B, false);

        self.set_flag(Flags6502::U, true);

        self.set_flag(Flags6502::I, true);

        io.write(0x0100 + (self.stkp as u16), self.status);

        self.stkp -= 1;

        self.addr_abs = 0xFFFE;

        let lo: u16 = io.read(self.addr_abs) as u16;

        let hi: u16 = io.read(self.addr_abs + 1) as u16;

        self.pc = (hi << 8) | lo;

        self.cycles = 7;
    }

    fn nmi(&mut self, io: &mut IODevice) {
        io.write(0x0100 + (self.stkp as u16), ((self.pc >> 8) & 0x00FF) as u8);

        self.stkp -= 1;

        io.write(0x0100 + (self.stkp as u16), (self.pc & 0x00FF) as u8);

        self.stkp -= 1;

        self.set_flag(Flags6502::B, false);

        self.set_flag(Flags6502::U, true);

        self.set_flag(Flags6502::I, true);

        io.write(0x0100 + (self.stkp as u16), self.status);

        self.stkp -= 1;

        self.addr_abs = 0xFFFA;

        let lo: u16 = io.read(self.addr_abs) as u16;

        let hi: u16 = io.read(self.addr_abs + 1) as u16;

        self.pc = (hi << 8) | lo;

        self.cycles = 8;
    }

    fn get_flag(&self, flag: Flags6502) -> u8 {
        let Flags6502(mask) = flag;

        if self.status & mask > 0 {
            1
        } else {
            0
        }
    }

    fn set_flag(&mut self, flag: Flags6502, value: bool) {
        let Flags6502(mask) = flag;

        if value {
            self.status |= mask;
        } else {
            self.status &= !mask;
        }
    }

    fn fetch(&mut self, io: &mut IODevice) -> u8{
        if !matches!(Instruction::from_opcode(self.opcode).get_addrmode(), AddrMode::IMP) {
            self.fetched = io.read(self.addr_abs);
        }
        self.fetched
    }

    fn clock(&mut self, io: &mut IODevice) {

        if self.cycles <= 0 {

            self.opcode = io.read(self.pc);

            self.set_flag(Flags6502::U, true);

            self.pc += 1;

            let instr: Instruction = Instruction::from_opcode(self.opcode);

            self.cycles = instr.get_cycles();

            let add_cycles1: u8 = instr.execute_addrmode(self, io); // lookup

            let add_cycles2: u8 = instr.execute_operator(self, io); // lookup

            self.cycles += add_cycles1 & add_cycles2;

            self.set_flag(Flags6502::U, true);
        }

        self.clock_count += 1;

        if self.cycles != 0 {
            self.cycles -= 1;
        }

    }

}
