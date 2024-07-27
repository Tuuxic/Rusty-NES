macro_rules! mapper {
    ($mapper:ident) => {
        Box::new($mapper::new())
    };
}

pub struct MapperId(pub u8);
impl From<MapperId> for Box<dyn Mapper> {
    fn from(mapper_id: MapperId) -> Box<dyn Mapper> {
        let MapperId(id) = mapper_id;

        let mapper = match id {
            0 => mapper!(Mapper000),
            _ => mapper!(Mapper000),
        };

        mapper
    }
}

pub trait Mapper {
    fn cpu_read(&self, addr: u16, mapped_addr: &mut u32) -> bool;
    fn cpu_write(&mut self, addr: u16, mapped_addr: &mut u32) -> bool;

    fn ppu_read(&self, addr: u16, mapped_addr: &mut u32) -> bool;
    fn ppu_write(&mut self, addr: u16, mapped_addr: &mut u32) -> bool;
}

struct Mapper000 {
    prg_banks: u8,
    char_banks: u8,
}

impl Mapper000 {
    fn new() -> Mapper000 {
        Mapper000 {
            prg_banks: 1,
            char_banks: 1,
        }
    }
}

impl Mapper for Mapper000 {
    fn cpu_read(&self, addr: u16, mapped_addr: &mut u32) -> bool {
        if addr >= 0x8000 {
            *mapped_addr = (addr & (if self.prg_banks > 1 { 0x7FFF } else { 0x3FFF })) as u32;
            return true;
        }
        false
    }

    fn cpu_write(&mut self, addr: u16, mapped_addr: &mut u32) -> bool {
        if addr >= 0x8000 {
            *mapped_addr = (addr & (if self.prg_banks > 1 { 0x7FFF } else { 0x3FFF })) as u32;
            return true;
        }
        false
    }

    fn ppu_read(&self, addr: u16, mapped_addr: &mut u32) -> bool {
        if addr <= 0x1FFF {
            *mapped_addr = addr as u32;
            return true;
        }
        false
    }

    fn ppu_write(&mut self, addr: u16, mapped_addr: &mut u32) -> bool {
        if addr <= 0x1FFF && self.char_banks == 0 {
            // Treat as RAM
            *mapped_addr = addr as u32;
            return true;
        }
        false
    }
}
