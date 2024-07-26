use crate::{
    addr_utils::AddrUtils, cartridge::cartridge::Cartridge, ppu::ppu::Ppu, ram::cpu_ram::CpuRAM,
};

pub struct CpuBus {
    ram: Box<CpuRAM>,
    cartridge: Box<Cartridge>,
    pub ppu: Box<Ppu>,
}

impl CpuBus {
    pub fn new(cartridge: Box<Cartridge>) -> CpuBus {
        CpuBus {
            ram: Box::new(CpuRAM::new()),
            ppu: Box::new(Ppu::new(Box::new(Cartridge::new()))), // TODO: Insert Real Cartridge
            cartridge,
        }
    }

    pub fn change_cartridge(&mut self, cartridge: Box<Cartridge>) {
        self.cartridge = cartridge;
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        if self.cartridge.cpu_write(addr, data) {
            //
        } else if addr >= AddrUtils::CPU_RAM_ADDR_MIN && addr <= AddrUtils::CPU_RAM_ADDR_MAX {
            self.ram.write(addr & 0x07FF, data);
        } else if addr >= AddrUtils::PPU_RAM_ADDR_MIN && addr <= AddrUtils::PPU_RAM_ADDR_MAX {
            self.ppu.bus.write(addr & 0x0007, data);
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        let mut data = 0;
        if self.cartridge.cpu_read(addr, &mut data) {
            //
        } else if addr >= AddrUtils::CPU_RAM_ADDR_MIN && addr <= AddrUtils::CPU_RAM_ADDR_MAX {
            return self.ram.read(addr & 0x07FF);
        } else if addr >= AddrUtils::PPU_RAM_ADDR_MIN && addr <= AddrUtils::PPU_RAM_ADDR_MAX {
            return self.ppu.bus.read(addr & 0x0007, true);
        }

        0x00
    }
}
