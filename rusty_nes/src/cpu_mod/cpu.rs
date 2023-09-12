use std::iter::Map;

use crate::bus_mod::bus::Bus;

use super::flags6502::Flags6502;


pub trait Cpu {
    fn reset(&mut self);
    fn irq(&mut self);
    fn nmi(&mut self);
    fn clock(&mut self);

    fn completed_instruction(&mut self) -> bool;
    fn connect_bus(&mut self, bus: &Bus);

    fn dissassemble(&self, start: u8, end: u8) -> Map<u8, String>;

    fn get_flag(&self, flag: Flags6502) -> u8;
    fn set_flag(&mut self, flag: Flags6502, value: bool);

    // Bus connection 
    // read and write functions
    fn read_from_bus(&self, addr: u16) -> u8;
    fn write_to_bus(&mut self, addr: u16, data: u8);

    fn fetch(&self);

    // Addressing Modes

    fn imp() -> u8; fn imm() -> u8;
    fn zp0() -> u8; fn zpx() -> u8;
    fn zpy() -> u8; fn rel() -> u8;
    fn abs() -> u8; fn abx() -> u8;
    fn aby() -> u8; fn ind() -> u8;
    fn izx() -> u8; fn izy() -> u8;

    // OpCodes

    /* 
    fn adc() -> u8; fn adc() -> u8; fn adc() -> u8; fn adc() -> u8; 
    fn adc() -> u8; fn adc() -> u8; fn adc() -> u8; fn adc() -> u8;
    fn adc() -> u8; fn adc() -> u8; fn adc() -> u8; fn adc() -> u8;
    fn adc() -> u8; fn adc() -> u8; fn adc() -> u8; fn adc() -> u8;
    fn adc() -> u8; fn adc() -> u8; fn adc() -> u8; fn adc() -> u8;
    fn adc() -> u8; fn adc() -> u8; fn adc() -> u8; fn adc() -> u8;
    fn adc() -> u8; fn adc() -> u8; fn adc() -> u8; fn adc() -> u8;
    fn adc() -> u8; fn adc() -> u8; fn adc() -> u8; fn adc() -> u8;
    fn adc() -> u8; fn adc() -> u8; fn adc() -> u8; fn adc() -> u8;
    fn adc() -> u8; fn adc() -> u8; fn adc() -> u8; fn adc() -> u8;
    fn adc() -> u8; fn adc() -> u8; fn adc() -> u8; fn adc() -> u8;
    fn adc() -> u8; fn adc() -> u8; fn adc() -> u8; fn adc() -> u8;
    fn adc() -> u8; fn adc() -> u8; fn adc() -> u8; fn adc() -> u8;
    fn adc() -> u8; fn adc() -> u8; fn adc() -> u8; fn adc() -> u8;
    */


}