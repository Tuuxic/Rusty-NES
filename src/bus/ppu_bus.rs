use std::{cell::RefCell, rc::Rc};

use crate::{cartridge::cartridge::Cartridge, ram::ppu_ram::PpuRAM};

pub struct PpuBus {
    pub ram: Box<PpuRAM>,
    cartridge: Rc<RefCell<Cartridge>>,
}

impl PpuBus {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>) -> PpuBus {
        PpuBus {
            ram: Box::new(PpuRAM::new()),
            cartridge,
        }
    }

    pub fn change_cartridge(&mut self, cartridge: Rc<RefCell<Cartridge>>) {
        self.cartridge = cartridge;
    }

    pub fn read(&self, addr: u16, readonly: bool) -> u8 {
        let mut data = 0;
        if self.cartridge.borrow().ppu_read(addr, &mut data) {
            //
        }
        return self.ram.read(addr, readonly);
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        if self.cartridge.borrow_mut().ppu_write(addr, data) {
            //
        }
        self.ram.write(addr, data);
    }
}
