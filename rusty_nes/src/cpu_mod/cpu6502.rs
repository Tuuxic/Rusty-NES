use crate::{iodevice::IODevice, bus_mod::bus::CpuRAM};

use super::{
    cpu::Cpu,
    flags6502::Flags6502,
    instruction::Instruction,
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
    fn reset(&mut self) {
        self.addr_abs = 0xFFFC;

        let lo: u16 = self.read_from_bus(self.addr_abs + 0) as u16;

        let hi: u16 = self.read_from_bus(self.addr_abs + 1) as u16;

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

    fn irq(&mut self) {
        if self.get_flag(Flags6502::I) != 0 {
            return;
        }

        // Save PC on stack

        self.write_to_bus(0x0100 + (self.stkp as u16), ((self.pc >> 8) & 0x00FF) as u8);

        self.stkp -= 1;

        self.write_to_bus(0x0100 + (self.stkp as u16), (self.pc & 0x00FF) as u8);

        self.stkp -= 1;

        self.set_flag(Flags6502::B, false);

        self.set_flag(Flags6502::U, true);

        self.set_flag(Flags6502::I, true);

        self.write_to_bus(0x0100 + (self.stkp as u16), self.status);

        self.stkp -= 1;

        self.addr_abs = 0xFFFE;

        let lo: u16 = self.read_from_bus(self.addr_abs) as u16;

        let hi: u16 = self.read_from_bus(self.addr_abs + 1) as u16;

        self.pc = (hi << 8) | lo;

        self.cycles = 7;
    }

    fn nmi(&mut self) {
        self.write_to_bus(0x0100 + (self.stkp as u16), ((self.pc >> 8) & 0x00FF) as u8);

        self.stkp -= 1;

        self.write_to_bus(0x0100 + (self.stkp as u16), (self.pc & 0x00FF) as u8);

        self.stkp -= 1;

        self.set_flag(Flags6502::B, false);

        self.set_flag(Flags6502::U, true);

        self.set_flag(Flags6502::I, true);

        self.write_to_bus(0x0100 + (self.stkp as u16), self.status);

        self.stkp -= 1;

        self.addr_abs = 0xFFFA;

        let lo: u16 = self.read_from_bus(self.addr_abs) as u16;

        let hi: u16 = self.read_from_bus(self.addr_abs + 1) as u16;

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

    fn fetch(&self) {
        todo!()
    }

    fn imp(&mut self) -> u8 {
        self.fetched = self.a;

        0
    }

    fn imm(&mut self) -> u8 {
        self.addr_abs = self.pc;

        self.pc += 1;

        0
    }

    fn zp0(&mut self) -> u8 {
        self.addr_abs = self.read_from_bus(self.pc) as u16;

        self.pc += 1;

        self.addr_abs &= 0x00FF;

        0
    }

    fn zpx(&mut self) -> u8 {
        self.addr_abs = (self.read_from_bus(self.pc) + self.x) as u16;

        self.pc += 1;

        self.addr_abs &= 0x00FF;

        0
    }

    fn zpy(&mut self) -> u8 {
        self.addr_abs = (self.read_from_bus(self.pc) + self.y) as u16;

        self.pc += 1;

        self.addr_abs &= 0x00FF;

        0
    }

    fn rel(&mut self) -> u8 {
        self.addr_rel = self.read_from_bus(self.pc) as u16;

        self.pc += 1;

        if (self.addr_rel & 0x80) != 0 {
            self.addr_rel |= 0xFF00;
        }

        0
    }

    fn abs(&mut self) -> u8 {
        let lo: u16 = self.read_from_bus(self.pc) as u16;

        self.pc += 1;

        let hi: u16 = self.read_from_bus(self.pc) as u16;

        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;

        0
    }

    fn abx(&mut self) -> u8 {
        let lo: u16 = self.read_from_bus(self.pc) as u16;

        self.pc += 1;

        let hi: u16 = self.read_from_bus(self.pc) as u16;

        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;

        self.addr_abs += self.x as u16;

        if (hi << 8) != (self.addr_abs & 0xFF00) {
            1
        } else {
            0
        }
    }

    fn aby(&mut self) -> u8 {
        let lo: u16 = self.read_from_bus(self.pc) as u16;

        self.pc += 1;

        let hi: u16 = self.read_from_bus(self.pc) as u16;

        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;

        self.addr_abs += self.y as u16;

        if (hi << 8) != (self.addr_abs & 0xFF00) {
            1
        } else {
            0
        }
    }

    fn ind(&mut self) -> u8 {
        let ptr_lo: u16 = self.read_from_bus(self.pc) as u16;

        self.pc += 1;

        let ptr_hi: u16 = self.read_from_bus(self.pc) as u16;

        self.pc += 1;

        let ptr: u16 = (ptr_hi << 8) | ptr_lo;

        if ptr_lo == 0x00FF {
            let hi: u16 = self.read_from_bus(ptr & 0xFF00) as u16;

            let lo: u16 = self.read_from_bus(ptr) as u16;

            self.addr_abs = (hi << 8) | lo;
        } else {
            let hi: u16 = self.read_from_bus(ptr + 1) as u16;

            let lo: u16 = self.read_from_bus(ptr) as u16;

            self.addr_abs = (hi << 8) | lo;
        }

        0
    }

    fn izx(&mut self) -> u8 {
        let t: u16 = self.read_from_bus(self.pc) as u16;

        self.pc += 1;

        let lo: u16 = self.read_from_bus(((t + (self.x as u16)) as u16) & 0x00FF) as u16;

        let hi: u16 = self.read_from_bus(((t + (self.x as u16) + 1) as u16) & 0x00FF) as u16;

        self.addr_abs = (hi << 8) | lo;

        0
    }

    fn izy(&mut self) -> u8 {
        let t: u16 = self.read_from_bus(self.pc) as u16;

        self.pc += 1;

        let lo: u16 = self.read_from_bus(t & 0x00FF) as u16;

        let hi: u16 = self.read_from_bus((t + 1) & 0x00FF) as u16;

        self.addr_abs = (hi << 8) | lo;

        self.addr_abs += self.y as u16;

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    //Operators

    fn adc(&mut self) -> u8 {
        todo!()
    }

    fn and(&mut self) -> u8 {
        todo!()
    }

    fn asl(&mut self) -> u8 {
        todo!()
    }

    fn bcc(&mut self) -> u8 {
        todo!()
    }

    fn bcs(&mut self) -> u8 {
        todo!()
    }

    fn beq(&mut self) -> u8 {
        todo!()
    }

    fn bit(&mut self) -> u8 {
        todo!()
    }

    fn bmi(&mut self) -> u8 {
        todo!()
    }

    fn bne(&mut self) -> u8 {
        todo!()
    }

    fn bpl(&mut self) -> u8 {
        todo!()
    }

    fn brk(&mut self) -> u8 {
        todo!()
    }

    fn bvc(&mut self) -> u8 {
        todo!()
    }

    fn bvs(&mut self) -> u8 {
        todo!()
    }

    fn clc(&mut self) -> u8 {
        todo!()
    }

    fn cld(&mut self) -> u8 {
        todo!()
    }

    fn cli(&mut self) -> u8 {
        todo!()
    }

    fn clv(&mut self) -> u8 {
        todo!()
    }

    fn cmp(&mut self) -> u8 {
        todo!()
    }

    fn cpx(&mut self) -> u8 {
        todo!()
    }

    fn cpy(&mut self) -> u8 {
        todo!()
    }

    fn dec(&mut self) -> u8 {
        todo!()
    }

    fn dex(&mut self) -> u8 {
        todo!()
    }

    fn dey(&mut self) -> u8 {
        todo!()
    }

    fn eor(&mut self) -> u8 {
        todo!()
    }

    fn inc(&mut self) -> u8 {
        todo!()
    }

    fn inx(&mut self) -> u8 {
        todo!()
    }

    fn iny(&mut self) -> u8 {
        todo!()
    }

    fn jmp(&mut self) -> u8 {
        todo!()
    }

    fn jsr(&mut self) -> u8 {
        todo!()
    }

    fn lda(&mut self) -> u8 {
        todo!()
    }

    fn ldx(&mut self) -> u8 {
        todo!()
    }

    fn ldy(&mut self) -> u8 {
        todo!()
    }

    fn lsr(&mut self) -> u8 {
        todo!()
    }

    fn nop(&mut self) -> u8 {
        todo!()
    }

    fn ora(&mut self) -> u8 {
        todo!()
    }

    fn pha(&mut self) -> u8 {
        todo!()
    }

    fn php(&mut self) -> u8 {
        todo!()
    }

    fn pla(&mut self) -> u8 {
        todo!()
    }

    fn plp(&mut self) -> u8 {
        todo!()
    }

    fn rol(&mut self) -> u8 {
        todo!()
    }

    fn ror(&mut self) -> u8 {
        todo!()
    }

    fn rti(&mut self) -> u8 {
        todo!()
    }

    fn rts(&mut self) -> u8 {
        todo!()
    }

    fn sbc(&mut self) -> u8 {
        todo!()
    }

    fn sec(&mut self) -> u8 {
        todo!()
    }

    fn sed(&mut self) -> u8 {
        todo!()
    }

    fn sei(&mut self) -> u8 {
        todo!()
    }

    fn sta(&mut self) -> u8 {
        todo!()
    }

    fn stx(&mut self) -> u8 {
        todo!()
    }

    fn sty(&mut self) -> u8 {
        todo!()
    }

    fn tax(&mut self) -> u8 {
        todo!()
    }

    fn tay(&mut self) -> u8 {
        todo!()
    }

    fn tsx(&mut self) -> u8 {
        todo!()
    }

    fn txa(&mut self) -> u8 {
        todo!()
    }

    fn txs(&mut self) -> u8 {
        todo!()
    }

    fn tya(&mut self) -> u8 {
        todo!()
    }

    fn clock(&mut self, io: &mut IODevice) {

        if self.cycles <= 0 {

            self.opcode = io.read(self.pc);

            self.set_flag(Flags6502::U, true);

            self.pc += 1;

            let instr: Instruction = Instruction::from_opcode(self.opcode);

            self.cycles = instr.get_cycles();

            let add_cycles1: u8 = instr.execute_addmode(self, io); // lookup

            let add_cycles2: u8 = instr.execute_operator(self, io); // lookup

            self.cycles += add_cycles1 & add_cycles2;

            self.set_flag(Flags6502::U, true);
        }

        self.clock_count += 1;

        if self.cycles != 0 {
            self.cycles -= 1;
        }

        println!("cycle = {}", self.cycles);


    }



    fn read_from_bus(&self, addr: u16) -> u8 {
        0
    }

    fn write_to_bus(&mut self, addr: u16, data: u8) {

    }
}
