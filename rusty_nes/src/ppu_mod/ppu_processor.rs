use palette::Srgb;

use crate::bus_mod::{bus::Bus, ppu2c02_ram::PpuStatusFlag};

pub struct Ppu {
    #[allow(unused)]
    colors: Vec<Srgb<u8>>,
    screen: Vec<Srgb<u8>>,

    frame_complete: bool,
    scanline: i16,
    cycle: i16,
}

impl Ppu {
    pub fn new() -> Ppu {
        let ppu: Ppu = Ppu {
            colors: vec![Srgb::<u8>::new(0, 0, 0); 0x40],
            screen: vec![Srgb::<u8>::new(0, 0, 0); 256 * 240],
            frame_complete: false,
            scanline: 0,
            cycle: 0,
        };
        ppu
    }

    pub fn clock(&mut self, bus: &mut Bus) {
        if self.scanline == -1 && self.cycle == 1 {
            bus.ppu_ram
                .set_status_flag(PpuStatusFlag::VERTICAL_BLANK, false);
        }
        if self.scanline == 241 && self.cycle == 1 {
            bus.ppu_ram
                .set_status_flag(PpuStatusFlag::VERTICAL_BLANK, true);
            // if (bus.ppu.control.enable_nmi) {
            //     nmi = true;
            // }
        }
        self.screen[((self.cycle) + self.scanline * 240) as usize] = Srgb::<u8>::new(0, 255, 0);
        self.cycle += 1;
        if self.cycle >= 341 {
            // Scan line length
            self.cycle = 0;
            self.scanline += 1;
        }
        if self.scanline >= 261 {
            self.scanline = -1;
            self.frame_complete = true;
        }
    }

    // TODO: GetPatternTable
}
