#![allow(dead_code)]

use crate::gameboy::rom::Rom;
use std::collections::BTreeMap;
use std::fmt;

pub struct Disassembler {
    rom: Rom,
    pc: usize,
}

impl Disassembler {
    pub fn new(rom: Rom) -> Self {
        Disassembler { rom: rom, pc: 0 }
    }
    pub fn disassemble(&mut self) -> Disassembly {
        let mut instructions: BTreeMap<u16, Inst> = BTreeMap::new();

        while self.pc < self.rom.size() {
            let orig_pc = self.pc;
            let code = self.next_code();
            let (dis, bytes) = match disassemble_code(code) {
                Some(val) => val,
                None => continue,
            };
            let mut values: Vec<u8> = Vec::new();

            for _ in 0..bytes {
                let b = self.read_byte();
                values.push(b);
            }
            // let vals = values.reverse();
            instructions.insert(orig_pc as u16, Inst::new(code, orig_pc, dis, values));
        }
        Disassembly {
            instructions: instructions,
        }
    }
    fn next_code(&mut self) -> u16 {
        match self.read_byte() {
            0xCB => (0xCB << 8) | self.read_byte() as u16,
            val => val as u16,
        }
    }
    fn read_byte(&mut self) -> u8 {
        let b = self.rom.read_raw(self.pc);
        self.pc += 1;
        b
    }
}

pub struct Disassembly {
    instructions: BTreeMap<u16, Inst>,
}
impl Disassembly {
    pub fn print(&self) {
        for ins in &self.instructions {
            println!("{:?}", ins.1);
        }
    }
}

struct Inst {
    code: u16,
    values: Vec<u8>,
    dis: String,
    loc: usize,
}

impl Inst {
    pub fn new(code: u16, loc: usize, dis: String, values: Vec<u8>) -> Self {
        Inst {
            code: code,
            values: values,
            dis: dis,
            loc: loc,
        }
    }
}

impl fmt::Debug for Inst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self.values.len() {
            2 => Some((self.values[0] as u16) << 8 | self.values[1] as u16),
            1 => Some(self.values[0] as u16),
            _ => None,
        };
        let dis = &self.dis;
        let dissed = match value {
            Some(v) => dis
                .replace("{r}", &format!("{}", &v))
                .replace("{$2}", &format!("${:02x}", &v))
                .replace("{$4}", &format!("${:04x}", &v))
                .replace("{$2h}", &format!("$ff00 + {:02x}", &v)),
            None => dis.to_string(),
        };
        match self.values.len() {
            0 => write!(f, "{:04X} |           {}", self.loc, dissed),
            1 => write!(
                f,
                "{:04X} | {:02X}        {}",
                self.loc, self.values[0], dissed
            ),
            2 => write!(
                f,
                "{:04X} | {:02X} {:02X}     {}",
                self.loc, self.values[0], self.values[1], dissed
            ),
            _ => panic!("Can't have more than 2 values."),
        }
    }
}

