use palette::Srgb;

pub struct Ppu {
    colors: [Srgb<u8>; 0x40],
    screen: [Srgb<u8>; 256 * 240],

    frame_complete: bool,
    scanline: i16,
    cycle: i16,
}

impl Ppu {
    pub fn new() -> Ppu {
        let ppu: Ppu = Ppu {
            colors: [Srgb::<u8>::new(0, 0, 0); 0x40],
            screen: [Srgb::<u8>::new(0, 0, 0); 256 * 240],
            frame_complete: false,
            scanline: 0,
            cycle: 0,
        };
        ppu
    }

    pub fn clock(&mut self) {
        println!("Cycle: {}, Scanline {}", self.cycle, self.scanline);
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
}
