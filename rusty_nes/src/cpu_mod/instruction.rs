use crate::iodevice::IODevice;

use super::{cpu::Cpu, cpu6502::Cpu6502, flags6502::Flags6502};

pub struct Instruction {
    name: String,
    operator: Box<dyn ExecutableOperation>,
    addrmode: Box<dyn ExecutableOperation>,
    addrtype: AddrMode,
    cycles: u8,
}

/*
pub struct InstructionRepository {
    instruction_lookup: HashMap<u8, Instruction>
}
*/

impl Instruction {
    pub fn new(
        name: &str,
        operator: Box<dyn ExecutableOperation>,
        addrmode: Box<dyn ExecutableOperation>,
        addrtype: AddrMode,
        cycles: u8,
    ) -> Instruction {
        Instruction {
            name: name.to_string(),
            operator,
            addrmode,
            addrtype,
            cycles,
        }
    }

    pub fn from_opcode(opcode: u8) -> Instruction {
        match opcode {
            0 => Instruction::new("BRK", Box::new(BRK), Box::new(IMM), AddrMode::IMM, 7),
            1 => Instruction::new("ORA", Box::new(ORA), Box::new(IZX), AddrMode::IZX, 6),
            2 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            3 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            4 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 3),
            5 => Instruction::new("ORA", Box::new(ORA), Box::new(ZP0), AddrMode::ZP0, 3),
            6 => Instruction::new("ASL", Box::new(ASL), Box::new(ZP0), AddrMode::ZP0, 5),
            7 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 5),
            8 => Instruction::new("PHP", Box::new(PHP), Box::new(IMP), AddrMode::IMP, 3),
            9 => Instruction::new("ORA", Box::new(ORA), Box::new(IMM), AddrMode::IMM, 2),
            10 => Instruction::new("ASL", Box::new(ASL), Box::new(IMP), AddrMode::IMP, 2),
            11 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            12 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            13 => Instruction::new("ORA", Box::new(ORA), Box::new(ABS), AddrMode::ABS, 4),
            14 => Instruction::new("ASL", Box::new(ASL), Box::new(ABS), AddrMode::ABS, 6),
            15 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            16 => Instruction::new("BPL", Box::new(BPL), Box::new(REL), AddrMode::REL, 2),
            17 => Instruction::new("ORA", Box::new(ORA), Box::new(IZY), AddrMode::IZY, 5),
            18 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            19 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            20 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            21 => Instruction::new("ORA", Box::new(ORA), Box::new(ZPX), AddrMode::ZPX, 4),
            22 => Instruction::new("ASL", Box::new(ASL), Box::new(ZPX), AddrMode::ZPX, 6),
            23 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            24 => Instruction::new("CLC", Box::new(CLC), Box::new(IMP), AddrMode::IMP, 2),
            25 => Instruction::new("ORA", Box::new(ORA), Box::new(ABY), AddrMode::ABY, 4),
            26 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            27 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
            28 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            29 => Instruction::new("ORA", Box::new(ORA), Box::new(ABX), AddrMode::ABX, 4),
            30 => Instruction::new("ASL", Box::new(ASL), Box::new(ABX), AddrMode::ABX, 7),
            31 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
            32 => Instruction::new("JSR", Box::new(JSR), Box::new(ABS), AddrMode::ABS, 6),
            33 => Instruction::new("AND", Box::new(AND), Box::new(IZX), AddrMode::IZX, 6),
            34 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            35 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            36 => Instruction::new("BIT", Box::new(BIT), Box::new(ZP0), AddrMode::ZP0, 3),
            37 => Instruction::new("AND", Box::new(AND), Box::new(ZP0), AddrMode::ZP0, 3),
            38 => Instruction::new("ROL", Box::new(ROL), Box::new(ZP0), AddrMode::ZP0, 5),
            39 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 5),
            40 => Instruction::new("PLP", Box::new(PLP), Box::new(IMP), AddrMode::IMP, 4),
            41 => Instruction::new("AND", Box::new(AND), Box::new(IMM), AddrMode::IMM, 2),
            42 => Instruction::new("ROL", Box::new(ROL), Box::new(IMP), AddrMode::IMP, 2),
            43 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            44 => Instruction::new("BIT", Box::new(BIT), Box::new(ABS), AddrMode::ABS, 4),
            45 => Instruction::new("AND", Box::new(AND), Box::new(ABS), AddrMode::ABS, 4),
            46 => Instruction::new("ROL", Box::new(ROL), Box::new(ABS), AddrMode::ABS, 6),
            47 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            48 => Instruction::new("BMI", Box::new(BMI), Box::new(REL), AddrMode::REL, 2),
            49 => Instruction::new("AND", Box::new(AND), Box::new(IZY), AddrMode::IZY, 5),
            50 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            51 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            52 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            53 => Instruction::new("AND", Box::new(AND), Box::new(ZPX), AddrMode::ZPX, 4),
            54 => Instruction::new("ROL", Box::new(ROL), Box::new(ZPX), AddrMode::ZPX, 6),
            55 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            56 => Instruction::new("SEC", Box::new(SEC), Box::new(IMP), AddrMode::IMP, 2),
            57 => Instruction::new("AND", Box::new(AND), Box::new(ABY), AddrMode::ABY, 4),
            58 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            59 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
            60 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            61 => Instruction::new("AND", Box::new(AND), Box::new(ABX), AddrMode::ABX, 4),
            62 => Instruction::new("ROL", Box::new(ROL), Box::new(ABX), AddrMode::ABX, 7),
            63 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
            64 => Instruction::new("RTI", Box::new(RTI), Box::new(IMP), AddrMode::IMP, 6),
            65 => Instruction::new("EOR", Box::new(EOR), Box::new(IZX), AddrMode::IZX, 6),
            66 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            67 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            68 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 3),
            69 => Instruction::new("EOR", Box::new(EOR), Box::new(ZP0), AddrMode::ZP0, 3),
            70 => Instruction::new("LSR", Box::new(LSR), Box::new(ZP0), AddrMode::ZP0, 5),
            71 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 5),
            72 => Instruction::new("PHA", Box::new(PHA), Box::new(IMP), AddrMode::IMP, 3),
            73 => Instruction::new("EOR", Box::new(EOR), Box::new(IMM), AddrMode::IMM, 2),
            74 => Instruction::new("LSR", Box::new(LSR), Box::new(IMP), AddrMode::IMP, 2),
            75 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            76 => Instruction::new("JMP", Box::new(JMP), Box::new(ABS), AddrMode::ABS, 3),
            77 => Instruction::new("EOR", Box::new(EOR), Box::new(ABS), AddrMode::ABS, 4),
            78 => Instruction::new("LSR", Box::new(LSR), Box::new(ABS), AddrMode::ABS, 6),
            79 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            80 => Instruction::new("BVC", Box::new(BVC), Box::new(REL), AddrMode::REL, 2),
            81 => Instruction::new("EOR", Box::new(EOR), Box::new(IZY), AddrMode::IZY, 5),
            82 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            83 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            84 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            85 => Instruction::new("EOR", Box::new(EOR), Box::new(ZPX), AddrMode::ZPX, 4),
            86 => Instruction::new("LSR", Box::new(LSR), Box::new(ZPX), AddrMode::ZPX, 6),
            87 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            88 => Instruction::new("CLI", Box::new(CLI), Box::new(IMP), AddrMode::IMP, 2),
            89 => Instruction::new("EOR", Box::new(EOR), Box::new(ABY), AddrMode::ABY, 4),
            90 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            91 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
            92 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            93 => Instruction::new("EOR", Box::new(EOR), Box::new(ABX), AddrMode::ABX, 4),
            94 => Instruction::new("LSR", Box::new(LSR), Box::new(ABX), AddrMode::ABX, 7),
            95 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
            96 => Instruction::new("RTS", Box::new(RTS), Box::new(IMP), AddrMode::IMP, 6),
            97 => Instruction::new("ADC", Box::new(ADC), Box::new(IZX), AddrMode::IZX, 6),
            98 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            99 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            100 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 3),
            101 => Instruction::new("ADC", Box::new(ADC), Box::new(ZP0), AddrMode::ZP0, 3),
            102 => Instruction::new("ROR", Box::new(ROR), Box::new(ZP0), AddrMode::ZP0, 5),
            103 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 5),
            104 => Instruction::new("PLA", Box::new(PLA), Box::new(IMP), AddrMode::IMP, 4),
            105 => Instruction::new("ADC", Box::new(ADC), Box::new(IMM), AddrMode::IMM, 2),
            106 => Instruction::new("ROR", Box::new(ROR), Box::new(IMP), AddrMode::IMP, 2),
            107 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            108 => Instruction::new("JMP", Box::new(JMP), Box::new(IND), AddrMode::IND, 5),
            109 => Instruction::new("ADC", Box::new(ADC), Box::new(ABS), AddrMode::ABS, 4),
            110 => Instruction::new("ROR", Box::new(ROR), Box::new(ABS), AddrMode::ABS, 6),
            111 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            112 => Instruction::new("BVS", Box::new(BVS), Box::new(REL), AddrMode::REL, 2),
            113 => Instruction::new("ADC", Box::new(ADC), Box::new(IZY), AddrMode::IZY, 5),
            114 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            115 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            116 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            117 => Instruction::new("ADC", Box::new(ADC), Box::new(ZPX), AddrMode::ZPX, 4),
            118 => Instruction::new("ROR", Box::new(ROR), Box::new(ZPX), AddrMode::ZPX, 6),
            119 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            120 => Instruction::new("SEI", Box::new(SEI), Box::new(IMP), AddrMode::IMP, 2),
            121 => Instruction::new("ADC", Box::new(ADC), Box::new(ABY), AddrMode::ABY, 4),
            122 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            123 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
            124 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            125 => Instruction::new("ADC", Box::new(ADC), Box::new(ABX), AddrMode::ABX, 4),
            126 => Instruction::new("ROR", Box::new(ROR), Box::new(ABX), AddrMode::ABX, 7),
            127 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
            128 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            129 => Instruction::new("STA", Box::new(STA), Box::new(IZX), AddrMode::IZX, 6),
            130 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            131 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            132 => Instruction::new("STY", Box::new(STY), Box::new(ZP0), AddrMode::ZP0, 3),
            133 => Instruction::new("STA", Box::new(STA), Box::new(ZP0), AddrMode::ZP0, 3),
            134 => Instruction::new("STX", Box::new(STX), Box::new(ZP0), AddrMode::ZP0, 3),
            135 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 3),
            136 => Instruction::new("DEY", Box::new(DEY), Box::new(IMP), AddrMode::IMP, 2),
            137 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            138 => Instruction::new("TXA", Box::new(TXA), Box::new(IMP), AddrMode::IMP, 2),
            139 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            140 => Instruction::new("STY", Box::new(STY), Box::new(ABS), AddrMode::ABS, 4),
            141 => Instruction::new("STA", Box::new(STA), Box::new(ABS), AddrMode::ABS, 4),
            142 => Instruction::new("STX", Box::new(STX), Box::new(ABS), AddrMode::ABS, 4),
            143 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 4),
            144 => Instruction::new("BCC", Box::new(BCC), Box::new(REL), AddrMode::REL, 2),
            145 => Instruction::new("STA", Box::new(STA), Box::new(IZY), AddrMode::IZY, 6),
            146 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            147 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            148 => Instruction::new("STY", Box::new(STY), Box::new(ZPX), AddrMode::ZPX, 4),
            149 => Instruction::new("STA", Box::new(STA), Box::new(ZPX), AddrMode::ZPX, 4),
            150 => Instruction::new("STX", Box::new(STX), Box::new(ZPY), AddrMode::ZPY, 4),
            151 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 4),
            152 => Instruction::new("TYA", Box::new(TYA), Box::new(IMP), AddrMode::IMP, 2),
            153 => Instruction::new("STA", Box::new(STA), Box::new(ABY), AddrMode::ABY, 5),
            154 => Instruction::new("TXS", Box::new(TXS), Box::new(IMP), AddrMode::IMP, 2),
            155 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 5),
            156 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 5),
            157 => Instruction::new("STA", Box::new(STA), Box::new(ABX), AddrMode::ABX, 5),
            158 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 5),
            159 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 5),
            160 => Instruction::new("LDY", Box::new(LDY), Box::new(IMM), AddrMode::IMM, 2),
            161 => Instruction::new("LDA", Box::new(LDA), Box::new(IZX), AddrMode::IZX, 6),
            162 => Instruction::new("LDX", Box::new(LDX), Box::new(IMM), AddrMode::IMM, 2),
            163 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            164 => Instruction::new("LDY", Box::new(LDY), Box::new(ZP0), AddrMode::ZP0, 3),
            165 => Instruction::new("LDA", Box::new(LDA), Box::new(ZP0), AddrMode::ZP0, 3),
            166 => Instruction::new("LDX", Box::new(LDX), Box::new(ZP0), AddrMode::ZP0, 3),
            167 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 3),
            168 => Instruction::new("TAY", Box::new(TAY), Box::new(IMP), AddrMode::IMP, 2),
            169 => Instruction::new("LDA", Box::new(LDA), Box::new(IMM), AddrMode::IMM, 2),
            170 => Instruction::new("TAX", Box::new(TAX), Box::new(IMP), AddrMode::IMP, 2),
            171 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            172 => Instruction::new("LDY", Box::new(LDY), Box::new(ABS), AddrMode::ABS, 4),
            173 => Instruction::new("LDA", Box::new(LDA), Box::new(ABS), AddrMode::ABS, 4),
            174 => Instruction::new("LDX", Box::new(LDX), Box::new(ABS), AddrMode::ABS, 4),
            175 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 4),
            176 => Instruction::new("BCS", Box::new(BCS), Box::new(REL), AddrMode::REL, 2),
            177 => Instruction::new("LDA", Box::new(LDA), Box::new(IZY), AddrMode::IZY, 5),
            178 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            179 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 5),
            180 => Instruction::new("LDY", Box::new(LDY), Box::new(ZPX), AddrMode::ZPX, 4),
            181 => Instruction::new("LDA", Box::new(LDA), Box::new(ZPX), AddrMode::ZPX, 4),
            182 => Instruction::new("LDX", Box::new(LDX), Box::new(ZPY), AddrMode::ZPY, 4),
            183 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 4),
            184 => Instruction::new("CLV", Box::new(CLV), Box::new(IMP), AddrMode::IMP, 2),
            185 => Instruction::new("LDA", Box::new(LDA), Box::new(ABY), AddrMode::ABY, 4),
            186 => Instruction::new("TSX", Box::new(TSX), Box::new(IMP), AddrMode::IMP, 2),
            187 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 4),
            188 => Instruction::new("LDY", Box::new(LDY), Box::new(ABX), AddrMode::ABX, 4),
            189 => Instruction::new("LDA", Box::new(LDA), Box::new(ABX), AddrMode::ABX, 4),
            190 => Instruction::new("LDX", Box::new(LDX), Box::new(ABY), AddrMode::ABY, 4),
            191 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 4),
            192 => Instruction::new("CPY", Box::new(CPY), Box::new(IMM), AddrMode::IMM, 2),
            193 => Instruction::new("CMP", Box::new(CMP), Box::new(IZX), AddrMode::IZX, 6),
            194 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            195 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            196 => Instruction::new("CPY", Box::new(CPY), Box::new(ZP0), AddrMode::ZP0, 3),
            197 => Instruction::new("CMP", Box::new(CMP), Box::new(ZP0), AddrMode::ZP0, 3),
            198 => Instruction::new("DEC", Box::new(DEC), Box::new(ZP0), AddrMode::ZP0, 5),
            199 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 5),
            200 => Instruction::new("INY", Box::new(INY), Box::new(IMP), AddrMode::IMP, 2),
            201 => Instruction::new("CMP", Box::new(CMP), Box::new(IMM), AddrMode::IMM, 2),
            202 => Instruction::new("DEX", Box::new(DEX), Box::new(IMP), AddrMode::IMP, 2),
            203 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            204 => Instruction::new("CPY", Box::new(CPY), Box::new(ABS), AddrMode::ABS, 4),
            205 => Instruction::new("CMP", Box::new(CMP), Box::new(ABS), AddrMode::ABS, 4),
            206 => Instruction::new("DEC", Box::new(DEC), Box::new(ABS), AddrMode::ABS, 6),
            207 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            208 => Instruction::new("BNE", Box::new(BNE), Box::new(REL), AddrMode::REL, 2),
            209 => Instruction::new("CMP", Box::new(CMP), Box::new(IZY), AddrMode::IZY, 5),
            210 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            211 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            212 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            213 => Instruction::new("CMP", Box::new(CMP), Box::new(ZPX), AddrMode::ZPX, 4),
            214 => Instruction::new("DEC", Box::new(DEC), Box::new(ZPX), AddrMode::ZPX, 6),
            215 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            216 => Instruction::new("CLD", Box::new(CLD), Box::new(IMP), AddrMode::IMP, 2),
            217 => Instruction::new("CMP", Box::new(CMP), Box::new(ABY), AddrMode::ABY, 4),
            218 => Instruction::new("NOP", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            219 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
            220 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            221 => Instruction::new("CMP", Box::new(CMP), Box::new(ABX), AddrMode::ABX, 4),
            222 => Instruction::new("DEC", Box::new(DEC), Box::new(ABX), AddrMode::ABX, 7),
            223 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
            224 => Instruction::new("CPX", Box::new(CPX), Box::new(IMM), AddrMode::IMM, 2),
            225 => Instruction::new("SBC", Box::new(SBC), Box::new(IZX), AddrMode::IZX, 6),
            226 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            227 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            228 => Instruction::new("CPX", Box::new(CPX), Box::new(ZP0), AddrMode::ZP0, 3),
            229 => Instruction::new("SBC", Box::new(SBC), Box::new(ZP0), AddrMode::ZP0, 3),
            230 => Instruction::new("INC", Box::new(INC), Box::new(ZP0), AddrMode::ZP0, 5),
            231 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 5),
            232 => Instruction::new("INX", Box::new(INX), Box::new(IMP), AddrMode::IMP, 2),
            233 => Instruction::new("SBC", Box::new(SBC), Box::new(IMM), AddrMode::IMM, 2),
            234 => Instruction::new("NOP", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            235 => Instruction::new("???", Box::new(SBC), Box::new(IMP), AddrMode::IMP, 2),
            236 => Instruction::new("CPX", Box::new(CPX), Box::new(ABS), AddrMode::ABS, 4),
            237 => Instruction::new("SBC", Box::new(SBC), Box::new(ABS), AddrMode::ABS, 4),
            238 => Instruction::new("INC", Box::new(INC), Box::new(ABS), AddrMode::ABS, 6),
            239 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            240 => Instruction::new("BEQ", Box::new(BEQ), Box::new(REL), AddrMode::REL, 2),
            241 => Instruction::new("SBC", Box::new(SBC), Box::new(IZY), AddrMode::IZY, 5),
            242 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 2),
            243 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 8),
            244 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            245 => Instruction::new("SBC", Box::new(SBC), Box::new(ZPX), AddrMode::ZPX, 4),
            246 => Instruction::new("INC", Box::new(INC), Box::new(ZPX), AddrMode::ZPX, 6),
            247 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 6),
            248 => Instruction::new("SED", Box::new(SED), Box::new(IMP), AddrMode::IMP, 2),
            249 => Instruction::new("SBC", Box::new(SBC), Box::new(ABY), AddrMode::ABY, 4),
            250 => Instruction::new("NOP", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 2),
            251 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
            252 => Instruction::new("???", Box::new(NOP), Box::new(IMP), AddrMode::IMP, 4),
            253 => Instruction::new("SBC", Box::new(SBC), Box::new(ABX), AddrMode::ABX, 4),
            254 => Instruction::new("INC", Box::new(INC), Box::new(ABX), AddrMode::ABX, 7),
            255 => Instruction::new("???", Box::new(XXX), Box::new(IMP), AddrMode::IMP, 7),
        }
    }
    /*
        pub fn execute_operator(&self, cpu: &mut Cpu6502, bus: &mut CpuRAM) -> u8 {
            self.operator.execute(cpu, bus)
        }

        pub fn execute_addmode(&self, cpu: &mut Cpu6502, bus: &mut CpuRAM) -> u8 {
            self.addrmode.execute(cpu, bus)
        }
    */
    pub fn execute_operator(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        self.operator.execute(cpu, io)
    }

    pub fn execute_addrmode(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        self.addrmode.execute(cpu, io)
    }
    pub fn get_cycles(&self) -> u8 {
        self.cycles
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_addrmode(&self) -> AddrMode {
        self.addrtype
    }

    pub fn opcode_to_addrmode(opcode: u8) -> AddrMode {
        return Instruction::from_opcode(opcode).get_addrmode();
    }
}

