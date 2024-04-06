use super::ppu::{PpuAddr, PpuRAM};

pub struct Ppu2C02 {
    name_table: [[u8; 1024]; 2],
    palette_table: [u8; 32],
}

impl Ppu2C02 {
    pub fn new() -> Ppu2C02 {
        let ppu: Ppu2C02 = Ppu2C02 {
            name_table: [[0; 1024]; 2],
            palette_table: [0; 32],
        };
        ppu
    }
}

impl PpuRAM for Ppu2C02 {
    fn cpu_read(&self, addr: u16, _readonly: bool) -> u8 {
        let mem_region = PpuAddr::to_ppuaddr(addr);
        match mem_region {
            PpuAddr::Control => return self.name_table[0][0], // Placeholder to avoid warning; Delete later
            PpuAddr::Mask => return self.palette_table[0], // Placeholder to avoid warning; Delete later
            PpuAddr::Status => return 0,
            PpuAddr::OAMAddr => return 0,
            PpuAddr::OAMData => return 0,
            PpuAddr::Scroll => return 0,
            PpuAddr::PPUAddr => return 0,
            PpuAddr::PPUData => return 0,
            PpuAddr::Invalid => return 0,
        }
    }

    fn cpu_write(&mut self, addr: u16, _data: u8) {
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

    fn ppu_write(&mut self, _addr: u16, _data: u8) {}
}
