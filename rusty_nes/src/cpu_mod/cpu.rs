use crate::bus_mod::bus::Bus;

use super::flags6502::Flags6502;

pub trait Cpu {
    fn reset(&mut self, io: &mut Bus);
    fn irq(&mut self, io: &mut Bus);
    fn nmi(&mut self, io: &mut Bus);
    fn clock(&mut self, io: &mut Bus);

    // fn completed_instruction(&mut self) -> bool;

    // fn dissassemble(&self, start: u8, end: u8) -> Map<u8, String>;

    fn get_flag(&self, flag: Flags6502) -> u8;
    fn set_flag(&mut self, flag: Flags6502, value: bool);

    fn fetch(&mut self, io: &mut Bus) -> u8;
}
