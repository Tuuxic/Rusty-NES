use bitflags::bitflags;

use crate::addr_utils::AddrUtils;

pub enum PpuAddr {
    Control,
    Mask,
    Status,
    OAMAddr,
    OAMData,
    Scroll,
    PPUAddr,
    PPUData,
    Invalid,
}

impl PpuAddr {
    pub fn to_ppuaddr(addr: u16) -> PpuAddr {
        match addr {
            AddrUtils::PPU_CONTROL_ADDR => PpuAddr::Control,
            AddrUtils::PPU_MASK_ADDR => PpuAddr::Mask,
            AddrUtils::PPU_STATUS_ADDR => PpuAddr::Status,
            AddrUtils::PPU_OAMADDRESS_ADDR => PpuAddr::OAMAddr,
            AddrUtils::PPU_OAMDATA_ADDR => PpuAddr::OAMData,
            AddrUtils::PPU_SCROLL_ADDR => PpuAddr::Scroll,
            AddrUtils::PPU_PPUADDRESS_ADDR => PpuAddr::PPUAddr,
            AddrUtils::PPU_PPUDATA_ADDR => PpuAddr::PPUData,
            _ => PpuAddr::Invalid,
        }
    }
}

#[allow(unused)]
pub struct PpuRAM {
    status: u8,
    mask: u8,
    control: u8,
    address_latch: u8,
    data_buffer: u8,
    ppu_address: u16,
    name_table: [[u8; 1024]; 2],
    palette_table: [u8; 32],
    pattern_table: [[u8; 4096]; 2],
}

impl PpuRAM {
    pub fn new() -> PpuRAM {
        let ppu: PpuRAM = PpuRAM {
            status: 0,
            mask: 0,
            control: 0,
            address_latch: 0x00,
            data_buffer: 0x00,
            ppu_address: 0x0000,
            name_table: [[0; 1024]; 2],
            palette_table: [0; 32],
            pattern_table: [[0; 4096]; 2],
        };
        ppu
    }
}

impl PpuRAM {
    pub fn read(&self, addr: u16, _readonly: bool) -> u8 {
        let mut index: usize = (addr as usize) & 0x3FFF;
        if addr <= 0x1FFF {
            // Pattern Memory
            let index_u: usize = ((addr as usize) & 0x1000) >> 12;
            let index_l: usize = (addr as usize) & 0x0FFF;
            return self.pattern_table[index_u][index_l];
        } else if addr >= 0x2000 && addr <= 0x3EFF { // Nametable Memory
        } else if addr >= 0x3F00 && addr <= 0x3FFF {
            // Palette Memory
            index &= 0x001F;

            // Mirroring
            // TODO: Optimize this code
            index = {
                match index {
                    0x0010 => 0x0000,
                    0x0014 => 0x0004,
                    0x0018 => 0x0008,
                    0x001C => 0x000C,
                    _ => index,
                }
            };
            return self.palette_table[index];
        }
        return 0;
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        let mut index: usize = (addr as usize) & 0x3FFF;
        if addr <= 0x1FFF {
            // Pattern Memory
            // Usually a ROM but nontheless allow write access to it
            let index_u: usize = ((addr as usize) & 0x1000) >> 12;
            let index_l: usize = (addr as usize) & 0x0FFF;
            self.pattern_table[index_u][index_l] = data;
        } else if addr >= 0x2000 && addr <= 0x3EFF { // Nametable Memory
        } else if addr >= 0x3F00 && addr <= 0x3FFF {
            // Palette Memory
            index &= 0x001F;

            // Mirroring
            // TODO: Optimize this code
            index = {
                match index {
                    0x0010 => 0x0000,
                    0x0014 => 0x0004,
                    0x0018 => 0x0008,
                    0x001C => 0x000C,
                    _ => index,
                }
            };
            self.palette_table[index] = data;
        }
    }

    pub fn set_status_flag(&mut self, flag: PpuStatusFlag, value: bool) {
        if value {
            self.status |= flag.bits();
        } else {
            self.status &= !flag.bits();
        }
    }
}

bitflags! {

    pub struct PpuStatusFlag: u8 {
        const SPRITE_OVERFLOW = (1 << 5);
        const SPRITE_ZERO_HIT = (1 << 6);
        const VERTICAL_BLANK = (1 << 7);
    }

}

bitflags! {

    pub struct PpuMaskFlag: u8 {
        const GRAYSCALE = (1 << 0);
        const RENDER_BACKGROUND_LEFT = (1 << 1);
        const RENDER_SPRITES_LEFT = (1 << 2);
        const RENDER_BACKGROUND = (1 << 3);
        const RENDER_SPRITES = (1 << 4);
        const ENHANCE_RED = (1 << 5);
        const ENHANCE_GREEN = (1 << 6);
        const ENHANCE_BLUE = (1 << 7);
    }

}

bitflags! {

    pub struct PpuControlFlag: u8 {
        const NAMETABLE_X = (1 << 0);
        const NAMETABLE_Y = (1 << 1);
        const INCREMENT_MODE = (1 << 2);
        const PATTERN_SPRITE = (1 << 3);
        const PATTERN_BACKGROUND = (1 << 4);
        const SPRITE_SIZE = (1 << 5);
        const SLAVE_MODE = (1 << 6);
        const ENABLE_NMI = (1 << 7);
    }

}
