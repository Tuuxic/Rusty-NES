use crate::iodevice::IODevice;

use super::{cpu::Cpu, cpu6502::Cpu6502, flags6502::Flags6502, instr_mapping::InstructionMapping};

pub struct Instruction {
    name: String,
    operator: Box<dyn ExecutableOperation>,
    addrmode: Box<dyn ExecutableOperation>,
    addrtype: AddrMode,
    cycles: u8,
}

/*
pub struct InstructionRepository {
    instruction_lookup: HashMap<u8, Instruction>
}
*/

impl Instruction {
    pub fn new(
        name: &str,
        operator: Box<dyn ExecutableOperation>,
        addrmode: Box<dyn ExecutableOperation>,
        addrtype: AddrMode,
        cycles: u8,
    ) -> Instruction {
        Instruction {
            name: name.to_string(),
            operator,
            addrmode,
            addrtype,
            cycles,
        }
    }

    pub fn from_opcode(opcode: u8) -> Instruction {
        InstructionMapping::opcode_to_instruction(opcode)
    }

    pub fn execute_operator(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        self.operator.execute(cpu, io)
    }

    pub fn execute_addrmode(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        self.addrmode.execute(cpu, io)
    }
    pub fn get_cycles(&self) -> u8 {
        self.cycles
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_addrmode(&self) -> AddrMode {
        self.addrtype
    }
    /* 
    pub fn opcode_to_addrmode(opcode: u8) -> AddrMode {
        return Instruction::from_opcode(opcode).get_addrmode();
    }
    */
}

pub trait ExecutableOperation {
    // fn execute(&self, cpu: &mut Cpu6502, bus: &mut CpuRAM) -> u8;
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8;
}

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
impl ExecutableOperation for IMP {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.fetched = cpu.a;
        0
    }
}

pub struct IMM;
impl ExecutableOperation for IMM {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.addr_abs = cpu.pc;
        cpu.pc += 1;
        0
    }
}

pub struct ZP0;
impl ExecutableOperation for ZP0 {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.addr_abs = io.read(cpu.pc) as u16;

        cpu.pc += 1;

        cpu.addr_abs &= 0x00FF;

        0
    }
}

pub struct ZPX;
impl ExecutableOperation for ZPX {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.addr_abs = (io.read(cpu.pc) + cpu.x) as u16;

        cpu.pc += 1;

        cpu.addr_abs &= 0x00FF;

        0
    }
}
pub struct ZPY;
impl ExecutableOperation for ZPY {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.addr_abs = (io.read(cpu.pc) + cpu.y) as u16;

        cpu.pc += 1;

        cpu.addr_abs &= 0x00FF;

        0
    }
}
pub struct REL;
impl ExecutableOperation for REL {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.addr_rel = io.read(cpu.pc) as u16;

        cpu.pc += 1;

        if (cpu.addr_rel & 0x80) != 0 {
            cpu.addr_rel |= 0xFF00;
        }

        0
    }
}
pub struct ABS;
impl ExecutableOperation for ABS {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        let lo: u16 = io.read(cpu.pc) as u16;

        cpu.pc += 1;

        let hi: u16 = io.read(cpu.pc) as u16;

        cpu.pc += 1;

        cpu.addr_abs = (hi << 8) | lo;

        0
    }
}
pub struct ABX;
impl ExecutableOperation for ABX {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        let lo: u16 = io.read(cpu.pc) as u16;

        cpu.pc += 1;

        let hi: u16 = io.read(cpu.pc) as u16;

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
impl ExecutableOperation for ABY {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        let lo: u16 = io.read(cpu.pc) as u16;

        cpu.pc += 1;

        let hi: u16 = io.read(cpu.pc) as u16;

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
impl ExecutableOperation for IND {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        let ptr_lo: u16 = io.read(cpu.pc) as u16;

        cpu.pc += 1;

        let ptr_hi: u16 = io.read(cpu.pc) as u16;

        cpu.pc += 1;

        let ptr: u16 = (ptr_hi << 8) | ptr_lo;

        if ptr_lo == 0x00FF {
            let hi: u16 = io.read(ptr & 0xFF00) as u16;

            let lo: u16 = io.read(ptr) as u16;

            cpu.addr_abs = (hi << 8) | lo;
        } else {
            let hi: u16 = io.read(ptr + 1) as u16;

            let lo: u16 = io.read(ptr) as u16;

            cpu.addr_abs = (hi << 8) | lo;
        }

