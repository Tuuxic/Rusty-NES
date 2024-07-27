use crate::constants;

pub struct CpuRAM {
    pub ram: [u8; constants::cpu::RAM_SIZE as usize],
}

impl CpuRAM {
    pub fn new() -> CpuRAM {
        CpuRAM {
            ram: [0x00; constants::cpu::RAM_SIZE as usize],
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        if addr < constants::cpu::RAM_ADDR_MIN || addr > constants::cpu::RAM_ADDR_MAX {
            print!("Warning: Writing outside of CPU RAM Bounds");
            return;
        }

        self.ram[addr as usize] = data;
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.read_mut(addr, false)
    }

    fn read_mut(&self, addr: u16, _read_only: bool) -> u8 {
        if addr < constants::cpu::RAM_ADDR_MIN || addr > constants::cpu::RAM_ADDR_MAX {
            print!("Warning: Reading outside of CPU RAM Bounds");
            return 0x00;
        }

        self.ram[addr as usize]
    }
}
