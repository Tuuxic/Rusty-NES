use crate::{addr_utils::AddrUtils, bus_mod::cpu_ram::CpuRAM, cartridge_mod::cartridge::Cartridge};

use super::ppu_ram::PpuRAM;

pub struct Bus {
    cpu_ram: Box<CpuRAM>,
    // PPU
    pub ppu_ram: Box<dyn PpuRAM>,
    // Cartrige
    cartridge: Box<Cartridge>,
}

impl Bus {
    pub fn new(ram: Box<CpuRAM>, ppu: Box<dyn PpuRAM>, cartridge: Box<Cartridge>) -> Bus {
        Bus {
            cpu_ram: ram,
            ppu_ram: ppu,
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
            self.cpu_ram.write(addr & 0x07FF, data);
        } else if addr >= AddrUtils::PPU_RAM_ADDR_MIN && addr <= AddrUtils::PPU_RAM_ADDR_MAX {
            self.ppu_ram.cpu_write(addr & 0x0007, data);
        }
    }

    pub fn cpu_read(&mut self, addr: u16) -> u8 {
        let mut data = 0;
        if self.cartridge.cpu_read(addr, &mut data) {
            //
        } else if addr >= AddrUtils::CPU_RAM_ADDR_MIN && addr <= AddrUtils::CPU_RAM_ADDR_MAX {
            return self.cpu_ram.read(addr & 0x07FF);
        } else if addr >= AddrUtils::PPU_RAM_ADDR_MIN && addr <= AddrUtils::PPU_RAM_ADDR_MAX {
            return self.ppu_ram.cpu_read(addr & 0x0007, true);
        }

        0x00
    }

    #[allow(unused)]
    fn ppu_read(&self, addr: u16, readonly: bool) -> u8 {
        let mut data = 0;
        if self.cartridge.ppu_read(addr, &mut data) {
            //
        }
        return self.ppu_ram.ppu_read(addr, readonly);
    }

    #[allow(unused)]
    fn ppu_write(&mut self, addr: u16, data: u8) {
        if self.cartridge.ppu_write(addr, data) {
            //
        }
        self.ppu_ram.ppu_write(addr, data);
    }
}