pub trait ExecutableOperation {
    // fn execute(&self, cpu: &mut Cpu6502, bus: &mut CpuRAM) -> u8;
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8;
}

#[derive(Clone, Copy)]
pub enum AddrMode {
    IMP,
    IMM,
    ZP0,
    ZPX,
    ZPY,
    REL,
    ABS,
    ABX,
    ABY,
    IND,
    IZX,
    IZY,
}

// Addressing Modes
pub struct IMP;
impl ExecutableOperation for IMP {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.fetched = cpu.a;
        0
    }
}

pub struct IMM;
impl ExecutableOperation for IMM {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.addr_abs = cpu.pc;
        cpu.pc += 1;
        0
    }
}

pub struct ZP0;
impl ExecutableOperation for ZP0 {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.addr_abs = io.read(cpu.pc) as u16;

        cpu.pc += 1;

        cpu.addr_abs &= 0x00FF;

        0
    }
}

pub struct ZPX;
impl ExecutableOperation for ZPX {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.addr_abs = (io.read(cpu.pc) + cpu.x) as u16;

        cpu.pc += 1;

        cpu.addr_abs &= 0x00FF;

        0
    }
}
pub struct ZPY;
impl ExecutableOperation for ZPY {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.addr_abs = (io.read(cpu.pc) + cpu.y) as u16;

