use crate::{bus_mod::bus::CpuRAM, iodevice::IODevice};

use super::cpu6502::Cpu6502;

pub struct Instruction {
    name: String,
    operator: Box<dyn ExecutableOperation>,
    addrmode: Box<dyn ExecutableOperation>,
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
        cycles: u8,
    ) -> Instruction {
        Instruction {
            name: name.to_string(),
            operator,
            addrmode,
            cycles,
        }
    }

    pub fn from_opcode(opcode: u8) -> Instruction {
        match opcode {
            0 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 5),
            1 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            2 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            3 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            4 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            5 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            6 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            7 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            8 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            9 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            10 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            11 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            12 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            13 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            14 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            15 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            16 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            17 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            18 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            19 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            20 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            21 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            22 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            23 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            24 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            25 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            26 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            27 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            28 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            29 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            30 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            31 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            32 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            33 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            34 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            35 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            36 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            37 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            38 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            39 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            40 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            41 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            42 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            43 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            44 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            45 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            46 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            47 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            48 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            49 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            50 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            51 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            52 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            53 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            54 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            55 => Instruction::new("OPNAME", Box::new(ADC), Box::new(IMP), 0),
            _ => Instruction::new("???", Box::new(ADC), Box::new(IMP), 0),
        }
    }

    pub fn execute_operator(&self, cpu: &mut Cpu6502, bus: &mut CpuRAM) -> u8 {
        self.operator.execute(cpu, bus)
    }

    pub fn execute_addmode(&self, cpu: &mut Cpu6502, bus: &mut CpuRAM) -> u8 {
        self.addrmode.execute(cpu, bus)
    }

    pub fn execute_operator_io(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        self.operator.execute_io(cpu, io)
    }

    pub fn execute_addmode_io(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        self.addrmode.execute_io(cpu, io)
    }
    pub fn get_cycles(&self) -> u8 {
        
        self.cycles
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

pub trait ExecutableOperation {
    fn execute(&self, cpu: &mut Cpu6502, bus: &mut CpuRAM) -> u8;
    fn execute_io(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8;
}

pub struct IMP;
impl ExecutableOperation for IMP {
    fn execute(&self, cpu: &mut Cpu6502, bus: &mut CpuRAM) -> u8 {
        println!("IMP executed");
        cpu.fetched = cpu.a;
        bus.write(0x0, 0x0);
        0
    }

    fn execute_io(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {

        println!("IMP IO");
        0
    }
}

pub struct ADC;
impl ExecutableOperation for ADC {
    fn execute(&self, cpu: &mut Cpu6502, bus: &mut CpuRAM) -> u8 {
        println!("ADC executed");
        0
    }

    fn execute_io(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
       println!("ADC IO");
       0
    }
}
