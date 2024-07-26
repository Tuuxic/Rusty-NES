use crate::addr_utils::AddrUtils;
use crate::bus_mod::bus::Bus;
use crate::cpu::cpu::Cpu;
use crate::cpu::cpu_flags::CpuFlags;

use super::addrmode::AddrMode;
use super::instruction::{Instruction, Operation};

// Operators
pub struct ADC;
impl Operation for ADC {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.fetch(bus);

        cpu.temp = cpu.a as u16 + cpu.fetched as u16 + cpu.get_flag(CpuFlags::C) as u16;

        cpu.set_flag(CpuFlags::C, cpu.temp > 255);
        cpu.set_flag(CpuFlags::Z, (cpu.temp & 0x00FF) == 0);
        cpu.set_flag(
            CpuFlags::V,
            (!(cpu.a as u16 ^ cpu.fetched as u16) & (cpu.a as u16 ^ cpu.temp as u16)) != 0,
        );
        cpu.set_flag(CpuFlags::N, (cpu.temp & 0x80) != 0);

        cpu.a = (cpu.temp & 0x00FF) as u8;

        1
    }
}

pub struct SBC;
impl Operation for SBC {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.fetch(bus);

        let value: u16 = (cpu.fetched as u16) ^ 0x00FF;
        cpu.temp = cpu.a as u16 + value + cpu.get_flag(CpuFlags::C) as u16;

        cpu.set_flag(CpuFlags::C, (cpu.temp & 0xFF00) != 0);
        cpu.set_flag(CpuFlags::Z, (cpu.temp & 0x00FF) == 0);
        cpu.set_flag(
            CpuFlags::V,
            ((cpu.temp ^ cpu.a as u16) & (cpu.temp ^ value) & 0x0080) != 0,
        );
        cpu.set_flag(CpuFlags::N, (cpu.temp & 0x0080) != 0);

        cpu.a = (cpu.temp & 0x00FF) as u8;

        1
    }
}

pub struct AND;
impl Operation for AND {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.fetch(bus);
        cpu.a = cpu.a & cpu.fetched;

        cpu.set_flag(CpuFlags::Z, cpu.a == 0);
        cpu.set_flag(CpuFlags::N, (cpu.a & 0x80) != 0);
        1
    }
}

pub struct ASL;
impl Operation for ASL {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.fetch(bus);
        cpu.temp = (cpu.fetched as u16) << 1;

        cpu.set_flag(CpuFlags::C, (cpu.temp & 0xFF00) > 0);
        cpu.set_flag(CpuFlags::Z, (cpu.temp & 0x00FF) == 0x00);
        cpu.set_flag(CpuFlags::N, (cpu.temp & 0x80) != 0);
        if matches!(
            Instruction::from_opcode(cpu.opcode).get_addrmode(),
            AddrMode::IMP
        ) {
            cpu.a = (cpu.temp & 0x00FF) as u8;
        } else {
            bus.cpu_write(cpu.addr_abs, (cpu.temp & 0x00FF) as u8);
        }
        0
    }
}

pub struct BCC;
impl Operation for BCC {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        if cpu.get_flag(CpuFlags::C) == 0 {
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
impl Operation for BCS {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        if cpu.get_flag(CpuFlags::C) == 1 {
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
impl Operation for BEQ {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        if cpu.get_flag(CpuFlags::Z) == 1 {
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
impl Operation for BIT {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.fetch(bus);

        cpu.temp = (cpu.a & cpu.fetched) as u16;

        cpu.set_flag(CpuFlags::Z, (cpu.temp & 0x00FF) == 0x00);
        cpu.set_flag(CpuFlags::N, (cpu.fetched & (1 << 7)) != 0);
        cpu.set_flag(CpuFlags::V, (cpu.fetched & (1 << 6)) != 0);

        0
    }
}

pub struct BMI;
impl Operation for BMI {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        if cpu.get_flag(CpuFlags::N) == 1 {
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
impl Operation for BNE {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        if cpu.get_flag(CpuFlags::Z) == 0 {
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
impl Operation for BPL {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        if cpu.get_flag(CpuFlags::N) == 0 {
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
impl Operation for BRK {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.pc += 1;

        cpu.set_flag(CpuFlags::I, true);
        bus.cpu_write(
            AddrUtils::CPU_STACK_BASE_ADDR + (cpu.stkp as u16),
            ((cpu.pc >> 8) & 0x00FF) as u8,
        );
        cpu.stkp -= 1;
        bus.cpu_write(
            AddrUtils::CPU_STACK_BASE_ADDR + (cpu.stkp as u16),
            (cpu.pc & 0x00FF) as u8,
        );
        cpu.stkp -= 1;