        cpu.pc += 1;

        cpu.addr_abs &= 0x00FF;

        0
    }
}
pub struct REL;
impl ExecutableOperation for REL {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.addr_rel = io.read(cpu.pc) as u16;

        cpu.pc += 1;

        if (cpu.addr_rel & 0x80) != 0 {
            cpu.addr_rel |= 0xFF00;
        }

        0
    }
}
pub struct ABS;
impl ExecutableOperation for ABS {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        let lo: u16 = io.read(cpu.pc) as u16;

        cpu.pc += 1;

        let hi: u16 = io.read(cpu.pc) as u16;

        cpu.pc += 1;

        cpu.addr_abs = (hi << 8) | lo;

        0
    }
}
pub struct ABX;
impl ExecutableOperation for ABX {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        let lo: u16 = io.read(cpu.pc) as u16;

        cpu.pc += 1;

        let hi: u16 = io.read(cpu.pc) as u16;

        cpu.pc += 1;

        cpu.addr_abs = (hi << 8) | lo;

        cpu.addr_abs += cpu.x as u16;

        if (hi << 8) != (cpu.addr_abs & 0xFF00) {
            1
        } else {
            0
        }
    }
}
pub struct ABY;
impl ExecutableOperation for ABY {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        let lo: u16 = io.read(cpu.pc) as u16;

