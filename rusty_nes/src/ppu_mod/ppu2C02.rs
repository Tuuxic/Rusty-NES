use super::ppu::{Ppu, PpuAddr};

pub struct Ppu2C02 {}

impl Ppu for Ppu2C02 {
    fn cpu_read(&self, addr: u16, _readonly: bool) -> u8 {
        let mem_region = PpuAddr::to_ppuaddr(addr);
        match mem_region {
            PpuAddr::Control => return 0,
            PpuAddr::Mask => return 0,
            PpuAddr::Status => return 0,
            PpuAddr::OAMAddr => return 0,
            PpuAddr::OAMData => return 0,
            PpuAddr::Scroll => return 0,
            PpuAddr::PPUAddr => return 0,
            PpuAddr::PPUData => return 0,
            PpuAddr::Invalid => return 0,
        }
    }

    fn cpu_write(&self, addr: u16, _data: u8) {
        let mem_region = PpuAddr::to_ppuaddr(addr);
        match mem_region {
            PpuAddr::Control => return,
            PpuAddr::Mask => return,
            PpuAddr::Status => return,
            PpuAddr::OAMAddr => return,
            PpuAddr::OAMData => return,
            PpuAddr::Scroll => return,
            PpuAddr::PPUAddr => return,
            PpuAddr::PPUData => return,
            PpuAddr::Invalid => return,
        }
    }

    fn ppu_read(&self, _addr: u16, _readonly: bool) -> u8 {
        return 0;
    }

    fn ppu_write(&self, _addr: u16, _data: u8) {}
}
