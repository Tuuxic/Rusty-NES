

const RAM_SIZE : u32 = 64 * 1024;
const RAM_ADDR_MAX : u16 = 0xFFFF;
const RAM_ADDR_MIN : u16 = 0x0000;

pub struct Bus {
    ram: [u8; RAM_SIZE as usize],
    // read_func: fn(u16) -> u8,
    // write_func: fn(u16, u8)
}


impl Bus {
    pub fn new() -> Bus{
        let bus = Bus {
            ram: [0x00; RAM_SIZE as usize],
            // read_func: Bus::read, 
            // write_func: Bus::write
            
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
