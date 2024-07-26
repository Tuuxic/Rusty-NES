pub struct AddrUtils;
impl AddrUtils {
    pub const RAM_SIZE: u32 = 2048;
    pub const CPU_RAM_ADDR_MAX: u16 = 0x1FFF;
    pub const CPU_RAM_ADDR_MIN: u16 = 0x0000;

    pub const CPU_STACK_BASE_ADDR: u16 = 0x0100;
    pub const CPU_START_ADDR: u16 = 0xFFFC;

    pub const PPU_RAM_ADDR_MAX: u16 = 0x3FFF;
    pub const PPU_RAM_ADDR_MIN: u16 = 0x2000;

    pub const PPU_CONTROL_ADDR: u16 = 0x0000;
    pub const PPU_MASK_ADDR: u16 = 0x0001;
    pub const PPU_STATUS_ADDR: u16 = 0x0002;
    pub const PPU_OAMADDRESS_ADDR: u16 = 0x0003;
    pub const PPU_OAMDATA_ADDR: u16 = 0x0004;
    pub const PPU_SCROLL_ADDR: u16 = 0x0005;
    pub const PPU_PPUADDRESS_ADDR: u16 = 0x0006;
    pub const PPU_PPUDATA_ADDR: u16 = 0x0007;
}
