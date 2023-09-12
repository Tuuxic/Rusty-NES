// use crate::bus_mod::bus::Bus;
use super::instruction::Instruction;


pub struct Cpu6502 {
    a: u8,
    x: u8,
    y: u8,
    stkp: u8,
    pc: u16,
    status: u8,

    //bus: Bus,
    instruction_lookup: Vec<Instruction>,

    fetched: u8
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

            //bus: Bus::new(),
            instruction_lookup: vec![],

            fetched: 0x00
        }
    }
}