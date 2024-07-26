use std::collections::HashMap;

use crate::{
    bus_mod::bus::Bus,
    cpu::operations::{addrmode::AddrMode, instruction::Instruction},
};

pub struct Disassembler;
impl Disassembler {
    pub fn dissassemble(
        start_addr: u16,
        end_addr: u16,
        io: &mut Bus,
    ) -> (HashMap<u16, u16>, Vec<String>) {
        let mut lines_map: HashMap<u16, u16> = HashMap::new();
        let mut instructions: Vec<String> = vec![];
        let mut instr_index: u16 = 0;
        let mut addr: u32 = start_addr as u32;
        let mut value: u8;
        let mut lo: u8;
        let mut hi: u8;
        let mut line_addr: u16;

        while addr <= (end_addr as u32) {
            line_addr = addr as u16;
            let mut instruction_str = ["0x", &Disassembler::hex(addr, 4), ": "].join("");
            let opcode: u8 = io.cpu_read(addr as u16);
            addr += 1;
            let instr = Instruction::from_opcode(opcode);
            instruction_str.push_str(&instr.get_name());
            instruction_str.push_str(" ");

            match instr.get_addrmode() {
                AddrMode::IMP => {
                    instruction_str.push_str(" {IMP}");
                }
                AddrMode::IMM => {
                    value = io.cpu_read(addr as u16);
                    addr += 1;
                    instruction_str
                        .push_str(&["#0x", &Disassembler::hex(value as u32, 2), " {IMM}"].join(""));
                }
                AddrMode::ZP0 => {
                    lo = io.cpu_read(addr as u16);
                    // hi = 0x00;
                    addr += 1;
                    instruction_str
                        .push_str(&["0x", &Disassembler::hex(lo as u32, 2), " {ZP0}"].join(""));
                }

                AddrMode::ZPX => {
                    lo = io.cpu_read(addr as u16);
                    // hi = 0x00;
                    addr += 1;
                    instruction_str
                        .push_str(&["0x", &Disassembler::hex(lo as u32, 2), ", X {ZPX}"].join(""));
                }
                AddrMode::ZPY => {
                    lo = io.cpu_read(addr as u16);
                    // hi = 0x00;
                    addr += 1;
                    instruction_str
                        .push_str(&["0x", &Disassembler::hex(lo as u32, 2), ", Y {ZPY}"].join(""));
                }
                AddrMode::REL => {
                    value = io.cpu_read(addr as u16);
                    addr += 1;
                    let dest = {
                        if (value & 0x80) > 0 {
                            addr - ((!value + 1) as u32)
                        } else {
                            addr + (value as u32)
                        }
                    };
                    instruction_str.push_str(
                        &[
                            "0x",
                            &Disassembler::hex(value as u32, 2),
                            " [0x",
                            &Disassembler::hex(dest, 4),
                            "] {REL}",
                        ]
                        .join(""),
                    );
                }
                AddrMode::ABS => {
                    lo = io.cpu_read(addr as u16);
                    addr += 1;
                    hi = io.cpu_read(addr as u16);
                    addr += 1;
                    instruction_str.push_str(
                        &[
                            "0x",
                            &Disassembler::hex((((hi as u16) << 8) | lo as u16) as u32, 4),
                            " {ABS}",
                        ]
                        .join(""),
                    );
                }
                AddrMode::ABX => {
                    lo = io.cpu_read(addr as u16);
                    addr += 1;
                    hi = io.cpu_read(addr as u16);
                    addr += 1;
                    instruction_str.push_str(
                        &[
                            "0x",
                            &Disassembler::hex((((hi as u16) << 8) | lo as u16) as u32, 4),
                            ", X {ABX}",
                        ]
                        .join(""),
                    );
                }
                AddrMode::ABY => {
                    lo = io.cpu_read(addr as u16);
                    addr += 1;
                    hi = io.cpu_read(addr as u16);
                    addr += 1;
                    instruction_str.push_str(
                        &[
                            "0x",
                            &Disassembler::hex((((hi as u16) << 8) | lo as u16) as u32, 4),
                            ", Y {ABY}",
                        ]
                        .join(""),
                    );
                }
                AddrMode::IND => {
                    lo = io.cpu_read(addr as u16);
                    addr += 1;
                    hi = io.cpu_read(addr as u16);
                    addr += 1;
                    instruction_str.push_str(
                        &[
                            "0x",
                            &Disassembler::hex((((hi as u16) << 8) | lo as u16) as u32, 4),
                            " {IND}",
                        ]
                        .join(""),
                    );
                }
                AddrMode::IZX => {
                    lo = io.cpu_read(addr as u16);
                    // hi = 0x00;
                    addr += 1;
                    instruction_str.push_str(
                        &["(0x", &Disassembler::hex(lo as u32, 2), ", X) {IZX}"].join(""),
                    );
                }
                AddrMode::IZY => {
                    lo = io.cpu_read(addr as u16);
                    // hi = 0x00;
                    addr += 1;
                    instruction_str.push_str(
                        &["(0x", &Disassembler::hex(lo as u32, 2), ", Y) {IZY}"].join(""),
                    );
                }
            }
            instructions.push(instruction_str);
            lines_map.insert(line_addr, instr_index);
            instr_index += 1;
        }

        (lines_map, instructions)
    }

    pub fn hex(n: u32, d: u8) -> String {
        let mut bytes: Vec<u8> = Vec::new();
        let mut number: u32 = n;
        for _i in 0..d {
            bytes.push((number & 0xF) as u8);
            number >>= 4;
        }
        bytes.reverse();
        let hex: String = bytes
            .iter()
            .map(|b| format!("{:01X}", b).to_string())
            .collect::<Vec<String>>()
            .join("");
        hex
    }
}