        0
    }
}
pub struct IZX;
impl ExecutableOperation for IZX {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        let t: u16 = io.read(cpu.pc) as u16;

        cpu.pc += 1;

        let lo: u16 = io.read(((t + (cpu.x as u16)) as u16) & 0x00FF) as u16;

        let hi: u16 = io.read(((t + (cpu.x as u16) + 1) as u16) & 0x00FF) as u16;

        cpu.addr_abs = (hi << 8) | lo;

        0
    }
}

pub struct IZY;
impl ExecutableOperation for IZY {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        let t: u16 = io.read(cpu.pc) as u16;

        cpu.pc += 1;

        let lo: u16 = io.read(t & 0x00FF) as u16;

        let hi: u16 = io.read((t + 1) & 0x00FF) as u16;

        cpu.addr_abs = (hi << 8) | lo;

        cpu.addr_abs += cpu.y as u16;

        if (cpu.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }
}

// Operators
pub struct ADC;
impl ExecutableOperation for ADC {
    /*
    fn execute(&self, cpu: &mut Cpu6502, bus: &mut CpuRAM) -> u8 {
        println!("ADC executed");
        0
    }
    */
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);

        cpu.temp = cpu.a as u16 + cpu.fetched as u16 + cpu.get_flag(Flags6502::C) as u16;

        cpu.set_flag(Flags6502::C, cpu.temp > 255);
        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0);
        cpu.set_flag(
            Flags6502::V,
            (!(cpu.a as u16 ^ cpu.fetched as u16) & (cpu.a as u16 ^ cpu.temp as u16)) != 0,
        );
        cpu.set_flag(Flags6502::N, (cpu.temp & 0x80) != 0);

        cpu.a = (cpu.temp & 0x00FF) as u8;

        1
    }
}

pub struct SBC;
impl ExecutableOperation for SBC {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);

        let value: u16 = (cpu.fetched as u16) ^ 0x00FF;
        cpu.temp = cpu.a as u16 + value + cpu.get_flag(Flags6502::C) as u16;

        cpu.set_flag(Flags6502::C, (cpu.temp & 0xFF00) != 0);
        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0);
        cpu.set_flag(
            Flags6502::V,
            ((cpu.temp ^ cpu.a as u16) & (cpu.temp ^ value) & 0x0080) != 0,
        );
        cpu.set_flag(Flags6502::N, (cpu.temp & 0x0080) != 0);

        cpu.a = (cpu.temp & 0x00FF) as u8;

        1
    }
}

pub struct AND;
impl ExecutableOperation for AND {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);
        cpu.a = cpu.a & cpu.fetched;

        cpu.set_flag(Flags6502::Z, cpu.a == 0);
        cpu.set_flag(Flags6502::N, (cpu.a & 0x80) != 0);
        1
    }
}

pub struct ASL;
impl ExecutableOperation for ASL {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);
        cpu.temp = (cpu.fetched as u16) << 1;

        cpu.set_flag(Flags6502::C, (cpu.temp & 0xFF00) > 0);
        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.temp & 0x80) != 0);
        if matches!(
            Instruction::from_opcode(cpu.opcode).get_addrmode(),
            AddrMode::IMP
        ) {
            cpu.a = (cpu.temp & 0x00FF) as u8;
        } else {
            io.write(cpu.addr_abs, (cpu.temp & 0x00FF) as u8);
        }
        0
    }
}

pub struct BCC;
impl ExecutableOperation for BCC {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        if cpu.get_flag(Flags6502::C) == 0 {
            cpu.cycles += 1; // Maybe move to return value

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }

            cpu.pc = cpu.addr_abs
        }

        0
    }
}

pub struct BCS;
impl ExecutableOperation for BCS {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        if cpu.get_flag(Flags6502::C) == 1 {
            cpu.cycles += 1; // Maybe move to return value

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }

            cpu.pc = cpu.addr_abs
        }

        0
    }
}

pub struct BEQ;
impl ExecutableOperation for BEQ {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        if cpu.get_flag(Flags6502::Z) == 1 {
            cpu.cycles += 1; // Maybe move to return value

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }

            cpu.pc = cpu.addr_abs
        }

        0
    }
}

pub struct BIT;
impl ExecutableOperation for BIT {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);

        cpu.temp = (cpu.a & cpu.fetched) as u16;

        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.fetched & (1 << 7)) != 0);
        cpu.set_flag(Flags6502::V, (cpu.fetched & (1 << 6)) != 0);

        0
    }
}

