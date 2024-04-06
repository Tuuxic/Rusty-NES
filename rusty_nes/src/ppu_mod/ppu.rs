use crate::addr_utils::AddrUtils;

pub trait Ppu {
    fn cpu_read(&self, addr: u16, readonly: bool) -> u8;
    fn cpu_write(&mut self, addr: u16, data: u8);

    fn ppu_read(&self, addr: u16, readonly: bool) -> u8;
    fn ppu_write(&mut self, addr: u16, data: u8);
}

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
