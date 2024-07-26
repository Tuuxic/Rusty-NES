use crate::addr_utils::AddrUtils;
use crate::bus::bus::Bus;

use super::{
    cpu_flags::CpuFlags,
    operations::{addrmode::AddrMode, instruction::Instruction},
};

pub struct Cpu {
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

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
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

    pub fn reset(&mut self, bus: &mut Bus) {
        self.addr_abs = AddrUtils::CPU_START_ADDR;

        let lo: u16 = bus.cpu_read(self.addr_abs + 0) as u16;

        let hi: u16 = bus.cpu_read(self.addr_abs + 1) as u16;

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

    #[allow(unused)] // TODO: Remove unused
    fn irq(&mut self, bus: &mut Bus) {
        if self.get_flag(CpuFlags::I) != 0 {
            return;
        }

        // Save PC on stack

        bus.cpu_write(
            AddrUtils::CPU_STACK_BASE_ADDR + (self.stkp as u16),
            ((self.pc >> 8) & 0x00FF) as u8,
        );

        self.stkp -= 1;

        bus.cpu_write(
            AddrUtils::CPU_STACK_BASE_ADDR + (self.stkp as u16),
            (self.pc & 0x00FF) as u8,
        );

        self.stkp -= 1;

        self.set_flag(CpuFlags::B, false);

        self.set_flag(CpuFlags::U, true);

        self.set_flag(CpuFlags::I, true);

        bus.cpu_write(
            AddrUtils::CPU_STACK_BASE_ADDR + (self.stkp as u16),
            self.status,
        );

        self.stkp -= 1;

        self.addr_abs = 0xFFFE;

        let lo: u16 = bus.cpu_read(self.addr_abs) as u16;

        let hi: u16 = bus.cpu_read(self.addr_abs + 1) as u16;

        self.pc = (hi << 8) | lo;

        self.cycles = 7;
    }

    #[allow(unused)] // TODO: Remove unused
    fn nmi(&mut self, bus: &mut Bus) {
        bus.cpu_write(
            AddrUtils::CPU_STACK_BASE_ADDR + (self.stkp as u16),
            ((self.pc >> 8) & 0x00FF) as u8,
        );

        self.stkp -= 1;

        bus.cpu_write(
            AddrUtils::CPU_STACK_BASE_ADDR + (self.stkp as u16),
            (self.pc & 0x00FF) as u8,
        );

        self.stkp -= 1;

        self.set_flag(CpuFlags::B, false);

        self.set_flag(CpuFlags::U, true);

        self.set_flag(CpuFlags::I, true);

        bus.cpu_write(
            AddrUtils::CPU_STACK_BASE_ADDR + (self.stkp as u16),
            self.status,
        );

        self.stkp -= 1;

        self.addr_abs = 0xFFFA;

        let lo: u16 = bus.cpu_read(self.addr_abs) as u16;

        let hi: u16 = bus.cpu_read(self.addr_abs + 1) as u16;

        self.pc = (hi << 8) | lo;

        self.cycles = 8;
    }

    pub fn get_flag(&self, flag: CpuFlags) -> u8 {
        let CpuFlags(mask) = flag;

        if self.status & mask > 0 {
            1
        } else {
            0
        }
    }

    pub fn set_flag(&mut self, flag: CpuFlags, value: bool) {
        let CpuFlags(mask) = flag;

        if value {
            self.status |= mask;
        } else {
            self.status &= !mask;
        }
    }

    pub fn fetch(&mut self, bus: &mut Bus) -> u8 {
        if !matches!(
            Instruction::from_opcode(self.opcode).get_addrmode(),
            AddrMode::IMP
        ) {
            self.fetched = bus.cpu_read(self.addr_abs);
        }
        self.fetched
    }

    pub fn clock(&mut self, bus: &mut Bus) {
        if self.cycles <= 0 {
            self.opcode = bus.cpu_read(self.pc);

            self.set_flag(CpuFlags::U, true);

            self.pc += 1;

            let instr: Instruction = Instruction::from_opcode(self.opcode);

            self.cycles = instr.get_cycles();

            let add_cycles_addr: u8 = instr.execute_addrmode(self, bus); // lookup

            let add_cycles_op: u8 = instr.execute_operator(self, bus); // lookup

            self.cycles += add_cycles_addr & add_cycles_op;

            self.set_flag(CpuFlags::U, true);
        }

        self.clock_count += 1;

        if self.cycles != 0 {
            self.cycles -= 1;
        }
    }
}
