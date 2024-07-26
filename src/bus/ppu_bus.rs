use crate::{cartridge::cartridge::Cartridge, ram::ppu_ram::PpuRAM};

pub struct PpuBus {
    pub ram: Box<PpuRAM>,
    cartridge: Box<Cartridge>,
}

impl PpuBus {
    pub fn new(cartridge: Box<Cartridge>) -> PpuBus {
        PpuBus {
            ram: Box::new(PpuRAM::new()),
            // ppu_ram: ppu,
            cartridge,
        }
    }

    pub fn read(&self, addr: u16, readonly: bool) -> u8 {
        let mut data = 0;
        if self.cartridge.ppu_read(addr, &mut data) {
            //
        }
        return self.ram.read(addr, readonly);
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        if self.cartridge.ppu_write(addr, data) {
            //
        }
        self.ram.write(addr, data);
    }
}
