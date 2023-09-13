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
/* 
    pub fn execute_operator(&self, cpu: &mut Cpu6502, bus: &mut CpuRAM) -> u8 {
        self.operator.execute(cpu, bus)
    }

    pub fn execute_addmode(&self, cpu: &mut Cpu6502, bus: &mut CpuRAM) -> u8 {
        self.addrmode.execute(cpu, bus)
    }
*/
    pub fn execute_operator(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        self.operator.execute(cpu, io)
    }

    pub fn execute_addmode(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        self.addrmode.execute(cpu, io)
    }
    pub fn get_cycles(&self) -> u8 {
        
        self.cycles
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

pub trait ExecutableOperation {
    // fn execute(&self, cpu: &mut Cpu6502, bus: &mut CpuRAM) -> u8;
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8;
}

// Addressing Modes
pub struct IMP;
impl ExecutableOperation for IMP {
    /* 
    fn execute(&self, cpu: &mut Cpu6502, bus: &mut CpuRAM) -> u8 {
        println!("IMP executed");
        cpu.fetched = cpu.a;
        bus.write(0x0, 0x0);
        0
    }
    */
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetched = cpu.a;
        io.write(0x0, 0x0);
        0
    }
}


pub struct IMM;
impl ExecutableOperation for IMM {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
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
       0
    }
}

