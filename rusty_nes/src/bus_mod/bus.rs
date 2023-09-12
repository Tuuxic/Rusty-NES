
use crate::cpu_mod::cpu::Cpu;
use crate::cpu_mod::cpu6502::Cpu6502;

const RAM_SIZE : u32 = 64 * 1024;
const RAM_ADDR_MAX : u16 = 0xFFFF;
const RAM_ADDR_MIN : u16 = 0x0000;

pub struct Bus {
    cpu: Cpu6502,
    ram: [u8; RAM_SIZE as usize],
}


impl Bus {
    pub fn new() -> Bus{
        let cpu = Cpu6502::new(); 
        let bus = Bus {
            cpu, 
            ram: [0x00; RAM_SIZE as usize]
        };

        // cpu.connect_bus(&bus);
        bus
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        if addr < RAM_ADDR_MIN || addr > RAM_ADDR_MAX {
            return
        }

        self.ram[addr as usize] = data;
    }

    pub fn read_mut(& self, addr: u16, read_only: bool) -> u8 {
        if addr < RAM_ADDR_MIN || addr > RAM_ADDR_MAX {
            return 0x00;
        }

        self.ram[addr as usize]
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.read_mut(addr, false)
    }
}
