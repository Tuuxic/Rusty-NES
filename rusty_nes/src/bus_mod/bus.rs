use crate::{addr_utils::AddrUtils, bus_mod::cpu_ram::CpuRAM, cartridge_mod::cartridge::Cartridge};

use super::ppu_ram::PpuRAM;

pub struct Bus {
    ram: Box<CpuRAM>,
    // PPU
    ppu: Box<dyn PpuRAM>,
    // Cartrige
    cartridge: Box<Cartridge>,
}

impl Bus {
    pub fn new(ram: Box<CpuRAM>, ppu: Box<dyn PpuRAM>, cartridge: Box<Cartridge>) -> Bus {
        Bus {
            ram,
            ppu,
            cartridge,
        }
    }

    pub fn change_cartridge(&mut self, cartridge: Box<Cartridge>) {
        self.cartridge = cartridge;
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

    #[allow(unused)]
    fn ppu_read(&self, addr: u16, _readonly: bool) -> u8 {
        let mut data = 0;
        if self.cartridge.ppu_read(addr, &mut data) {
            //
        }
        return 0;
    }

    #[allow(unused)]
    fn ppu_write(&mut self, addr: u16, data: u8) {
        if self.cartridge.ppu_write(addr, data) {
            //
        }
    }
}
