use bincode::deserialize;
use serde::Deserialize;
use std::fs;

use super::mapper::{Mapper, MapperId};

pub struct Cartridge {
    vprg_memory: Vec<u8>,
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

    pub fn from_file(filename: &str) -> Cartridge {
        let content = fs::read(filename).expect("Failed reading the cartridge file");
        let mut header_bytes: [u8; 16] = [0x00; 16];

        for i in 0..16 {
            header_bytes[i] = content[i]
        }

        let header = INesHeader::new(header_bytes);
        let mapper_id = header.mapper_id();
        let prg_banks = header.prg_rom_chunks;
        let chr_banks = header.chr_rom_chunks;

        let mut offset: usize = 0;
        offset += 16; // Header Size

        if header.has_trainer() {
            offset += 512; // Trainer Size
        }

        let filetype = 1;
        let mut vprg_memory = vec![];
        let mut vchr_memory = vec![];

        if filetype == 1 {
            vprg_memory = vec![0; prg_banks as usize * 16384];
            for i in 0..vprg_memory.len() {
                vprg_memory[i] = content[offset + i];
            }
            offset += vprg_memory.len(); // vPRG Memory Size

            vchr_memory = vec![0; chr_banks as usize * 8192];

            for i in 0..vchr_memory.len() {
                vchr_memory[i] = content[offset + i];
            }
            offset += vchr_memory.len(); // vPRG Memory Size
        }

        assert_eq!(offset, content.len());

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
        if self.mapper.cpu_map_write(addr, &mut mapped_addr) {
            self.vprg_memory[mapped_addr as usize] = data;
            return true;
        }
        false
    }

    pub fn ppu_read(&self, addr: u16, data: &mut u8) -> bool {
        let mut mapped_addr: u32 = 0;
        if self.mapper.ppu_map_read(addr, &mut mapped_addr) {
            *data = self.vchr_memory[mapped_addr as usize];
            return true;
        }
        false
    }

    pub fn ppu_write(&mut self, addr: u16, data: u8) -> bool {
        let mut mapped_addr: u32 = 0;
        if self.mapper.ppu_map_write(addr, &mut mapped_addr) {
            self.vchr_memory[mapped_addr as usize] = data;
            return true;
        }
        false
    }
}

#[derive(Deserialize, Debug)]
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
    _unused: [u8; 5],
}

impl INesHeader {
    pub fn new(data: [u8; 16]) -> INesHeader {
        deserialize(&data).unwrap()
    }

    pub fn has_trainer(&self) -> bool {
        return (self.mapper1 & 0x08) != 0;
    }

    pub fn mapper_id(&self) -> u8 {
        ((self.mapper2 >> 4) << 4) | (self.mapper1 >> 4)
    }
}
