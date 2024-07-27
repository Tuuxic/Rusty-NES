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
    _ = engine::start();
}
