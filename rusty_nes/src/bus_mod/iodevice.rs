use crate::{addr_utils::AddrUtils, bus_mod::cpu_ram::CpuRAM, cartridge_mod::cartridge::Cartridge};

use super::ppu_ram::PpuRAM;

pub struct IODevice<'a> {
    ram: &'a mut CpuRAM,
    // PPU
    ppu: &'a mut Box<dyn PpuRAM>,
    // Cartrige
    cartridge: &'a mut Cartridge,
}

impl IODevice<'_> {
    pub fn new<'a>(
        ram: &'a mut CpuRAM,
        ppu: &'a mut Box<dyn PpuRAM>,
        cartridge: &'a mut Cartridge,
    ) -> IODevice<'a> {
        IODevice {
            ram,
            ppu,
            cartridge,
        }
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8) {
        if self.cartridge.cpu_write(addr, data) {
            //
        } else if addr >= AddrUtils::CPU_RAM_ADDR_MIN && addr <= AddrUtils::CPU_RAM_ADDR_MAX {
            self.ram.write(addr & 0x07FF, data);
        } else if addr >= AddrUtils::PPU_RAM_ADDR_MIN && addr <= AddrUtils::PPU_RAM_ADDR_MAX {
            self.ppu.cpu_write(addr & 0x0007, data);
        }
    }

    pub fn cpu_read(&self, addr: u16) -> u8 {
        let mut data = 0;
        if self.cartridge.cpu_read(addr, &mut data) {
            //
        } else if addr >= AddrUtils::CPU_RAM_ADDR_MIN && addr <= AddrUtils::CPU_RAM_ADDR_MAX {
            return self.ram.read(addr & 0x07FF);
        } else if addr >= AddrUtils::PPU_RAM_ADDR_MIN && addr <= AddrUtils::PPU_RAM_ADDR_MAX {
            return self.ppu.cpu_read(addr & 0x0007, true);
        }

        0x00
    }
    fn ppu_read(&self, addr: u16, _readonly: bool) -> u8 {
        let mut data = 0;
        if self.cartridge.ppu_read(addr, &mut data) {
            //
        }
        return 0;
    }

    fn ppu_write(&mut self, addr: u16, data: u8) {
        if self.cartridge.ppu_write(addr, data) {
            //
        }
    }
}
