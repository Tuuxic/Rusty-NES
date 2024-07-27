pub mod emulator {
    pub const GAME_ID: &str = "RustyNes";
    pub const AUTHOR_NAME: &str = "Nikolai Prjanikov";
}

pub mod cpu {

    pub const RAM_SIZE: u32 = 2048;
    pub const RAM_ADDR_MAX: u16 = 0x1FFF;
    pub const RAM_ADDR_MIN: u16 = 0x0000;

    pub const STACK_BASE_ADDR: u16 = 0x0100;
    pub const START_ADDR: u16 = 0xFFFC;
}

pub mod ppu {

    pub const RAM_ADDR_MAX: u16 = 0x3FFF;
    pub const RAM_ADDR_MIN: u16 = 0x2000;

    pub const CONTROL_ADDR: u16 = 0x0000;
    pub const MASK_ADDR: u16 = 0x0001;
    pub const STATUS_ADDR: u16 = 0x0002;
    pub const OAMADDRESS_ADDR: u16 = 0x0003;
    pub const OAMDATA_ADDR: u16 = 0x0004;
    pub const SCROLL_ADDR: u16 = 0x0005;
    pub const PPUADDRESS_ADDR: u16 = 0x0006;
    pub const PPUDATA_ADDR: u16 = 0x0007;
}
