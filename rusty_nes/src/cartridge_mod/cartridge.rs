use std::fs;

pub struct Cartridge {
    vprg_memory: Vec<u8>,
    vchr_memory: Vec<u8>,

    mapper_id: u8,
    prg_banks: u8,
    chr_banks: u8,
}

impl Cartridge {
    pub fn new() -> Cartridge {
        Cartridge {
            vprg_memory: vec![],
            vchr_memory: vec![],
            mapper_id: 0,
            prg_banks: 0,
            chr_banks: 0,
        }
    }
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
        }
    }
}

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