pub struct BMI;
impl ExecutableOperation for BMI {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        if cpu.get_flag(Flags6502::N) == 1 {
            cpu.cycles += 1; // Maybe move to return value

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }

            cpu.pc = cpu.addr_abs
        }

        0
    }
}

pub struct BNE;
impl ExecutableOperation for BNE {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        if cpu.get_flag(Flags6502::Z) == 0 {
            cpu.cycles += 1; // Maybe move to return value

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }

            cpu.pc = cpu.addr_abs
        }

        0
    }
}

pub struct BPL;
impl ExecutableOperation for BPL {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        if cpu.get_flag(Flags6502::N) == 0 {
            cpu.cycles += 1; // Maybe move to return value

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }

            cpu.pc = cpu.addr_abs
        }

        0
    }
}

pub struct BRK;
impl ExecutableOperation for BRK {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.pc += 1;

        cpu.set_flag(Flags6502::I, true);
        io.write(0x0100 + (cpu.stkp as u16), ((cpu.pc >> 8) & 0x00FF) as u8);
        cpu.stkp -= 1;
        io.write(0x0100 + (cpu.stkp as u16), (cpu.pc & 0x00FF) as u8);
        cpu.stkp -= 1;

        cpu.set_flag(Flags6502::B, true);
        io.write(0x0100 + (cpu.stkp as u16), cpu.status);
        cpu.stkp -= 1;
        cpu.set_flag(Flags6502::B, false);

        cpu.pc = io.read(0xFFFE) as u16 | ((io.read(0xFFFF) as u16) << 8);
        0
    }
}

pub struct BVC;
impl ExecutableOperation for BVC {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        if cpu.get_flag(Flags6502::V) == 0 {
            cpu.cycles += 1; // Maybe move to return value

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }

            cpu.pc = cpu.addr_abs
        }

        0
    }
}

pub struct BVS;
impl ExecutableOperation for BVS {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        if cpu.get_flag(Flags6502::V) == 1 {
            cpu.cycles += 1; // Maybe move to return value

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }

            cpu.pc = cpu.addr_abs
        }

        0
    }
}

pub struct CLC;
impl ExecutableOperation for CLC {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.set_flag(Flags6502::C, false);
        0
    }
}

pub struct CLD;
impl ExecutableOperation for CLD {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.set_flag(Flags6502::D, false);
        0
    }
}

pub struct CLI;
impl ExecutableOperation for CLI {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.set_flag(Flags6502::I, false);
        0
    }
}

pub struct CLV;
impl ExecutableOperation for CLV {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.set_flag(Flags6502::V, false);
        0
    }
}

pub struct CMP;
impl ExecutableOperation for CMP {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);
        cpu.temp = cpu.a as u16 - cpu.fetched as u16;

        cpu.set_flag(Flags6502::C, cpu.a >= cpu.fetched);
        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(Flags6502::N, (cpu.temp & 0x0080) != 0);
        1
    }
}

pub struct CPX;
impl ExecutableOperation for CPX {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);
        cpu.temp = cpu.x as u16 - cpu.fetched as u16;

        cpu.set_flag(Flags6502::C, cpu.x >= cpu.fetched);
        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(Flags6502::N, (cpu.temp & 0x0080) != 0);
        0
    }
}

pub struct CPY;
impl ExecutableOperation for CPY {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);
        cpu.temp = cpu.y as u16 - cpu.fetched as u16;

        cpu.set_flag(Flags6502::C, cpu.y >= cpu.fetched);
        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(Flags6502::N, (cpu.temp & 0x0080) != 0);
        0
    }
}

pub struct DEC;
impl ExecutableOperation for DEC {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);
        cpu.temp = (cpu.fetched - 1) as u16;
        io.write(cpu.addr_abs, (cpu.temp & 0x00FF) as u8);
        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(Flags6502::N, (cpu.temp & 0x0080) != 0);
        0
    }
}

pub struct DEX;
impl ExecutableOperation for DEX {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.x -= 1;
        cpu.set_flag(Flags6502::Z, cpu.x == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.x & 0x80) != 0);
        0
    }
}

pub struct DEY;
impl ExecutableOperation for DEY {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.y -= 1;
        cpu.set_flag(Flags6502::Z, cpu.y == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.y & 0x80) != 0);
        0
    }
}

