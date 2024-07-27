use std::fs;

use super::mapper::{Mapper, MapperId};

pub struct Cartridge {
    vprg_memory: Vec<u8>,
    #[allow(unused)]
    vchr_memory: Vec<u8>,

    #[allow(unused)]
    mapper_id: u8,
    #[allow(unused)]
    prg_banks: u8,
    #[allow(unused)]
    chr_banks: u8,
    mapper: Box<dyn Mapper>,
}

impl Cartridge {
    pub fn new() -> Cartridge {
        Cartridge {
            vprg_memory: vec![0; 16384],
            vchr_memory: vec![0; 8192],
            mapper_id: 0,
            prg_banks: 1,
            chr_banks: 1,
            mapper: MapperId(0).into(),
        }
    }

    #[allow(unused)]
    pub fn from_file(filename: &str) -> Cartridge {
        let content = fs::read(filename).expect("Failed reading the cartridge file");
        let mut offset: usize = 0;
        let header = INesHeader::new(&content);
        offset += 16; // Header Size
        offset += 512; // Trainer Size

        let mapper_id = ((header.mapper2 >> 4) << 4) | (header.mapper1 >> 4);

        // If filetype == 1 START
        let prg_banks = header.prg_rom_chunks;
        let mut vprg_memory = vec![0; prg_banks as usize * 16384];
        for i in 0..vprg_memory.len() {
            vprg_memory[i] = content[offset + i];
        }
        offset += vprg_memory.len(); // vPRG Memory Size

        let chr_banks = header.chr_rom_chunks;
        let mut vchr_memory = vec![0; chr_banks as usize * 8192];

        for i in 0..vchr_memory.len() {
            vchr_memory[i] = content[offset + i];
        }
        // offset += vchr_memory.len(); // vPRG Memory Size

        // If filetype == 1 END

        Cartridge {
            vprg_memory,
            vchr_memory,
            mapper_id,
            prg_banks,
            chr_banks,
            mapper: MapperId(mapper_id).into(),
        }
    }

    pub fn cpu_read(&self, addr: u16, data: &mut u8) -> bool {
        let mut mapped_addr: u32 = 0;
        if self.mapper.cpu_map_read(addr, &mut mapped_addr) {
            *data = self.vprg_memory[mapped_addr as usize];
            return true;
        }
        false
    }
    pub fn cpu_write(&mut self, addr: u16, data: u8) -> bool {
        let mut mapped_addr: u32 = 0;
        if self.mapper.cpu_map_read(addr, &mut mapped_addr) {
            self.vprg_memory[mapped_addr as usize] = data;
            return true;
        }
        false
    }

    #[allow(unused)]
    pub fn ppu_read(&self, addr: u16, data: &mut u8) -> bool {
        let mut mapped_addr: u32 = 0;
        if self.mapper.cpu_map_read(addr, &mut mapped_addr) {
            *data = self.vchr_memory[mapped_addr as usize];
            return true;
        }
        false
    }

    #[allow(unused)]
    pub fn ppu_write(&mut self, addr: u16, data: u8) -> bool {
        let mut mapped_addr: u32 = 0;
        if self.mapper.cpu_map_read(addr, &mut mapped_addr) {
            self.vchr_memory[mapped_addr as usize] = data;
            return true;
        }
        false
    }
}

#[allow(unused)]
struct INesHeader {
    name: [char; 4],
    prg_rom_chunks: u8,
    chr_rom_chunks: u8,
    mapper1: u8,
    mapper2: u8,
    prg_ram_size: u8,
    tv_system1: u8,
    ty_system2: u8,
}

impl INesHeader {
    pub fn new(data: &Vec<u8>) -> INesHeader {
        let header: INesHeader = unsafe { std::ptr::read(data.as_ptr() as *const _) };
        header
    }
}