        cpu.set_flag(CpuFlags::B, true);
        bus.cpu_write(
            AddrUtils::CPU_STACK_BASE_ADDR + (cpu.stkp as u16),
            cpu.status,
        );
        cpu.stkp -= 1;
        cpu.set_flag(CpuFlags::B, false);

        cpu.pc = bus.cpu_read(0xFFFE) as u16 | ((bus.cpu_read(0xFFFF) as u16) << 8);
        0
    }
}

pub struct BVC;
impl Operation for BVC {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        if cpu.get_flag(CpuFlags::V) == 0 {
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
impl Operation for BVS {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        if cpu.get_flag(CpuFlags::V) == 1 {
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
impl Operation for CLC {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        cpu.set_flag(CpuFlags::C, false);
        0
    }
}

pub struct CLD;
impl Operation for CLD {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        cpu.set_flag(CpuFlags::D, false);
        0
    }
}

pub struct CLI;
impl Operation for CLI {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        cpu.set_flag(CpuFlags::I, false);
        0
    }
}

pub struct CLV;
impl Operation for CLV {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        cpu.set_flag(CpuFlags::V, false);
        0
    }
}

pub struct CMP;
impl Operation for CMP {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.fetch(bus);
        cpu.temp = cpu.a as u16 - cpu.fetched as u16;

        cpu.set_flag(CpuFlags::C, cpu.a >= cpu.fetched);
        cpu.set_flag(CpuFlags::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(CpuFlags::N, (cpu.temp & 0x0080) != 0);
        1
    }
}

pub struct CPX;
impl Operation for CPX {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.fetch(bus);
        cpu.temp = cpu.x as u16 - cpu.fetched as u16;

        cpu.set_flag(CpuFlags::C, cpu.x >= cpu.fetched);
        cpu.set_flag(CpuFlags::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(CpuFlags::N, (cpu.temp & 0x0080) != 0);
        0
    }
}

pub struct CPY;
impl Operation for CPY {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.fetch(bus);
        cpu.temp = cpu.y as u16 - cpu.fetched as u16;

        cpu.set_flag(CpuFlags::C, cpu.y >= cpu.fetched);
        cpu.set_flag(CpuFlags::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(CpuFlags::N, (cpu.temp & 0x0080) != 0);
        0
    }
}

pub struct DEC;
impl Operation for DEC {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.fetch(bus);
        cpu.temp = (cpu.fetched - 1) as u16;
        bus.cpu_write(cpu.addr_abs, (cpu.temp & 0x00FF) as u8);
        cpu.set_flag(CpuFlags::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(CpuFlags::N, (cpu.temp & 0x0080) != 0);
        0
    }
}

pub struct DEX;
impl Operation for DEX {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        cpu.x -= 1;
        cpu.set_flag(CpuFlags::Z, cpu.x == 0x00);
        cpu.set_flag(CpuFlags::N, (cpu.x & 0x80) != 0);
        0
    }
}

pub struct DEY;
impl Operation for DEY {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        cpu.y -= 1;
        cpu.set_flag(CpuFlags::Z, cpu.y == 0x00);
        cpu.set_flag(CpuFlags::N, (cpu.y & 0x80) != 0);
        0
    }
}

pub struct EOR;
impl Operation for EOR {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.fetch(bus);
        cpu.a = cpu.a ^ cpu.fetched;

        cpu.set_flag(CpuFlags::Z, cpu.a == 0x00);
        cpu.set_flag(CpuFlags::N, (cpu.a & 0x80) != 0);

        1
    }
}

pub struct INC;
impl Operation for INC {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.fetch(bus);

        cpu.temp = (cpu.fetched as u16) + 1;
        bus.cpu_write(cpu.addr_abs, (cpu.temp & 0x00FF) as u8);
        cpu.set_flag(CpuFlags::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(CpuFlags::N, (cpu.temp & 0x0080) != 0);

        0
    }
}

pub struct INX;
impl Operation for INX {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        cpu.x += 1;
        cpu.set_flag(CpuFlags::Z, cpu.x == 0x00);
        cpu.set_flag(CpuFlags::N, (cpu.x & 0x80) != 0);

        0
    }
}

pub struct INY;
impl Operation for INY {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        cpu.y += 1;
        cpu.set_flag(CpuFlags::Z, cpu.y == 0x00);
        cpu.set_flag(CpuFlags::N, (cpu.y & 0x80) != 0);