pub struct EOR;
impl ExecutableOperation for EOR {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);
        cpu.a = cpu.a ^ cpu.fetched;

        cpu.set_flag(Flags6502::Z, cpu.a == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.a & 0x80) != 0);

        1
    }
}

pub struct INC;
impl ExecutableOperation for INC {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);

        cpu.temp = (cpu.fetched as u16) + 1;
        io.write(cpu.addr_abs, (cpu.temp & 0x00FF) as u8);
        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(Flags6502::N, (cpu.temp & 0x0080) != 0);

        0
    }
}

pub struct INX;
impl ExecutableOperation for INX {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.x += 1;
        cpu.set_flag(Flags6502::Z, cpu.x == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.x & 0x80) != 0);

        0
    }
}

pub struct INY;
impl ExecutableOperation for INY {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.y += 1;
        cpu.set_flag(Flags6502::Z, cpu.y == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.y & 0x80) != 0);

        0
    }
}

pub struct JMP;
impl ExecutableOperation for JMP {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.pc = cpu.addr_abs;
        0
    }
}

pub struct JSR;
impl ExecutableOperation for JSR {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.pc -= 1;

        io.write(0x0100 + (cpu.stkp as u16), ((cpu.pc >> 8) & 0x00FF) as u8);
        cpu.stkp -= 1;

        io.write(0x0100 + (cpu.stkp as u16), (cpu.pc & 0x00FF) as u8);
        cpu.stkp -= 1;

        cpu.pc = cpu.addr_abs;
        0
    }
}

pub struct LDA;
impl ExecutableOperation for LDA {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);

        cpu.a = cpu.fetched;
        cpu.set_flag(Flags6502::Z, cpu.a == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.a & 0x80) != 0);
        1
    }
}

pub struct LDX;
impl ExecutableOperation for LDX {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);

        cpu.x = cpu.fetched;
        cpu.set_flag(Flags6502::Z, cpu.x == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.x & 0x80) != 0);
        1
    }
}

pub struct LDY;
impl ExecutableOperation for LDY {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);

        cpu.y = cpu.fetched;
        cpu.set_flag(Flags6502::Z, cpu.y == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.y & 0x80) != 0);
        1
    }
}

pub struct LSR;
impl ExecutableOperation for LSR {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);
        cpu.set_flag(Flags6502::C, (cpu.fetched & 0x0001) != 0);
        cpu.temp = (cpu.fetched >> 1) as u16;

        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(Flags6502::N, (cpu.temp & 0x0080) != 0);

        if matches!(
            Instruction::from_opcode(cpu.opcode).get_addrmode(),
            AddrMode::IMP
        ) {
            cpu.a = (cpu.temp & 0x00FF) as u8;
        } else {
            io.write(cpu.addr_abs, (cpu.temp & 0x00FF) as u8)
        }

        0
    }
}

pub struct NOP;
impl ExecutableOperation for NOP {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        // Not all nops are equal
        // TODO: Implement illegal opcodes

        let extra = match cpu.opcode {
            0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => 1,
            _ => 0,
        };

        extra
    }
}

pub struct ORA;
impl ExecutableOperation for ORA {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);
        cpu.a = cpu.a | cpu.fetched;

        cpu.set_flag(Flags6502::Z, cpu.a == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.a & 0x80) != 0);

        1
    }
}

pub struct PHA;
impl ExecutableOperation for PHA {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        io.write(0x0100 + (cpu.stkp as u16), cpu.a);
        cpu.stkp -= 1;
        0
    }
}

pub struct PHP;
impl ExecutableOperation for PHP {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        io.write(
            0x0100 + (cpu.stkp as u16),
            cpu.status | Flags6502::B.0 | Flags6502::U.0,
        );
        cpu.set_flag(Flags6502::B, false);
        cpu.set_flag(Flags6502::U, false);

        cpu.stkp -= 1;
        0
    }
}

pub struct PLA;
impl ExecutableOperation for PLA {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.stkp += 1;
        cpu.a = io.read(0x0100 + (cpu.stkp as u16));

        cpu.set_flag(Flags6502::Z, cpu.a == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.a & 0x80) != 0);

        0
    }
}

pub struct PLP;
impl ExecutableOperation for PLP {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.stkp += 1;
        cpu.status = io.read(0x0100 + (cpu.stkp as u16));

        cpu.set_flag(Flags6502::U, true);

