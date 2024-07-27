use crate::cpu::cpu::Cpu;

use super::{addrmode::AddrMode, opcodes::Opcode};

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
        let instr: Instruction = Opcode(opcode).into();
        instr
    }

    pub fn execute(&self, cpu: &mut Cpu) -> u8 {
        let cycle_addr: u8 = self.addrmode.execute(cpu);
        let cycle_op: u8 = self.operator.execute(cpu);
        cycle_addr + cycle_op
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
    fn execute(&self, cpu: &mut Cpu) -> u8;
}
