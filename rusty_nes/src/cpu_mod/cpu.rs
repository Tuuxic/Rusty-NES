
use crate::iodevice::IODevice;

use super::flags6502::Flags6502;

pub trait Cpu {
    fn reset(&mut self, io: &mut IODevice);
    fn irq(&mut self, io: &mut IODevice);
    fn nmi(&mut self, io: &mut IODevice);
    fn clock(&mut self, io: &mut IODevice);

    // fn completed_instruction(&mut self) -> bool;

    // fn dissassemble(&self, start: u8, end: u8) -> Map<u8, String>;

    fn get_flag(&self, flag: Flags6502) -> u8;
    fn set_flag(&mut self, flag: Flags6502, value: bool);

    fn fetch(&mut self, io: &mut IODevice) -> u8;

}
