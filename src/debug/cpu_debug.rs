use std::collections::HashMap;

use crate::{
    bus::cpu_bus::CpuBus,
    cpu::operations::{addrmode::AddrMode, instruction::Instruction},
    nes::Nes,
};

pub struct CpuDebug {
    pub debug_dissassembly: (HashMap<u16, u16>, Vec<String>),
}

impl CpuDebug {
    pub fn new(nes: &mut Nes) -> CpuDebug {
        CpuDebug {
            debug_dissassembly: Disassembler::dissassemble(0x0000, 0xFFFF, &mut nes.cpu.bus),
        }
    }

    pub fn get_debug_code(&mut self, nes: &mut Nes) -> String {
        let range: usize = 12;
        let mut str: String = String::from("");
        let (disassembler, _) = &self.debug_dissassembly;
        if !disassembler.contains_key(&nes.cpu.pc) {
            self.redissassamble(nes);
        }
        let (disassembler, instructions) = &self.debug_dissassembly;

        if !disassembler.contains_key(&nes.cpu.pc) {
            return "--- Dissassembly Error ---".to_string();
        }
        let pc_index: usize = disassembler[&(nes.cpu.pc)] as usize;

        let pre_buffer = {
            match pc_index < range {
                true => range - pc_index,
                false => 0,
            }
        };
        for _ in 0..(pre_buffer) {
            str.push_str("-----------------------\n");
        }

        for i in 1..(range + 1) {
            if pc_index < (range + 1 - i) {
                continue;
            }
            let instr = pc_index - (range + 1 - i);
            str.push_str(&instructions[instr]);
            str.push_str("\n");
        }

        str.push_str("> ");
        str.push_str(&instructions[pc_index]);
        str.push_str("\n");

        for i in 1..(range + 1) {
            if (pc_index + i) >= instructions.len() {
                continue;
            }
            let instr = (pc_index + i) as usize;
            str.push_str(&instructions[instr]);
            str.push_str("\n");
        }

        let post_buffer = {
            match instructions.len() <= (pc_index + range) {
                true => (pc_index + range) - instructions.len() + 1,
                false => 0,
            }
        };
        for _ in 0..(post_buffer) {
            str.push_str("-----------------------\n");
        }

        str
    }

    pub fn get_debug_registers(&self, nes: &mut Nes) -> String {
        let mut str = String::from("");

        str.push_str(&["Status: ", &format!("{:08b}", nes.cpu.status), "\n"].join(""));
        str.push_str(&["PC: 0x", &Disassembler::hex(nes.cpu.pc as u32, 4), "\n"].join(""));
        str.push_str(
            &[
                "A: 0x",
                &Disassembler::hex(nes.cpu.a as u32, 2),
                " [",
                &nes.cpu.a.to_string(),
                "]",
                "\n",
            ]
            .join(""),
        );
        str.push_str(
            &[
                "X: 0x",
                &Disassembler::hex(nes.cpu.x as u32, 2),
                " [",
                &nes.cpu.x.to_string(),
                "]",
                "\n",
            ]
            .join(""),
        );
        str.push_str(
            &[
                "Y: 0x",
                &Disassembler::hex(nes.cpu.y as u32, 2),
                " [",
                &nes.cpu.y.to_string(),
                "]",
                "\n",
            ]
            .join(""),
        );
        str.push_str(&["StackPtr: 0x", &Disassembler::hex(nes.cpu.stkp as u32, 4)].join(""));

        str
    }

    pub fn get_debug_ram(&self, nes: &mut Nes, start: u16, rows: u32, cols: u32) -> String {
        // let io = IODevice::new(&mut nes: &mut Nes.ram, &mut self.ppu_ram, &mut self.cartridge);
        let mut str = String::from("");
        let mut offset = 0;
        for _ in 0..rows {
            str.push_str(&["0x", &Disassembler::hex((start + offset) as u32, 4), ":"].join(""));
            for _ in 0..cols {
                str.push_str(
                    &[
                        " ",
                        &Disassembler::hex(nes.cpu.bus.read(start + offset) as u32, 2),
                    ]
                    .join(""),
                );
                offset += 1;
            }
            str.push_str("\n");
        }

        str
    }

    pub fn redissassamble(&mut self, nes: &mut Nes) {
        self.debug_dissassembly = Disassembler::dissassemble(0x0000, 0xFFFF, &mut nes.cpu.bus);
    }
}

pub struct Disassembler;
impl Disassembler {
    pub fn dissassemble(
        start_addr: u16,
        end_addr: u16,
        bus: &mut Box<CpuBus>,
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
            let opcode: u8 = bus.read(addr as u16);
            addr += 1;
            let instr = Instruction::from_opcode(opcode);
            instruction_str.push_str(&instr.get_name());
            instruction_str.push_str(" ");

            match instr.get_addrmode() {
                AddrMode::IMP => {
                    instruction_str.push_str(" {IMP}");
                }
                AddrMode::IMM => {
                    value = bus.read(addr as u16);
                    addr += 1;
                    instruction_str
                        .push_str(&["#0x", &Disassembler::hex(value as u32, 2), " {IMM}"].join(""));
                }
                AddrMode::ZP0 => {
                    lo = bus.read(addr as u16);
                    // hi = 0x00;
                    addr += 1;
                    instruction_str
                        .push_str(&["0x", &Disassembler::hex(lo as u32, 2), " {ZP0}"].join(""));
                }

                AddrMode::ZPX => {
                    lo = bus.read(addr as u16);
                    // hi = 0x00;
                    addr += 1;
                    instruction_str
                        .push_str(&["0x", &Disassembler::hex(lo as u32, 2), ", X {ZPX}"].join(""));
                }
                AddrMode::ZPY => {
                    lo = bus.read(addr as u16);
                    // hi = 0x00;
                    addr += 1;
                    instruction_str
                        .push_str(&["0x", &Disassembler::hex(lo as u32, 2), ", Y {ZPY}"].join(""));
                }
                AddrMode::REL => {
                    value = bus.read(addr as u16);
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
                    lo = bus.read(addr as u16);
                    addr += 1;
                    hi = bus.read(addr as u16);
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
                    lo = bus.read(addr as u16);
                    addr += 1;
                    hi = bus.read(addr as u16);
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
                    lo = bus.read(addr as u16);
                    addr += 1;
                    hi = bus.read(addr as u16);
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
                    lo = bus.read(addr as u16);
                    addr += 1;
                    hi = bus.read(addr as u16);
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
                    lo = bus.read(addr as u16);
                    // hi = 0x00;
                    addr += 1;
                    instruction_str.push_str(
                        &["(0x", &Disassembler::hex(lo as u32, 2), ", X) {IZX}"].join(""),
                    );
                }
                AddrMode::IZY => {
                    lo = bus.read(addr as u16);
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