        cpu.pc += 1;

        let hi: u16 = io.read(cpu.pc) as u16;

        cpu.pc += 1;

        cpu.addr_abs = (hi << 8) | lo;

        cpu.addr_abs += cpu.y as u16;

        if (hi << 8) != (cpu.addr_abs & 0xFF00) {
            1
        } else {
            0
        }
    }
}
pub struct IND;
impl ExecutableOperation for IND {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        let ptr_lo: u16 = io.read(cpu.pc) as u16;

        cpu.pc += 1;

        let ptr_hi: u16 = io.read(cpu.pc) as u16;

        cpu.pc += 1;

        let ptr: u16 = (ptr_hi << 8) | ptr_lo;

        if ptr_lo == 0x00FF {
            let hi: u16 = io.read(ptr & 0xFF00) as u16;

            let lo: u16 = io.read(ptr) as u16;

            cpu.addr_abs = (hi << 8) | lo;
        } else {
            let hi: u16 = io.read(ptr + 1) as u16;

            let lo: u16 = io.read(ptr) as u16;

            cpu.addr_abs = (hi << 8) | lo;
        }

        0
    }
}
pub struct IZX;
impl ExecutableOperation for IZX {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        let t: u16 = io.read(cpu.pc) as u16;

        cpu.pc += 1;

