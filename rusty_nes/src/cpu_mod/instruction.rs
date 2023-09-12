

pub struct Instruction {
    name: String,
    operator: fn() -> u8,
    addrmode: fn() -> u8,
    cycles: u8
}