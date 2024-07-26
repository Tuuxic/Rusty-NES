use crate::addr_utils::AddrUtils;

pub struct CpuRAM {
    pub ram: [u8; AddrUtils::RAM_SIZE as usize],
}

impl CpuRAM {
    pub fn new() -> CpuRAM {
        let cpu_ram = CpuRAM {
            ram: [0x00; AddrUtils::RAM_SIZE as usize],
        };

        cpu_ram
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        if addr < AddrUtils::CPU_RAM_ADDR_MIN || addr > AddrUtils::CPU_RAM_ADDR_MAX {
            print!("Warning: Writing outside of CPU RAM Bounds");
            return;
        }

        self.ram[addr as usize] = data;
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.read_mut(addr, false)
    }

    fn read_mut(&self, addr: u16, _read_only: bool) -> u8 {
        if addr < AddrUtils::CPU_RAM_ADDR_MIN || addr > AddrUtils::CPU_RAM_ADDR_MAX {
            print!("Warning: Reading outside of CPU RAM Bounds");
            return 0x00;
        }

        self.ram[addr as usize]
    }
}
