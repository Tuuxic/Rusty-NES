use crate::{bus_mod::bus::Bus, cpu::cpu::Cpu};

use super::{addrmode::AddrMode, instr_mapping::InstructionMapping};

pub struct Instruction {
    name: String,
    operator: Box<dyn Operation>,
    addrmode: Box<dyn Operation>,
    addrtype: AddrMode,
    cycles: u8,
}

impl Instruction {
    pub fn new(
        name: &str,
        operator: Box<dyn Operation>,
        addrmode: Box<dyn Operation>,
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

    pub fn execute_operator(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        self.operator.execute(cpu, bus)
    }

    pub fn execute_addrmode(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8 {
        self.addrmode.execute(cpu, bus)
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
}

pub trait Operation {
    fn execute(&self, cpu: &mut Cpu, bus: &mut Bus) -> u8;
}