        let lo: u16 = io.read(((t + (cpu.x as u16)) as u16) & 0x00FF) as u16;

        let hi: u16 = io.read(((t + (cpu.x as u16) + 1) as u16) & 0x00FF) as u16;

        cpu.addr_abs = (hi << 8) | lo;

        0
    }
}

pub struct IZY;
impl ExecutableOperation for IZY {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        let t: u16 = io.read(cpu.pc) as u16;

        cpu.pc += 1;

        let lo: u16 = io.read(t & 0x00FF) as u16;

        let hi: u16 = io.read((t + 1) & 0x00FF) as u16;

        cpu.addr_abs = (hi << 8) | lo;

        cpu.addr_abs += cpu.y as u16;

        if (cpu.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }
}

// Operators
pub struct ADC;
impl ExecutableOperation for ADC {
    /*
    fn execute(&self, cpu: &mut Cpu6502, bus: &mut CpuRAM) -> u8 {
        println!("ADC executed");
        0
    }
    */
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);

        cpu.temp = cpu.a as u16 + cpu.fetched as u16 + cpu.get_flag(Flags6502::C) as u16;

        cpu.set_flag(Flags6502::C, cpu.temp > 255);
        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0);
        cpu.set_flag(
            Flags6502::V,
            (!(cpu.a as u16 ^ cpu.fetched as u16) & (cpu.a as u16 ^ cpu.temp as u16)) != 0,
        );
        cpu.set_flag(Flags6502::N, (cpu.temp & 0x80) != 0);

        cpu.a = (cpu.temp & 0x00FF) as u8;

        1
    }
}

