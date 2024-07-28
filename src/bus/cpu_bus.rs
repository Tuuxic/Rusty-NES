use std::{cell::RefCell, rc::Rc};

use crate::{cartridge::cartridge::Cartridge, constants, ppu::ppu::Ppu, ram::cpu_ram::CpuRAM};

pub struct CpuBus {
    ram: Box<CpuRAM>,
    cartridge: Rc<RefCell<Cartridge>>,
    pub ppu: Box<Ppu>,
}

impl CpuBus {
    pub fn new() -> CpuBus {
        let cartridge = Rc::new(RefCell::new(Cartridge::new()));
        CpuBus {
            ram: Box::new(CpuRAM::new()),
            ppu: Box::new(Ppu::new(Rc::clone(&cartridge))),
            cartridge, // cartridge,
        }
    }

    pub fn change_cartridge(&mut self, cartridge: Rc<RefCell<Cartridge>>) {
        self.cartridge = Rc::clone(&cartridge);
        self.ppu.bus.change_cartridge(cartridge)
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        if self.cartridge.borrow_mut().cpu_write(addr, data) {
            //
        } else if addr >= constants::cpu::RAM_ADDR_MIN && addr <= constants::cpu::RAM_ADDR_MAX {
            self.ram.write(addr & 0x07FF, data);
        } else if addr >= constants::ppu::RAM_ADDR_MIN && addr <= constants::ppu::RAM_ADDR_MAX {
            // Might cause runtime error due to double mut borrow
            // Change to PPU Write instead of PPU Bus => PPU Bus only accessible by PPU
            self.ppu.bus.write(addr & 0x0007, data);
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        let mut data = 0;
        if self.cartridge.borrow().cpu_read(addr, &mut data) {
            return data;
        } else if addr >= constants::cpu::RAM_ADDR_MIN && addr <= constants::cpu::RAM_ADDR_MAX {
            return self.ram.read(addr & 0x07FF);
        } else if addr >= constants::ppu::RAM_ADDR_MIN && addr <= constants::ppu::RAM_ADDR_MAX {
            // Might cause runtime error due to double mut borrow
            // Change to PPU Read instead of PPU Bus => PPU Bus only accessible by PPU
            return self.ppu.bus.read(addr & 0x0007, true);
        }

        0x00
    }
}
