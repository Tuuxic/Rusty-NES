
use crate::{bus_mod::bus::CpuRAM, iodevice::IODevice};

use super::flags6502::Flags6502;

pub trait Cpu {
    fn reset(&mut self);
    fn irq(&mut self);
    fn nmi(&mut self);
    fn clock(&mut self, bus: &mut CpuRAM);
    fn clock_io(&mut self, io: &mut IODevice);

    // fn completed_instruction(&mut self) -> bool;

    // fn dissassemble(&self, start: u8, end: u8) -> Map<u8, String>;

    fn get_flag(&self, flag: Flags6502) -> u8;
    fn set_flag(&mut self, flag: Flags6502, value: bool);

    // Bus connection
    // read and write functions
    fn read_from_bus(&self, addr: u16) -> u8;
    fn write_to_bus(&mut self, addr: u16, data: u8);

    fn fetch(&self);

    // Addressing Modes

    fn imp(&mut self) -> u8;
    fn imm(&mut self) -> u8;
    fn zp0(&mut self) -> u8;
    fn zpx(&mut self) -> u8;
    fn zpy(&mut self) -> u8;
    fn rel(&mut self) -> u8;
    fn abs(&mut self) -> u8;
    fn abx(&mut self) -> u8;
    fn aby(&mut self) -> u8;
    fn ind(&mut self) -> u8;
    fn izx(&mut self) -> u8;
    fn izy(&mut self) -> u8;

    // OpCodes

    fn adc(&mut self) -> u8;
    fn and(&mut self) -> u8;
    fn asl(&mut self) -> u8;
    fn bcc(&mut self) -> u8;
    fn bcs(&mut self) -> u8;
    fn beq(&mut self) -> u8;
    fn bit(&mut self) -> u8;
    fn bmi(&mut self) -> u8;
    fn bne(&mut self) -> u8;
    fn bpl(&mut self) -> u8;
    fn brk(&mut self) -> u8;
    fn bvc(&mut self) -> u8;
    fn bvs(&mut self) -> u8;
    fn clc(&mut self) -> u8;
    fn cld(&mut self) -> u8;
    fn cli(&mut self) -> u8;
    fn clv(&mut self) -> u8;
    fn cmp(&mut self) -> u8;
    fn cpx(&mut self) -> u8;
    fn cpy(&mut self) -> u8;
    fn dec(&mut self) -> u8;
    fn dex(&mut self) -> u8;
    fn dey(&mut self) -> u8;
    fn eor(&mut self) -> u8;
    fn inc(&mut self) -> u8;
    fn inx(&mut self) -> u8;
    fn iny(&mut self) -> u8;
    fn jmp(&mut self) -> u8;
    fn jsr(&mut self) -> u8;
    fn lda(&mut self) -> u8;
    fn ldx(&mut self) -> u8;
    fn ldy(&mut self) -> u8;
    fn lsr(&mut self) -> u8;
    fn nop(&mut self) -> u8;
    fn ora(&mut self) -> u8;
    fn pha(&mut self) -> u8;
    fn php(&mut self) -> u8;
    fn pla(&mut self) -> u8;
    fn plp(&mut self) -> u8;
    fn rol(&mut self) -> u8;
    fn ror(&mut self) -> u8;
    fn rti(&mut self) -> u8;
    fn rts(&mut self) -> u8;
    fn sbc(&mut self) -> u8;
    fn sec(&mut self) -> u8;
    fn sed(&mut self) -> u8;
    fn sei(&mut self) -> u8;
    fn sta(&mut self) -> u8;
    fn stx(&mut self) -> u8;
    fn sty(&mut self) -> u8;
    fn tax(&mut self) -> u8;
    fn tay(&mut self) -> u8;
    fn tsx(&mut self) -> u8;
    fn txa(&mut self) -> u8;
    fn txs(&mut self) -> u8;
    fn tya(&mut self) -> u8;
}