        0
    }
}

pub struct JMP;
impl Operation for JMP {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        cpu.pc = cpu.addr_abs;
        0
    }
}

pub struct JSR;
impl Operation for JSR {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.pc -= 1;

        bus.cpu_write(
            AddrUtils::CPU_STACK_BASE_ADDR + (cpu.stkp as u16),
            ((cpu.pc >> 8) & 0x00FF) as u8,
        );
        cpu.stkp -= 1;

        bus.cpu_write(
            AddrUtils::CPU_STACK_BASE_ADDR + (cpu.stkp as u16),
            (cpu.pc & 0x00FF) as u8,
        );
        cpu.stkp -= 1;

        cpu.pc = cpu.addr_abs;
        0
    }
}

pub struct LDA;
impl Operation for LDA {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.fetch(bus);

        cpu.a = cpu.fetched;
        cpu.set_flag(CpuFlags::Z, cpu.a == 0x00);
        cpu.set_flag(CpuFlags::N, (cpu.a & 0x80) != 0);
        1
    }
}

pub struct LDX;
impl Operation for LDX {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.fetch(bus);

        cpu.x = cpu.fetched;
        cpu.set_flag(CpuFlags::Z, cpu.x == 0x00);
        cpu.set_flag(CpuFlags::N, (cpu.x & 0x80) != 0);
        1
    }
}

pub struct LDY;
impl Operation for LDY {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.fetch(bus);

        cpu.y = cpu.fetched;
        cpu.set_flag(CpuFlags::Z, cpu.y == 0x00);
        cpu.set_flag(CpuFlags::N, (cpu.y & 0x80) != 0);
        1
    }
}

pub struct LSR;
impl Operation for LSR {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.fetch(bus);
        cpu.set_flag(CpuFlags::C, (cpu.fetched & 0x0001) != 0);
        cpu.temp = (cpu.fetched >> 1) as u16;

        cpu.set_flag(CpuFlags::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(CpuFlags::N, (cpu.temp & 0x0080) != 0);

        if matches!(
            Instruction::from_opcode(cpu.opcode).get_addrmode(),
            AddrMode::IMP
        ) {
            cpu.a = (cpu.temp & 0x00FF) as u8;
        } else {
            bus.cpu_write(cpu.addr_abs, (cpu.temp & 0x00FF) as u8)
        }

        0
    }
}

pub struct NOP;
impl Operation for NOP {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
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
impl Operation for ORA {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.fetch(bus);
        cpu.a = cpu.a | cpu.fetched;

        cpu.set_flag(CpuFlags::Z, cpu.a == 0x00);
        cpu.set_flag(CpuFlags::N, (cpu.a & 0x80) != 0);

        1
    }
}

pub struct PHA;
impl Operation for PHA {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        bus.cpu_write(AddrUtils::CPU_STACK_BASE_ADDR + (cpu.stkp as u16), cpu.a);
        cpu.stkp -= 1;
        0
    }
}

pub struct PHP;
impl Operation for PHP {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        bus.cpu_write(
            AddrUtils::CPU_STACK_BASE_ADDR + (cpu.stkp as u16),
            cpu.status | CpuFlags::B.0 | CpuFlags::U.0,
        );
        cpu.set_flag(CpuFlags::B, false);
        cpu.set_flag(CpuFlags::U, false);

        cpu.stkp -= 1;
        0
    }
}

pub struct PLA;
impl Operation for PLA {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.stkp += 1;
        cpu.a = bus.cpu_read(AddrUtils::CPU_STACK_BASE_ADDR + (cpu.stkp as u16));

        cpu.set_flag(CpuFlags::Z, cpu.a == 0x00);
        cpu.set_flag(CpuFlags::N, (cpu.a & 0x80) != 0);

        0
    }
}

pub struct PLP;
impl Operation for PLP {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.stkp += 1;
        cpu.status = bus.cpu_read(AddrUtils::CPU_STACK_BASE_ADDR + (cpu.stkp as u16));

        cpu.set_flag(CpuFlags::U, true);

        0
    }
}

pub struct ROL;
impl Operation for ROL {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.fetch(bus);
        cpu.temp = ((cpu.fetched << 1) as u16) | (cpu.get_flag(CpuFlags::C) as u16);

        cpu.set_flag(CpuFlags::C, (cpu.temp & 0xFF00) != 0x0000);
        cpu.set_flag(CpuFlags::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(CpuFlags::N, (cpu.temp & 0x0080) != 0x0000);

        if matches!(
            Instruction::from_opcode(cpu.opcode).get_addrmode(),
            AddrMode::IMP
        ) {
            cpu.a = (cpu.temp & 0x00FF) as u8;
        } else {
            bus.cpu_write(cpu.addr_abs, (cpu.temp & 0x00FF) as u8)
        }

        0
    }
}

