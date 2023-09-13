pub struct Flags6502(pub u8);

impl Flags6502 {
    pub const C: Flags6502 = Self(1 << 0);
    pub const Z: Flags6502 = Self(1 << 1);
    pub const I: Flags6502 = Self(1 << 2);
    pub const D: Flags6502 = Self(1 << 3);
    pub const B: Flags6502 = Self(1 << 4);
    pub const U: Flags6502 = Self(1 << 5);
    pub const V: Flags6502 = Self(1 << 6);
    pub const N: Flags6502 = Self(1 << 7);
}