pub struct SBC;
impl ExecutableOperation for SBC {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);

        let value: u16 = (cpu.fetched as u16) ^ 0x00FF;
        cpu.temp = cpu.a as u16 + value + cpu.get_flag(Flags6502::C) as u16;

        cpu.set_flag(Flags6502::C, (cpu.temp & 0xFF00) != 0);
        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0);
        cpu.set_flag(
            Flags6502::V,
            ((cpu.temp ^ cpu.a as u16) & (cpu.temp ^ value) & 0x0080) != 0,
        );
        cpu.set_flag(Flags6502::N, (cpu.temp & 0x0080) != 0);

        cpu.a = (cpu.temp & 0x00FF) as u8;

        1
    }
}

pub struct AND;
impl ExecutableOperation for AND {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);
        cpu.a = cpu.a & cpu.fetched;

        cpu.set_flag(Flags6502::Z, cpu.a == 0);
        cpu.set_flag(Flags6502::N, (cpu.a & 0x80) != 0);
        1
    }
}

pub struct ASL;
impl ExecutableOperation for ASL {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);
        cpu.temp = (cpu.fetched as u16) << 1;

        cpu.set_flag(Flags6502::C, (cpu.temp & 0xFF00) > 0);
        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.temp & 0x80) != 0);
        if matches!(
            Instruction::from_opcode(cpu.opcode).get_addrmode(),
            AddrMode::IMP
        ) {
            cpu.a = (cpu.temp & 0x00FF) as u8;
        } else {
            io.write(cpu.addr_abs, (cpu.temp & 0x00FF) as u8);
        }
        0
    }
}

pub struct BCC;
impl ExecutableOperation for BCC {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        if cpu.get_flag(Flags6502::C) == 0 {
            cpu.cycles += 1; // Maybe move to return value

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }

            cpu.pc = cpu.addr_abs
        }

        0
    }
}

pub struct BCS;
impl ExecutableOperation for BCS {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        if cpu.get_flag(Flags6502::C) == 1 {
            cpu.cycles += 1; // Maybe move to return value

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }

            cpu.pc = cpu.addr_abs
        }

        0
    }
}

pub struct BEQ;
impl ExecutableOperation for BEQ {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        if cpu.get_flag(Flags6502::Z) == 1 {
            cpu.cycles += 1; // Maybe move to return value

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }

            cpu.pc = cpu.addr_abs
        }

        0
    }
}

pub struct BIT;
impl ExecutableOperation for BIT {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);

        cpu.temp = (cpu.a & cpu.fetched) as u16;

        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.fetched & (1 << 7)) != 0);
        cpu.set_flag(Flags6502::V, (cpu.fetched & (1 << 6)) != 0);

        0
    }
}

pub struct BMI;
impl ExecutableOperation for BMI {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        if cpu.get_flag(Flags6502::N) == 1 {
            cpu.cycles += 1; // Maybe move to return value

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }

            cpu.pc = cpu.addr_abs
        }

        0
    }
}

pub struct BNE;
impl ExecutableOperation for BNE {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        if cpu.get_flag(Flags6502::Z) == 0 {
            cpu.cycles += 1; // Maybe move to return value

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }

            cpu.pc = cpu.addr_abs
        }

        0
    }
}

pub struct BPL;
impl ExecutableOperation for BPL {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        if cpu.get_flag(Flags6502::N) == 0 {
            cpu.cycles += 1; // Maybe move to return value

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }

            cpu.pc = cpu.addr_abs
        }

        0
    }
}

pub struct BRK;
impl ExecutableOperation for BRK {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.pc += 1;

        cpu.set_flag(Flags6502::I, true);
        io.write(0x0100 + (cpu.stkp as u16), ((cpu.pc >> 8) & 0x00FF) as u8);
        cpu.stkp -= 1;
        io.write(0x0100 + (cpu.stkp as u16), (cpu.pc & 0x00FF) as u8);
        cpu.stkp -= 1;

        cpu.set_flag(Flags6502::B, true);
        io.write(0x0100 + (cpu.stkp as u16), cpu.status);
        cpu.stkp -= 1;
        cpu.set_flag(Flags6502::B, false);

        cpu.pc = io.read(0xFFFE) as u16 | ((io.read(0xFFFF) as u16) << 8);
        0
    }
}

pub struct BVC;
impl ExecutableOperation for BVC {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        if cpu.get_flag(Flags6502::V) == 0 {
            cpu.cycles += 1; // Maybe move to return value

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }

            cpu.pc = cpu.addr_abs
        }

        0
    }
}

pub struct BVS;
impl ExecutableOperation for BVS {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        if cpu.get_flag(Flags6502::V) == 1 {
            cpu.cycles += 1; // Maybe move to return value

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }

            cpu.pc = cpu.addr_abs
        }

        0
    }
}

pub struct CLC;
impl ExecutableOperation for CLC {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.set_flag(Flags6502::C, false);
        0
    }
}

pub struct CLD;
impl ExecutableOperation for CLD {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.set_flag(Flags6502::D, false);
        0
    }
}

pub struct CLI;
impl ExecutableOperation for CLI {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.set_flag(Flags6502::I, false);
        0
    }
}

pub struct CLV;
impl ExecutableOperation for CLV {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.set_flag(Flags6502::V, false);
        0
    }
}

