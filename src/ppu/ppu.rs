use std::{cell::RefCell, rc::Rc};

use palette::Srgb;

use crate::{bus::ppu_bus::PpuBus, cartridge::cartridge::Cartridge, ram::ppu_ram::PpuStatusFlag};

pub struct Ppu {
    #[allow(unused)]
    colors: Vec<Srgb<u8>>,
    screen: Vec<Srgb<u8>>,

    frame_complete: bool,
    scanline: i32,
    cycle: i32,

    pub bus: Box<PpuBus>,
}

impl Ppu {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>) -> Ppu {
        let ppu: Ppu = Ppu {
            colors: vec![Srgb::<u8>::new(0, 0, 0); 0x40],
            screen: vec![Srgb::<u8>::new(0, 0, 0); 256 * 240],
            frame_complete: false,
            scanline: 0,
            cycle: 0,
            bus: Box::new(PpuBus::new(cartridge)),
        };
        ppu
    }

    pub fn clock(&mut self) {
        if self.scanline == -1 && self.cycle == 1 {
            self.bus
                .ram
                .set_status_flag(PpuStatusFlag::VERTICAL_BLANK, false);
        }
        if self.scanline == 241 && self.cycle == 1 {
            self.bus
                .ram
                .set_status_flag(PpuStatusFlag::VERTICAL_BLANK, true);
            // if (bus.ppu.control.enable_nmi) {
            //     nmi = true;
            // }
        }

        // ! TODO: FIX BUG WITH SCREEN VEC OVERFLOW!
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