        0
    }
}

pub struct ROL;
impl ExecutableOperation for ROL {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);
        cpu.temp = ((cpu.fetched << 1) as u16) | (cpu.get_flag(Flags6502::C) as u16);

        cpu.set_flag(Flags6502::C, (cpu.temp & 0xFF00) != 0x0000);
        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(Flags6502::N, (cpu.temp & 0x0080) != 0x0000);

        if matches!(
            Instruction::from_opcode(cpu.opcode).get_addrmode(),
            AddrMode::IMP
        ) {
            cpu.a = (cpu.temp & 0x00FF) as u8;
        } else {
            io.write(cpu.addr_abs, (cpu.temp & 0x00FF) as u8)
        }

        0
    }
}

pub struct ROR;
impl ExecutableOperation for ROR {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);
        cpu.temp = ((cpu.get_flag(Flags6502::C) << 7) as u16) | ((cpu.fetched >> 1) as u16);

        cpu.set_flag(Flags6502::C, (cpu.fetched & 0x01) != 0x00);
        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(Flags6502::N, (cpu.temp & 0x0080) != 0x0000);

        if matches!(
            Instruction::from_opcode(cpu.opcode).get_addrmode(),
            AddrMode::IMP
        ) {
            cpu.a = (cpu.temp & 0x00FF) as u8;
        } else {
            io.write(cpu.addr_abs, (cpu.temp & 0x00FF) as u8)
        }

        0
    }
}

pub struct RTI;
impl ExecutableOperation for RTI {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.stkp += 1;
        cpu.status = io.read(0x0100 + (cpu.stkp as u16));
        cpu.status &= !Flags6502::B.0;
        cpu.status &= !Flags6502::U.0;

        cpu.stkp += 1;
        cpu.pc = io.read(0x0100 + (cpu.stkp as u16)) as u16;
        cpu.stkp += 1;
        cpu.pc |= (io.read(0x0100 + (cpu.stkp as u16)) as u16) << 8;
        0
    }
}

pub struct RTS;
impl ExecutableOperation for RTS {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.stkp += 1;
        cpu.pc = io.read(0x0100 + (cpu.stkp as u16)) as u16;
        cpu.stkp += 1;
        cpu.pc |= (io.read(0x0100 + (cpu.stkp as u16)) as u16) << 8;

        cpu.pc += 1;
        0
    }
}

pub struct SEC;
impl ExecutableOperation for SEC {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.set_flag(Flags6502::C, true);
        0
    }
}

pub struct SED;
impl ExecutableOperation for SED {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.set_flag(Flags6502::D, true);
        0
    }
}

pub struct SEI;
impl ExecutableOperation for SEI {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.set_flag(Flags6502::I, true);
        0
    }
}

pub struct STA;
impl ExecutableOperation for STA {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        io.write(cpu.addr_abs, cpu.a);
        0
    }
}

pub struct STX;
impl ExecutableOperation for STX {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        io.write(cpu.addr_abs, cpu.x);
        0
    }
}

pub struct STY;
impl ExecutableOperation for STY {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        io.write(cpu.addr_abs, cpu.y);
        0
    }
}

pub struct TAX;
impl ExecutableOperation for TAX {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.x = cpu.a;
        cpu.set_flag(Flags6502::Z, cpu.x == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.x & 0x80) != 0);
        0
    }
}

pub struct TAY;
impl ExecutableOperation for TAY {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.y = cpu.a;
        cpu.set_flag(Flags6502::Z, cpu.y == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.y & 0x80) != 0);
        0
    }
}

pub struct TSX;
impl ExecutableOperation for TSX {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.x = cpu.stkp;
        cpu.set_flag(Flags6502::Z, cpu.x == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.x & 0x80) != 0);
        0
    }
}

pub struct TXA;
impl ExecutableOperation for TXA {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.a = cpu.x;
        cpu.set_flag(Flags6502::Z, cpu.a == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.a & 0x80) != 0);
        0
    }
}

pub struct TXS;
impl ExecutableOperation for TXS {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.stkp = cpu.x;
        0
    }
}

pub struct TYA;
impl ExecutableOperation for TYA {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.a = cpu.y;
        cpu.set_flag(Flags6502::Z, cpu.a == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.a & 0x80) != 0);
        0
    }
}

pub struct XXX;
impl ExecutableOperation for XXX {
    fn execute(&self, _cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        0
    }
}