pub struct CMP;
impl ExecutableOperation for CMP {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);
        cpu.temp = cpu.a as u16 - cpu.fetched as u16;

        cpu.set_flag(Flags6502::C, cpu.a >= cpu.fetched);
        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(Flags6502::N, (cpu.temp & 0x0080) != 0);
        1
    }
}

pub struct CPX;
impl ExecutableOperation for CPX {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);
        cpu.temp = cpu.x as u16 - cpu.fetched as u16;

        cpu.set_flag(Flags6502::C, cpu.x >= cpu.fetched);
        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(Flags6502::N, (cpu.temp & 0x0080) != 0);
        0
    }
}

pub struct CPY;
impl ExecutableOperation for CPY {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);
        cpu.temp = cpu.y as u16 - cpu.fetched as u16;

        cpu.set_flag(Flags6502::C, cpu.y >= cpu.fetched);
        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(Flags6502::N, (cpu.temp & 0x0080) != 0);
        0
    }
}

pub struct DEC;
impl ExecutableOperation for DEC {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);
        cpu.temp = (cpu.fetched - 1) as u16;
        io.write(cpu.addr_abs, (cpu.temp & 0x00FF) as u8);
        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(Flags6502::N, (cpu.temp & 0x0080) != 0);
        0
    }
}

pub struct DEX;
impl ExecutableOperation for DEX {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.x -= 1;
        cpu.set_flag(Flags6502::Z, cpu.x == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.x & 0x80) != 0);
        0
    }
}

pub struct DEY;
impl ExecutableOperation for DEY {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.y -= 1;
        cpu.set_flag(Flags6502::Z, cpu.y == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.y & 0x80) != 0);
        0
    }
}

pub struct EOR;
impl ExecutableOperation for EOR {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);
        cpu.a = cpu.a ^ cpu.fetched;

        cpu.set_flag(Flags6502::Z, cpu.a == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.a & 0x80) != 0);

        1
    }
}

pub struct INC;
impl ExecutableOperation for INC {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);

        cpu.temp = (cpu.fetched as u16) + 1;
        io.write(cpu.addr_abs, (cpu.temp & 0x00FF) as u8);
        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(Flags6502::N, (cpu.temp & 0x0080) != 0);

        0
    }
}

pub struct INX;
impl ExecutableOperation for INX {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.x += 1;
        cpu.set_flag(Flags6502::Z, cpu.x == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.x & 0x80) != 0);

        0
    }
}

pub struct INY;
impl ExecutableOperation for INY {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.y += 1;
        cpu.set_flag(Flags6502::Z, cpu.y == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.y & 0x80) != 0);

        0
    }
}

pub struct JMP;
impl ExecutableOperation for JMP {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.pc = cpu.addr_abs;
        0
    }
}

pub struct JSR;
impl ExecutableOperation for JSR {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.pc -= 1;

        io.write(0x0100 + (cpu.stkp as u16), ((cpu.pc >> 8) & 0x00FF) as u8);
        cpu.stkp -= 1;

        io.write(0x0100 + (cpu.stkp as u16), (cpu.pc & 0x00FF) as u8);
        cpu.stkp -= 1;

        cpu.pc = cpu.addr_abs;
        0
    }
}

pub struct LDA;
impl ExecutableOperation for LDA {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);

        cpu.a = cpu.fetched;
        cpu.set_flag(Flags6502::Z, cpu.a == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.a & 0x80) != 0);
        1
    }
}

pub struct LDX;
impl ExecutableOperation for LDX {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);

        cpu.x = cpu.fetched;
        cpu.set_flag(Flags6502::Z, cpu.x == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.x & 0x80) != 0);
        1
    }
}

pub struct LDY;
impl ExecutableOperation for LDY {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);

        cpu.y = cpu.fetched;
        cpu.set_flag(Flags6502::Z, cpu.y == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.y & 0x80) != 0);
        1
    }
}

pub struct LSR;
impl ExecutableOperation for LSR {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);
        cpu.set_flag(Flags6502::C, (cpu.fetched & 0x0001) != 0);
        cpu.temp = (cpu.fetched >> 1) as u16;

        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(Flags6502::N, (cpu.temp & 0x0080) != 0);

        if matches!(
            Instruction::from_opcode(cpu.opcode).get_addrmode(),
            AddrMode::IMP
        ) {
            cpu.a = (cpu.temp & 0x00FF) as u8;
        } else {
            io.write(cpu.addr_abs, (cpu.temp & 0x00FF) as u8)
        }

        0
    }
}

pub struct NOP;
impl ExecutableOperation for NOP {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        // Not all nops are equal
        // TODO: Implement illegal opcodes

        let extra = match cpu.opcode {
            0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => 1,
            _ => 0,
        };

        extra
    }
}

pub struct ORA;
impl ExecutableOperation for ORA {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);
        cpu.a = cpu.a | cpu.fetched;

        cpu.set_flag(Flags6502::Z, cpu.a == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.a & 0x80) != 0);

        1
    }
}

pub struct PHA;
impl ExecutableOperation for PHA {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        io.write(0x0100 + (cpu.stkp as u16), cpu.a);
        cpu.stkp -= 1;
        0
    }
}

pub struct PHP;
impl ExecutableOperation for PHP {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        io.write(
            0x0100 + (cpu.stkp as u16),
            cpu.status | Flags6502::B.0 | Flags6502::U.0,
        );
        cpu.set_flag(Flags6502::B, false);
        cpu.set_flag(Flags6502::U, false);

