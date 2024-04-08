pub trait Mapper {
    fn cpu_map_read(&self, addr: u16, mapped_addr: &mut u32) -> bool;
    fn cpu_map_write(&self, addr: u16, mapped_addr: &mut u32) -> bool;
    fn ppu_map_read(&self, addr: u16, mapped_addr: &mut u32) -> bool;
    fn ppu_map_write(&self, addr: u16, mapped_addr: &mut u32) -> bool;
}

pub struct MapperUtils;
impl MapperUtils {
    pub fn from_id(id: u8) -> Box<dyn Mapper> {
        let mapper: Box<dyn Mapper> = match id {
            0 => Box::new(Mapper000 {
                prg_banks: 1,
                char_banks: 1,
            }),
            _ => Box::new(Mapper000 {
                prg_banks: 1,
                char_banks: 1,
            }),
        };
        mapper
    }
}
struct Mapper000 {
    prg_banks: u8,
    char_banks: u8,
}

impl Mapper for Mapper000 {
    fn cpu_map_read(&self, addr: u16, mapped_addr: &mut u32) -> bool {
        if addr >= 0x8000 {
            *mapped_addr = (addr & (if self.prg_banks > 1 { 0x7FFF } else { 0x3FFF })) as u32;
            return true;
        }
        false
    }

    fn cpu_map_write(&self, addr: u16, mapped_addr: &mut u32) -> bool {
        if addr >= 0x8000 {
            *mapped_addr = (addr & (if self.prg_banks > 1 { 0x7FFF } else { 0x3FFF })) as u32;
            return true;
        }
        false
    }

    fn ppu_map_read(&self, addr: u16, mapped_addr: &mut u32) -> bool {
        if addr <= 0x1FFF {
            *mapped_addr = addr as u32;
            return true;
        }
        false
    }

    fn ppu_map_write(&self, addr: u16, mapped_addr: &mut u32) -> bool {
        if addr <= 0x1FFF {
            if self.char_banks == 0 {
                *mapped_addr = addr as u32;
                return true;
            }
        }
        false
    }
}
