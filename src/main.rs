use std::env;

mod bus;
mod cartridge;
mod constants;
mod cpu;
mod disassembler;
mod engine;
mod nes;
mod ppu;
mod ram;

#[allow(arithmetic_overflow)]
fn main() {
    if env::args().len() != 2 {
        panic!("Provide path to ROM as argument")
    }
    _ = engine::start();
}
