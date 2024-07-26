pub struct CpuFlags(pub u8);

impl CpuFlags {
    pub const C: CpuFlags = Self(1 << 0);
    pub const Z: CpuFlags = Self(1 << 1);
    pub const I: CpuFlags = Self(1 << 2);
    pub const D: CpuFlags = Self(1 << 3);
    pub const B: CpuFlags = Self(1 << 4);
    pub const U: CpuFlags = Self(1 << 5);
    pub const V: CpuFlags = Self(1 << 6);
    pub const N: CpuFlags = Self(1 << 7);
}
