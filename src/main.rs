use std::env;

mod bus;
mod cartridge;
mod constants;
mod cpu;
mod debug;
mod emulator;
mod nes;
mod ppu;
mod ram;

#[allow(arithmetic_overflow)]
fn main() {
    if env::args().len() != 2 {
        panic!("ERROR: Provide Path to ROM as Argument!")
    }
    emulator::start();
}
