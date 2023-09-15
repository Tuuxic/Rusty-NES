use crate::bus_mod::bus::CpuRAM;

const RAM_ADDR_MAX: u16 = 0xFFFF;
const RAM_ADDR_MIN: u16 = 0x0000;

pub struct IODevice<'a> {
    ram: &'a mut CpuRAM,
    // PPU
    // Cartrige
}

impl IODevice<'_> {
    pub fn new<'a>(ram: &'a mut CpuRAM) -> IODevice {
        IODevice { ram }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        if addr < RAM_ADDR_MIN || addr > RAM_ADDR_MAX {
            return;
        }

        self.ram.write(addr, data);
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        if addr < RAM_ADDR_MIN || addr > RAM_ADDR_MAX {
            return 0x00;
        }

        self.ram.read(addr)
    }
}
