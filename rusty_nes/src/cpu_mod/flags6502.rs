

pub struct Flags6502(u8);

impl Flags6502 {
    const C: Flags6502 = Self(1 << 0);
    const Z: Flags6502 = Self(1 << 1);
    const I: Flags6502 = Self(1 << 2);
    const D: Flags6502 = Self(1 << 3);
    const B: Flags6502 = Self(1 << 4);
    const U: Flags6502 = Self(1 << 5);
    const V: Flags6502 = Self(1 << 6);
    const N: Flags6502 = Self(1 << 7);
}