pub struct ROR;
impl Operation for ROR {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.fetch(bus);
        cpu.temp = ((cpu.get_flag(CpuFlags::C) << 7) as u16) | ((cpu.fetched >> 1) as u16);

        cpu.set_flag(CpuFlags::C, (cpu.fetched & 0x01) != 0x00);
        cpu.set_flag(CpuFlags::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(CpuFlags::N, (cpu.temp & 0x0080) != 0x0000);

        if matches!(
            Instruction::from_opcode(cpu.opcode).get_addrmode(),
            AddrMode::IMP
        ) {
            cpu.a = (cpu.temp & 0x00FF) as u8;
        } else {
            bus.cpu_write(cpu.addr_abs, (cpu.temp & 0x00FF) as u8)
        }

        0
    }
}

pub struct RTI;
impl Operation for RTI {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.stkp += 1;
        cpu.status = bus.cpu_read(AddrUtils::CPU_STACK_BASE_ADDR + (cpu.stkp as u16));
        cpu.status &= !CpuFlags::B.0;
        cpu.status &= !CpuFlags::U.0;

        cpu.stkp += 1;
        cpu.pc = bus.cpu_read(AddrUtils::CPU_STACK_BASE_ADDR + (cpu.stkp as u16)) as u16;
        cpu.stkp += 1;
        cpu.pc |= (bus.cpu_read(AddrUtils::CPU_STACK_BASE_ADDR + (cpu.stkp as u16)) as u16) << 8;
        0
    }
}

pub struct RTS;
impl Operation for RTS {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        cpu.stkp += 1;
        cpu.pc = bus.cpu_read(AddrUtils::CPU_STACK_BASE_ADDR + (cpu.stkp as u16)) as u16;
        cpu.stkp += 1;
        cpu.pc |= (bus.cpu_read(AddrUtils::CPU_STACK_BASE_ADDR + (cpu.stkp as u16)) as u16) << 8;

        cpu.pc += 1;
        0
    }
}

pub struct SEC;
impl Operation for SEC {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        cpu.set_flag(CpuFlags::C, true);
        0
    }
}

pub struct SED;
impl Operation for SED {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        cpu.set_flag(CpuFlags::D, true);
        0
    }
}

pub struct SEI;
impl Operation for SEI {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        cpu.set_flag(CpuFlags::I, true);
        0
    }
}

pub struct STA;
impl Operation for STA {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        bus.cpu_write(cpu.addr_abs, cpu.a);
        0
    }
}

pub struct STX;
impl Operation for STX {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        bus.cpu_write(cpu.addr_abs, cpu.x);
        0
    }
}

pub struct STY;
impl Operation for STY {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        bus.cpu_write(cpu.addr_abs, cpu.y);
        0
    }
}

pub struct TAX;
impl Operation for TAX {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        cpu.x = cpu.a;
        cpu.set_flag(CpuFlags::Z, cpu.x == 0x00);
        cpu.set_flag(CpuFlags::N, (cpu.x & 0x80) != 0);
        0
    }
}

pub struct TAY;
impl Operation for TAY {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        cpu.y = cpu.a;
        cpu.set_flag(CpuFlags::Z, cpu.y == 0x00);
        cpu.set_flag(CpuFlags::N, (cpu.y & 0x80) != 0);
        0
    }
}

pub struct TSX;
impl Operation for TSX {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        cpu.x = cpu.stkp;
        cpu.set_flag(CpuFlags::Z, cpu.x == 0x00);
        cpu.set_flag(CpuFlags::N, (cpu.x & 0x80) != 0);
        0
    }
}

pub struct TXA;
impl Operation for TXA {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        cpu.a = cpu.x;
        cpu.set_flag(CpuFlags::Z, cpu.a == 0x00);
        cpu.set_flag(CpuFlags::N, (cpu.a & 0x80) != 0);
        0
    }
}

pub struct TXS;
impl Operation for TXS {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        cpu.stkp = cpu.x;
        0
    }
}

pub struct TYA;
impl Operation for TYA {
    fn execute(&self, cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        cpu.a = cpu.y;
        cpu.set_flag(CpuFlags::Z, cpu.a == 0x00);
        cpu.set_flag(CpuFlags::N, (cpu.a & 0x80) != 0);
        0
    }
}

pub struct XXX;
impl Operation for XXX {
    fn execute(&self, _cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
        0
    }
}