pub fn disassemble_code(code: u16) -> Option<(String, u8)> {
    let big = (code & 0xFF00) >> 8;
    let lil = code & 0x00FF;

    match big {
        0xCB => {
            let instr = match lil {
                0x00..=0x07 => "RLC",
                0x08..=0x0F => "RRC",
                0x10..=0x17 => "RL",
                0x18..=0x1F => "RR",
                0x20..=0x27 => "SLA",
                0x28..=0x2F => "SRA",
                0x30..=0x37 => "SWAP",
                0x38..=0x3F => "SRL",
                0x40..=0x7F => "BIT",
                0x80..=0xBF => "RES",
                0xC0..=0xFF => "SET",
                _ => panic!("Not a valid CB code."),
            };

            let operand = match lil & 0x0F {
                0x00 | 0x08 => "B",
                0x01 | 0x09 => "C",
                0x02 | 0x0A => "D",
                0x03 | 0x0B => "E",
                0x04 | 0x0C => "H",
                0x05 | 0x0D => "L",
                0x06 | 0x0E => "(HL)",
                0x07 | 0x0F => "A",
                _ => panic!("Not a valid CB code."),
            };

            let operator = match instr {
                "RES" | "BIT" | "SET" => format!("{} {}", instr, (lil % 0x40) / 8),
                _ => instr.to_string(),
            };
            return Some((format!("{}, {}", operator, operand), 0));
        }
        0x00 => {
            let (dis, size) = match lil {
                0x00 => ("NOP", 0),
                0x01 => ("LD BC, {$4}", 2),
                0x02 => ("LD (BC), A", 0),
                0x03 => ("INC BC", 0),
                0x04 => ("INC B", 0),
                0x05 => ("DEC B", 0),
                0x06 => ("LD B, {$2}", 1),
                0x07 => ("RCLA", 0),
                0x08 => ("LD ({$4}),SP", 2),
                0x09 => ("ADD HL, BC", 0),
                0x0A => ("LD A, (BC)", 0),
                0x0B => ("DEC BC", 0),
                0x0C => ("INC C", 0),
                0x0D => ("DEC C", 0),
                0x0E => ("LD C, {$2}", 1),
                0x0F => ("RRCA", 0),
                0x10 => ("STOP 0", 0),
                0x11 => ("LD DE, {$4}", 2),
                0x12 => ("LD (DE), A", 0),
                0x13 => ("INC DE", 0),
                0x14 => ("INC D", 0),
                0x15 => ("DEC D", 0),
                0x16 => ("LD D, {$2}", 1),
                0x17 => ("RLA", 0),
                0x18 => ("JR {r}", 1),
                0x19 => ("ADD HL, DE", 0),
                0x1A => ("LD A, (DE)", 0),
                0x1B => ("DEC DE", 0),
                0x1C => ("INC E", 0),
                0x1D => ("DEC E", 0),
                0x1E => ("LD E, {$2}", 1),
                0x1F => ("RRA", 0),
                0x20 => ("JR NZ, {r}", 1),
                0x21 => ("LD HL, {$4}", 2),
                0x22 => ("LD (HL+), A", 0),
                0x23 => ("INC HL", 0),
                0x24 => ("INC H", 0),
                0x25 => ("DEC H", 0),
                0x26 => ("LD H, {$2}", 1),
                0x27 => ("DAA", 0),
                0x28 => ("JR Z, {r}", 1),
                0x29 => ("ADD HL, HL", 0),
                0x2A => ("LD A, (HL+)", 0),
                0x2B => ("DEC HL", 0),
                0x2C => ("INC L", 0),
                0x2D => ("DEC L", 0),
                0x2E => ("LD L, {$2}", 1),
                0x2F => ("CPL", 0),
                0x30 => ("JR NC, {r}", 1),
                0x31 => ("LD SP, {$4}", 2),
                0x32 => ("LD (HL-), A", 0),
                0x33 => ("INC SP", 0),
                0x34 => ("INC (HL)", 0),
                0x35 => ("DEC (HL)", 0),
                0x36 => ("LD (HL),{$2}", 1),
                0x37 => ("SCF", 0),
                0x38 => ("JR C, {r}", 1),
                0x39 => ("ADD HL, SP", 0),
                0x3A => ("LD A, (HL-)", 0),
                0x3B => ("DEC SP", 0),
                0x3C => ("INC A", 0),
                0x3D => ("DEC A", 0),
                0x3E => ("LD A, {$2}", 1),
                0x3F => ("CCF", 0),
                0x40 => ("LD B, B", 0),
                0x41 => ("LD B, C", 0),
                0x42 => ("LD B, D", 0),
                0x43 => ("LD B, E", 0),
                0x44 => ("LD B, H", 0),
                0x45 => ("LD B, L", 0),
                0x46 => ("LD B, (HL)", 0),
                0x47 => ("LD B, A", 0),
                0x48 => ("LD C, B", 0),
                0x49 => ("LD C, C", 0),
                0x4A => ("LD C, D", 0),
                0x4B => ("LD C, E", 0),
                0x4C => ("LD C, H", 0),
                0x4D => ("LD C, L", 0),
                0x4E => ("LD C, (HL)", 0),
                0x4F => ("LD C, A", 0),
                0x50 => ("LD D, B", 0),
                0x51 => ("LD D, C", 0),
                0x52 => ("LD D, D", 0),
                0x53 => ("LD D, E", 0),
                0x54 => ("LD D, H", 0),
                0x55 => ("LD D, L", 0),
                0x56 => ("LD D, (HL)", 0),
                0x57 => ("LD D, A", 0),
                0x58 => ("LD E, B", 0),
                0x59 => ("LD E, C", 0),
                0x5A => ("LD E, D", 0),
                0x5B => ("LD E, E", 0),
                0x5C => ("LD E, H", 0),
                0x5D => ("LD E, L", 0),
                0x5E => ("LD E, (HL)", 0),
                0x5F => ("LD E, A", 0),
                0x60 => ("LD H, B", 0),
                0x61 => ("LD H, C", 0),
                0x62 => ("LD H, D", 0),
                0x63 => ("LD H, E", 0),
                0x64 => ("LD H, H", 0),
                0x65 => ("LD H, L", 0),
                0x66 => ("LD H, (HL)", 0),
                0x67 => ("LD H, A", 0),
                0x68 => ("LD L, B", 0),
                0x69 => ("LD L, C", 0),
                0x6A => ("LD L, D", 0),
                0x6B => ("LD L, E", 0),
                0x6C => ("LD L, H", 0),
                0x6D => ("LD L, L", 0),
                0x6E => ("LD L, (HL)", 0),
                0x6F => ("LD L, A", 0),
                0x70 => ("LD (HL), B", 0),
                0x71 => ("LD (HL), C", 0),
                0x72 => ("LD (HL), D", 0),
                0x73 => ("LD (HL), E", 0),
                0x74 => ("LD (HL), H", 0),
                0x75 => ("LD (HL), L", 0),
                0x76 => ("HALT", 0),
                0x77 => ("LD (HL), A", 0),
                0x78 => ("LD A, B", 0),
                0x79 => ("LD A, C", 0),
                0x7A => ("LD A, D", 0),
                0x7B => ("LD A, E", 0),
                0x7C => ("LD A, H", 0),
                0x7D => ("LD A, L", 0),
                0x7E => ("LD A, (HL)", 0),
                0x7F => ("LD A, A", 0),
                0x80 => ("ADD A, B", 0),
                0x81 => ("ADD A, C", 0),
                0x82 => ("ADD A, D", 0),
                0x83 => ("ADD A, E", 0),
                0x84 => ("ADD A, H", 0),
                0x85 => ("ADD A, L", 0),
                0x86 => ("ADD A, (HL)", 0),
                0x87 => ("ADD A, A", 0),
                0x88 => ("ADC A, B", 0),
                0x89 => ("ADC A, C", 0),
                0x8A => ("ADC A, D", 0),
                0x8B => ("ADC A, E", 0),
                0x8C => ("ADC A, H", 0),
                0x8D => ("ADC A, L", 0),
                0x8E => ("ADC A, (HL)", 0),
                0x8F => ("ADC A, A", 0),
                0x90 => ("SUB B", 0),
                0x91 => ("SUB C", 0),
                0x92 => ("SUB D", 0),
                0x93 => ("SUB e", 0),
                0x94 => ("SUB H", 0),
                0x95 => ("SUB L", 0),
                0x97 => ("SUB A", 0),
                0x98 => ("SBC A, B", 0),
                0x99 => ("SBC A, C", 0),
                0x9A => ("SBC A, D", 0),
                0x9B => ("SBC A, E", 0),
                0x9C => ("SBC A, H", 0),
                0x9D => ("SBC A, L", 0),
                0x9E => ("SBC A, (HL)", 0),
                0x9F => ("SBC A, A", 0),
                0xA0 => ("AND B", 0),
                0xA1 => ("AND C", 0),
                0xA2 => ("AND D", 0),
                0xA3 => ("AND E", 0),
                0xA4 => ("AND H", 0),
                0xA5 => ("AND L", 0),
                0xA6 => ("AND (HL)", 0),
                0xA7 => ("AND A", 0),
                0xA8 => ("XOR B", 0),
                0xA9 => ("XOR C", 0),
                0xAA => ("XOR D", 0),
                0xAB => ("XOR E", 0),
                0xAC => ("XOR H", 0),
                0xAD => ("XOR L", 0),
                0xAE => ("XOR (HL)", 0),
                0xAF => ("XOR A", 0),
                0xB0 => ("OR B", 0),
                0xB1 => ("OR C", 0),
                0xB2 => ("OR D", 0),
                0xB3 => ("OR E", 0),
                0xB4 => ("OR H", 0),
                0xB5 => ("OR L", 0),
                0xB6 => ("OR (HL)", 0),
                0xB7 => ("OR A", 0),
                0xB8 => ("CP B", 0),
                0xB9 => ("CP C", 0),
                0xBA => ("CP D", 0),
                0xBB => ("CP E", 0),
                0xBC => ("CP H", 0),
                0xBD => ("CP L", 0),
                0xBE => ("CP (HL)", 0),
                0xBF => ("CP A", 0),
                0xC0 => ("RET NZ", 0),
                0xC1 => ("POP BC", 0),
                0xC2 => ("JP NZ {$4}", 2),
                0xC3 => ("JP {$4}", 2),
                0xC4 => ("CALL NZ, {$4}", 2),
                0xC5 => ("PUSH BC", 0),
                0xC6 => ("ADD A, {$2}", 1),
                0xC7 => ("RST 00H", 0),
                0xC8 => ("RET Z", 0),
                0xC9 => ("RET", 0),
                0xCA => ("JP Z, {$4}", 2),
                0xCC => ("CALL Z, {$4}", 2),
                0xCD => ("CALL {$4}", 2),
                0xCE => ("ADC A, {$2}", 1),
                0xCF => ("RST 08H", 0),
                0xD0 => ("RET NC", 0),
                0xD1 => ("POP DE", 0),
                0xD2 => ("JP NC, {$4}", 2),
                0xD4 => ("CALL NC, {$4}", 2),
                0xD5 => ("PUSH DE", 0),
                0xD6 => ("SUB {$2}", 1),
                0xD7 => ("RST 10H", 0),
                0xD8 => ("RET C", 0),
                0xD9 => ("RETI", 0),
                0xDA => ("JP C, {$4}", 2),
                0xDC => ("CALL C, {$4}", 2),
                0xDE => ("SBC A, {$2}", 1),
                0xDF => ("RST 18H", 0),
                0xE0 => ("LDH ({$2h}), A", 1),
                0xE1 => ("POP HL", 0),
                0xE2 => ("LD (C), A", 0),
                0xE5 => ("PUSH HL", 0),
                0xE6 => ("AND d8", 1),
                0xE7 => ("RST 20H", 0),
                0xE8 => ("ADD SP, {r}", 1),
                0xE9 => ("JP (HL)", 0),
                0xEA => ("LD ({$4}), A", 2),
                0xEE => ("XOR {$2}", 1),
                0xEF => ("RST 28H", 0),
                0xF0 => ("LDH A, ({$2h})", 1),
                0xF1 => ("POP AF", 0),
                0xF2 => ("LD A, (C)", 0),
                0xF3 => ("DI", 0),
                0xF5 => ("PUSH AF", 0),
                0xF6 => ("OR {$2}", 1),
                0xF7 => ("RST 30H", 0),
                0xF8 => ("LD HL SP+{r}", 1),
                0xF9 => ("LD SP, HL", 0),
                0xFA => ("LD A, ({$4})", 2),
                0xFB => ("EI", 0),
                0xFE => ("CP {$2}", 1),
                0xFF => ("RST 38H", 0),
                _ => return None,
            };
            return Some((format!("{}", dis), size));
        }
        _ => return None,
    }
}

#[test]
fn test_op_disassemble() {
    let ff = disassemble_code(0x00FF).unwrap();
    assert_eq!("RST 38H".to_owned(), ff.0);
}

#[test]
fn test_cb_disassemble() {
    let ff = disassemble_code(0xCBFF).unwrap();
    assert_eq!("SET 7, A".to_owned(), ff.0);
}
