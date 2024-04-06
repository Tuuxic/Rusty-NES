use crate::{addr_utils::AddrUtils, bus_mod::bus::CpuRAM};

pub struct IODevice<'a> {
    ram: &'a mut CpuRAM,
    // PPU
    // ppu: &'a mut dyn Ppu
    // Cartrige
}

impl IODevice<'_> {
    pub fn new<'a>(ram: &'a mut CpuRAM) -> IODevice {
        IODevice { ram }
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8) {
        if addr >= AddrUtils::CPU_RAM_ADDR_MIN && addr <= AddrUtils::CPU_RAM_ADDR_MAX {
            self.ram.write(addr & 0x07FF, data);
        } else if addr >= AddrUtils::PPU_RAM_ADDR_MIN && addr <= AddrUtils::PPU_RAM_ADDR_MAX {
            // self.ppu.cpu_write(addr & 0x0007, data);
        }
    }

    pub fn cpu_read(&mut self, addr: u16) -> u8 {
        if addr >= AddrUtils::CPU_RAM_ADDR_MIN && addr <= AddrUtils::CPU_RAM_ADDR_MAX {
            return self.ram.read(addr & 0x07FF);
        } else if addr >= AddrUtils::PPU_RAM_ADDR_MIN && addr <= AddrUtils::PPU_RAM_ADDR_MAX {
            // return self.ppu.cpu_read(addr & 0x0007, true);
        }

        0x00
    }
}
