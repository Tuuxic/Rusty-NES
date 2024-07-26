use std::{collections::HashMap, time::Duration};

use crate::{cartridge::cartridge::Cartridge, cpu::cpu::Cpu, disassembler::Disassembler};

pub const FRAME_LENGTH: Duration = Duration::from_millis(1); // Duration::new(0, 16_666_666);

pub struct Nes {
    cpu: Cpu,
    clock_counter: u64,
    frame_delta_time: f64,
    debug_dissassembly: (HashMap<u16, u16>, Vec<String>),
}

impl Nes {
    pub fn new() -> Nes {
        let debug_dissassembly = (HashMap::new(), vec![]);
        let cartridge = Box::new(Cartridge::new());
        let cpu = Cpu::new(cartridge);
        Nes {
            cpu,
            clock_counter: 0,
            frame_delta_time: 0.0,
            debug_dissassembly,
        }
    }

    pub fn init(&mut self) {
        let program: Vec<u8> = vec![
            0xA2, 0x0A, 0x8E, 0xB0, 0x00, 0xA2, 0x03, 0x8E, 0xB1, 0x00, 0xAC, 0xB0, 0x00, 0xA9,
            0x00, 0x18, 0x6D, 0xB1, 0x00, 0x88, 0xD0, 0xFA, 0x8D, 0xB2, 0x00, 0xEA, 0xEA, 0xEA,
            0x4c, 0x00, 0x00,
        ];
        let offset: u16 = 0x0000;

        for (i, num) in program.into_iter().enumerate() {
            // self.ram.ram[(offset as usize) + i] = num;
            self.cpu.bus.write(offset + (i as u16), num);
        }

        // self.ram.ram[0xFFFC] = 0x00;
        // self.ram.ram[0xFFFD] = 0x80;

        // let mut io = IODevice::new(&mut self.ram);
        self.debug_dissassembly = Disassembler::dissassemble(0x0000, 0xFFFF, &mut self.cpu.bus);
        self.reset();
    }

    pub fn update(&mut self, dt: Duration) {
        self.frame_delta_time += dt.as_secs_f64();
        if self.frame_delta_time > FRAME_LENGTH.as_secs_f64() {
            self.clock();
            self.frame_delta_time = 0.0;
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset()
    }

    pub fn step(&mut self) {
        while self.cpu.cycles == 0 {
            self.clock();
        }
        while self.cpu.cycles != 0 {
            self.clock();
        }
    }

    pub fn get_debug_code(&mut self) -> String {
        let range: usize = 12;
        let mut str: String = String::from("");
        let (disassembler, _) = &self.debug_dissassembly;
        if !disassembler.contains_key(&self.cpu.pc) {
            self.redissassamble();
        }
        let (disassembler, instructions) = &self.debug_dissassembly;

        if !disassembler.contains_key(&self.cpu.pc) {
            return "--- Dissassembly Error ---".to_string();
        }
        let pc_index: usize = disassembler[&(self.cpu.pc)] as usize;

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

    pub fn get_debug_registers(&mut self) -> String {
        let mut str = String::from("");

        str.push_str(&["Status: ", &format!("{:08b}", self.cpu.status), "\n"].join(""));
        str.push_str(&["PC: 0x", &Disassembler::hex(self.cpu.pc as u32, 4), "\n"].join(""));
        str.push_str(
            &[
                "A: 0x",
                &Disassembler::hex(self.cpu.a as u32, 2),
                " [",
                &self.cpu.a.to_string(),
                "]",
                "\n",
            ]
            .join(""),
        );
        str.push_str(
            &[
                "X: 0x",
                &Disassembler::hex(self.cpu.x as u32, 2),
                " [",
                &self.cpu.x.to_string(),
                "]",
                "\n",
            ]
            .join(""),
        );
        str.push_str(
            &[
                "Y: 0x",
                &Disassembler::hex(self.cpu.y as u32, 2),
                " [",
                &self.cpu.y.to_string(),
                "]",
                "\n",
            ]
            .join(""),
        );
        str.push_str(&["StackPtr: 0x", &Disassembler::hex(self.cpu.stkp as u32, 4)].join(""));

        str
    }

    pub fn get_debug_ram(&mut self, start: u16, rows: u32, cols: u32) -> String {
        // let io = IODevice::new(&mut self.ram, &mut self.ppu_ram, &mut self.cartridge);
        let mut str = String::from("");
        let mut offset = 0;
        for _ in 0..rows {
            str.push_str(&["0x", &Disassembler::hex((start + offset) as u32, 4), ":"].join(""));
            for _ in 0..cols {
                str.push_str(
                    &[
                        " ",
                        &Disassembler::hex(self.cpu.bus.read(start + offset) as u32, 2),
                    ]
                    .join(""),
                );
                offset += 1;
            }
            str.push_str("\n");
        }

        str
    }

    fn clock(&mut self) {
        self.cpu.clock_ppu();
        if self.clock_counter % 3 == 0 {
            self.cpu.clock();
        }
        self.clock_counter += 1;
    }

    #[allow(unused)] // TODO: Remove unused
    fn insert_cartridge(&mut self, cartridge: Cartridge) {
        self.cpu.bus.change_cartridge(Box::new(cartridge));
    }

    fn redissassamble(&mut self) {
        self.debug_dissassembly = Disassembler::dissassemble(0x0000, 0xFFFF, &mut self.cpu.bus);
    }
}