        cpu.stkp -= 1;
        0
    }
}

pub struct PLA;
impl ExecutableOperation for PLA {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.stkp += 1;
        cpu.a = io.read(0x0100 + (cpu.stkp as u16));

        cpu.set_flag(Flags6502::Z, cpu.a == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.a & 0x80) != 0);

        0
    }
}

pub struct PLP;
impl ExecutableOperation for PLP {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.stkp += 1;
        cpu.status = io.read(0x0100 + (cpu.stkp as u16));

        cpu.set_flag(Flags6502::U, true);

        0
    }
}

pub struct ROL;
impl ExecutableOperation for ROL {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);
        cpu.temp = ((cpu.fetched << 1) as u16) | (cpu.get_flag(Flags6502::C) as u16);

        cpu.set_flag(Flags6502::C, (cpu.temp & 0xFF00) != 0x0000);
        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(Flags6502::N, (cpu.temp & 0x0080) != 0x0000);

        if matches!(
            Instruction::from_opcode(cpu.opcode).get_addrmode(),
            AddrMode::IMP
        ) {
            cpu.a = (cpu.temp & 0x00FF) as u8;
        } else {
            io.write(cpu.addr_abs, (cpu.temp & 0x00FF) as u8)
        }

        0
    }
}

pub struct ROR;
impl ExecutableOperation for ROR {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.fetch(io);
        cpu.temp = ((cpu.get_flag(Flags6502::C) << 7) as u16) | ((cpu.fetched >> 1) as u16);

        cpu.set_flag(Flags6502::C, (cpu.fetched & 0x01) != 0x00);
        cpu.set_flag(Flags6502::Z, (cpu.temp & 0x00FF) == 0x0000);
        cpu.set_flag(Flags6502::N, (cpu.temp & 0x0080) != 0x0000);

        if matches!(
            Instruction::from_opcode(cpu.opcode).get_addrmode(),
            AddrMode::IMP
        ) {
            cpu.a = (cpu.temp & 0x00FF) as u8;
        } else {
            io.write(cpu.addr_abs, (cpu.temp & 0x00FF) as u8)
        }

        0
    }
}

pub struct RTI;
impl ExecutableOperation for RTI {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.stkp += 1;
        cpu.status = io.read(0x0100 + (cpu.stkp as u16));
        cpu.status &= !Flags6502::B.0;
        cpu.status &= !Flags6502::U.0;

        cpu.stkp += 1;
        cpu.pc = io.read(0x0100 + (cpu.stkp as u16)) as u16;
        cpu.stkp += 1;
        cpu.pc |= (io.read(0x0100 + (cpu.stkp as u16)) as u16) << 8;
        0
    }
}

pub struct RTS;
impl ExecutableOperation for RTS {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        cpu.stkp += 1;
        cpu.pc = io.read(0x0100 + (cpu.stkp as u16)) as u16;
        cpu.stkp += 1;
        cpu.pc |= (io.read(0x0100 + (cpu.stkp as u16)) as u16) << 8;

        cpu.pc += 1;
        0
    }
}

pub struct SEC;
impl ExecutableOperation for SEC {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.set_flag(Flags6502::C, true);
        0
    }
}

pub struct SED;
impl ExecutableOperation for SED {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.set_flag(Flags6502::D, true);
        0
    }
}

pub struct SEI;
impl ExecutableOperation for SEI {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.set_flag(Flags6502::I, true);
        0
    }
}

pub struct STA;
impl ExecutableOperation for STA {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        io.write(cpu.addr_abs, cpu.a);
        0
    }
}

pub struct STX;
impl ExecutableOperation for STX {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        io.write(cpu.addr_abs, cpu.x);
        0
    }
}

pub struct STY;
impl ExecutableOperation for STY {
    fn execute(&self, cpu: &mut Cpu6502, io: &mut IODevice) -> u8 {
        io.write(cpu.addr_abs, cpu.y);
        0
    }
}

pub struct TAX;
impl ExecutableOperation for TAX {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.x = cpu.a;
        cpu.set_flag(Flags6502::Z, cpu.x == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.x & 0x80) != 0);
        0
    }
}

pub struct TAY;
impl ExecutableOperation for TAY {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.y = cpu.a;
        cpu.set_flag(Flags6502::Z, cpu.y == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.y & 0x80) != 0);
        0
    }
}

pub struct TSX;
impl ExecutableOperation for TSX {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.x = cpu.stkp;
        cpu.set_flag(Flags6502::Z, cpu.x == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.x & 0x80) != 0);
        0
    }
}

pub struct TXA;
impl ExecutableOperation for TXA {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.a = cpu.x;
        cpu.set_flag(Flags6502::Z, cpu.a == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.a & 0x80) != 0);
        0
    }
}

pub struct TXS;
impl ExecutableOperation for TXS {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.stkp = cpu.x;
        0
    }
}

pub struct TYA;
impl ExecutableOperation for TYA {
    fn execute(&self, cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        cpu.a = cpu.y;
        cpu.set_flag(Flags6502::Z, cpu.a == 0x00);
        cpu.set_flag(Flags6502::N, (cpu.a & 0x80) != 0);
        0
    }
}

pub struct XXX;
impl ExecutableOperation for XXX {
    fn execute(&self, _cpu: &mut Cpu6502, _io: &mut IODevice) -> u8 {
        0
    }
}
