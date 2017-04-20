use std::fmt;
use rom::Rom;

pub struct Disassembler {
    rom: Option<Rom>,
    pc: usize,
}
impl Disassembler {
    pub fn new() -> Self {
        Disassembler {
            rom: None,
            pc: 0,
        }
    }
    pub fn load_rom(&mut self, rom: Rom) {
        self.rom = Some(rom);
    }
    pub fn load_path(&mut self, path: &str) {
        self.rom = Some(Rom::new(path));
    }
    pub fn disassemble(&mut self) -> Disassembly {
        let rom = match self.rom {
            Some(ref rom) => rom,
            _ => panic!("NO ROM")
        };
        let mut instructions: Vec<Instruction> = Vec::new();

        while self.pc < rom.size() {
            let cur_pc = self.pc;
            let mut prefix = None;
            let code = match rom.read(self.pc) {
                0xCB => {prefix = Some(0xCB); rom.read(self.pc + 1)},
                normal => normal,
            };
            let (dis, size) = match prefix {
                Some(_) => disassemble_cb(code),
                None => disassemble_00(code),
            };
            let value: Option<u16> = match (prefix, size > 1) {
                (None, true) => {
                    let mut val1 = rom.read(self.pc + 1) as u8;
                    let mut val2 = rom.read(self.pc + 2) as u8;
                    if size > 2 {
                        value |= (rom.read(self.pc + 2) as u16) << 8;
                    }
                    Some(value)
                }, (_, _) => None
            };
            match dis {
                Some(dis) => {
                    let ins = Instruction::new(cur_pc, code, prefix, dis, value);
                    println!("{:04X}:{:?}", self.pc, ins);
                    instructions.push(ins);
                },
                None => {}
            }
            self.pc += size as usize;
        }
        Disassembly{instructions: instructions}
    }
}
pub struct Disassembly {
    instructions: Vec<Instruction>,
}
impl Disassembly {
    pub fn print(&self) {
        for ins in &self.instructions {
            println!("PC:{:04X} | {:?}", ins.pc, ins);
        }
    }
}
struct Instruction {
    code: u8,
    prefix: Option<u8>,
    value1: Option<u8>,
    value2: Option<u8>,
    dis: String,
    pc: usize,
}
impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let code = match self.prefix {
            Some(cb) => (cb as u16) << 8 | self.code as u16,
            None => self.code as u16,
        };

        let val1 = match self.value1 { Some(v) => format!("{:2X}", v), None =>  "  ".to_owned()};
        let val2 = match self.value2 { Some(v) => format!("{:2X}", v), None =>  "  ".to_owned()};
        write!(f, "{:04X} {} {}\t{}", code, val1, val2, self.dis)
    }
}

impl Instruction {
    pub fn new(pc: usize, code: u8, prefix: Option<u8>, dis: String, value: Option<u16>) -> Self {
        let dissed = match value {
            Some(v) => {
                dis.replace("{r}", &format!("{}", &v))
                   .replace("{$2}", &format!("${:02x}", &v))
                   .replace("{$4}", &format!("${:04x}", &v))
                   .replace("{$2h}", &format!("$ff00+{:02x}", &v))
            },
            None => dis,
        };
        Instruction {
            pc: pc,
            code: code,
            dis: dissed,
            value1: value1,
            value2: value2,
            prefix: prefix,
        }
    }
}
enum ValueType {
    A16,
    A8Hi,
    D8,
    D16,
    R8,
    None
}
struct Inst {
    code: u16,
    values: Vec<u8>,
    dis: String,
    value_type: 
}
impl Inst {
    pub fn new(values: &[u8])
}
pub fn disassemble_00(code: u8) -> (Option<String>, u8){
    let (dis, size) = match code {
        0x00 => ("NOP",             1),
        0x01 => ("LD BC, {$4}",     3),
        0x02 => ("LD (BC), A",      1),
        0x03 => ("INC BC",          1),
        0x04 => ("INC B",           1),
        0x05 => ("DEC B",           1),
        0x06 => ("LD B, {$2}",      2),
        0x07 => ("RCLA",            1),
        0x08 => ("LD ({$4}),SP",    3),
        0x09 => ("ADD HL, BC",      1),
        0x0A => ("LD A, (BC)",      1),
        0x0B => ("DEC BC",          1),
        0x0C => ("INC C",           1),
        0x0D => ("DEC C",           1),
        0x0E => ("LD C, {$2}",      2),
        0x0F => ("RRCA",            1),
        0x10 => ("STOP 0",          1),
        0x11 => ("LD DE, {$4}",     3),
        0x12 => ("LD (DE), A",      1),
        0x13 => ("INC DE",          1),
        0x14 => ("INC D",           1),
        0x15 => ("DEC D",           1),
        0x16 => ("LD D, {$2}",      2),
        0x17 => ("RLA",             1),
        0x18 => ("JR {r}",          2),
        0x19 => ("ADD HL, DE",      1),
        0x1A => ("LD A, (DE)",      1),
        0x1B => ("DEC DE",          1),
        0x1C => ("INC E",           1),
        0x1D => ("DEC E",           1),
        0x1E => ("LD E, {$2}",      2),
        0x1F => ("RRA",             1),
        0x20 => ("JR NZ, {r}" ,     2),
        0x21 => ("LD HL, {$4}",     3),
        0x22 => ("LD (HL+), A",     1),
        0x23 => ("INC HL",          1),
        0x24 => ("INC H",           1),
        0x25 => ("DEC H",           1),
        0x26 => ("LD H, {$2}",      2),
        0x27 => ("DAA",             1),
        0x28 => ("JR Z, {r}",       2),
        0x29 => ("ADD HL, HL",      1),
        0x2A => ("LD A, (HL+)",     1),
        0x2B => ("DEC HL",          1),
        0x2C => ("INC L",           1),
        0x2D => ("DEC L",           1),
        0x2E => ("LD L, {$2}",      2),
        0x2F => ("CPL",             1),
        0x30 => ("JR NC, {r}",      2),
        0x31 => ("LD SP, {$4}",     3),
        0x32 => ("LD (HL-), A",     1),
        0x33 => ("INC SP",          1),
        0x34 => ("INC (HL)",        1),
        0x35 => ("DEC (HL)",        1),
        0x36 => ("LD (HL),{$2}",    2),
        0x37 => ("SCF",             1),
        0x38 => ("JR C, {r}",       2),
        0x39 => ("ADD HL, SP",      1),
        0x3A => ("LD A, (HL-)",     1),
        0x3B => ("DEC SP",          1),
        0x3C => ("INC A",           1),
        0x3D => ("DEC A",           1),
        0x3E => ("LD A, {$2}",      2),
        0x3F => ("CCF",             1),
        0x40 => ("LD B, B",         1),
        0x41 => ("LD B, C",         1),
        0x42 => ("LD B, D",         1),
        0x43 => ("LD B, E",         1),
        0x44 => ("LD B, H",         1),
        0x45 => ("LD B, L",         1),
        0x46 => ("LD B, (HL)",      1),
        0x47 => ("LD B, A",         1),
        0x48 => ("LD C, B",         1),
        0x49 => ("LD C, C",         1),
        0x4A => ("LD C, D",         1),
        0x4B => ("LD C, E",         1),
        0x4C => ("LD C, H",         1),
        0x4D => ("LD C, L",         1),
        0x4E => ("LD C, (HL)",      1),
        0x4F => ("LD C, A",         1),
        0x50 => ("LD D, B",         1),
        0x51 => ("LD D, C",         1),
        0x52 => ("LD D, D",         1),
        0x53 => ("LD D, E",         1),
        0x54 => ("LD D, H",         1),
        0x55 => ("LD D, L",         1),
        0x56 => ("LD D, (HL)",      1),
        0x57 => ("LD D, A",         1),
        0x58 => ("LD E, B",         1),
        0x59 => ("LD E, C",         1),
        0x5A => ("LD E, D",         1),
        0x5B => ("LD E, E",         1),
        0x5C => ("LD E, H",         1),
        0x5D => ("LD E, L",         1),
        0x5E => ("LD E, (HL)",      1),
        0x5F => ("LD E, A",         1),
        0x60 => ("LD H, B",         1),
        0x61 => ("LD H, C",         1),
        0x62 => ("LD H, D",         1),
        0x63 => ("LD H, E",         1),
        0x64 => ("LD H, H",         1),
        0x65 => ("LD H, L",         1),
        0x66 => ("LD H, (HL)",      1),
        0x67 => ("LD H, A",         1),
        0x68 => ("LD L, B",         1),
        0x69 => ("LD L, C",         1),
        0x6A => ("LD L, D",         1),
        0x6B => ("LD L, E",         1),
        0x6C => ("LD L, H",         1),
        0x6D => ("LD L, L",         1),
        0x6E => ("LD L, (HL)",      1),
        0x6F => ("LD L, A",         1),
        0x70 => ("LD (HL), B",      1),
        0x71 => ("LD (HL), C",      1),
        0x72 => ("LD (HL), D",      1),
        0x73 => ("LD (HL), E",      1),
        0x74 => ("LD (HL), H",      1),
        0x75 => ("LD (HL), L",      1),
        0x76 => ("HALT",            1),
        0x77 => ("LD (HL), A",      1),
        0x78 => ("LD A, B",         1),
        0x79 => ("LD A, C",         1),
        0x7A => ("LD A, D",         1),
        0x7B => ("LD A, E",         1),
        0x7C => ("LD A, H",         1),
        0x7D => ("LD A, L",         1),
        0x7E => ("LD A, (HL)",      1),
        0x7F => ("LD A, A",         1),
        0x80 => ("ADD A, B",        1),
        0x81 => ("ADD A, C",        1),
        0x82 => ("ADD A, D",        1),
        0x83 => ("ADD A, E",        1),
        0x84 => ("ADD A, H",        1),
        0x85 => ("ADD A, L",        1),
        0x86 => ("ADD A, (HL)",     1),
        0x87 => ("ADD A, A",        1),
        0x88 => ("ADC A, B",        1),
        0x89 => ("ADC A, C",        1),
        0x8A => ("ADC A, D",        1),
        0x8B => ("ADC A, E",        1),
        0x8C => ("ADC A, H",        1),
        0x8D => ("ADC A, L",        1),
        0x8E => ("ADC A, (HL)",     1),
        0x8F => ("ADC A, A",        1),
        0x90 => ("SUB B",           1),
        0x91 => ("SUB C",           1),
        0x92 => ("SUB D",           1),
        0x93 => ("SUB e",           1),
        0x94 => ("SUB H",           1),
        0x95 => ("SUB L",           1),
        0x97 => ("SUB A",           1),
        0x98 => ("SBC A, B",        1),
        0x99 => ("SBC A, C",        1),
        0x9A => ("SBC A, D",        1),
        0x9B => ("SBC A, E",        1),
        0x9C => ("SBC A, H",        1),
        0x9D => ("SBC A, L",        1),
        0x9E => ("SBC A, (HL)",     1),
        0x9F => ("SBC A, A",        1),
        0xA0 => ("AND B",           1),
        0xA1 => ("AND C",           1),
        0xA2 => ("AND D",           1),
        0xA3 => ("AND E",           1),
        0xA4 => ("AND H",           1),
        0xA5 => ("AND L",           1),
        0xA6 => ("AND (HL)",        1),
        0xA7 => ("AND A",           1),
        0xA8 => ("XOR B",           1),
        0xA9 => ("XOR C",           1),
        0xAA => ("XOR D",           1),
        0xAB => ("XOR E",           1),
        0xAC => ("XOR H",           1),
        0xAD => ("XOR L",           1),
        0xAE => ("XOR (HL)",        1),
        0xAF => ("XOR A",           1),
        0xB0 => ("OR B",            1),
        0xB1 => ("OR C",            1),
        0xB2 => ("OR D",            1),
        0xB3 => ("OR E",            1),
        0xB4 => ("OR H",            1),
        0xB5 => ("OR L",            1),
        0xB6 => ("OR (HL)",         1),
        0xB7 => ("OR A",            1),
        0xB8 => ("CP B",            1),
        0xB9 => ("CP C",            1),
        0xBA => ("CP D",            1),
        0xBB => ("CP E",            1),
        0xBC => ("CP H",            1),
        0xBD => ("CP L",            1),
        0xBE => ("CP (HL)",         1),
        0xBF => ("CP A",            1),
        0xC0 => ("RET NZ",          1),
        0xC1 => ("POP BC",          1),
        0xC2 => ("JP NZ {$4}",      3),
        0xC3 => ("JP {$4}",         3),
        0xC4 => ("CALL NZ, {$4}",   3),
        0xC5 => ("PUSH BC",         1),
        0xC6 => ("ADD A, {$2}",     2),
        0xC7 => ("RST 00H",         1),
        0xC8 => ("RET Z",           1),
        0xC9 => ("RET",             1),
        0xCA => ("JP Z, {$4}",      3),
        0xCC => ("CALL Z, {$4}",    3),
        0xCD => ("CALL {$4}",       3),
        0xCE => ("ADC A, {$2}",     2),
        0xCF => ("RST 08H",         1),
        0xD0 => ("RET NC",          1),
        0xD1 => ("POP DE",          1),
        0xD2 => ("JP NC, {$4}",     3),
        0xD4 => ("CALL NC, {$4}",   3),
        0xD5 => ("PUSH DE",         1),
        0xD6 => ("SUB {$2}",        2),
        0xD7 => ("RST 10H",         1),
        0xD8 => ("RET C",           1),
        0xD9 => ("RETI",            1),
        0xDA => ("JP C, {$4}",      3),
        0xDC => ("CALL C, {$4}",    3),
        0xDE => ("SBC A, {$2}",     2),
        0xDF => ("RST 18H",         1),
        0xE0 => ("LDH ({$2h}), A",  2),
        0xE1 => ("POP HL",          1),
        0xE2 => ("LD (C), A",       1),
        0xE5 => ("PUSH HL",         1),
        0xE6 => ("AND d8",          2),
        0xE7 => ("RST 20H",         1),
        0xE8 => ("ADD SP, {r}",     2),
        0xE9 => ("JP (HL)",         1),
        0xEA => ("LD ({$4}), A",    3),
        0xEE => ("XOR {$2}",        2),
        0xEF => ("RST 28H",         1),
        0xF0 => ("LDH A, ({$2h}})", 2),
        0xF1 => ("POP AF",          1),
        0xF2 => ("LD A, (C)",       1),
        0xF3 => ("DI",              1),
        0xF5 => ("PUSH AF",         1),
        0xF6 => ("OR {$2}",         2),
        0xF7 => ("RST 30H",         1),
        0xF8 => ("LD HL SP+{r}",    2),
        0xF9 => ("LD SP, HL",       1),
        0xFA => ("LD A, ({$4})",    3),
        0xFB => ("EI",              1),
        0xFE => ("CP {$2}",         2),
        0xFF => ("RST 38H",         1),
        _    => return (None, 1)
    };
    return (Some(format!("{}", dis)), size)
}

fn disassemble_cb(code: u8) -> (Option<String>, u8){
    let instr = match code {
        0x00...0x07 => "RLC",
        0x08...0x0F => "RRC",
        0x10...0x17 => "RL",
        0x18...0x1F => "RR",
        0x20...0x27 => "SLA",
        0x28...0x2F => "SRA",
        0x30...0x37 => "SWAP",
        0x38...0x3F => "SRL",
        0x40...0x7F => "BIT",
        0x80...0xBF => "RES",
        0xC0...0xFF => "SET",
        _ => panic!("Nope")
    };
    let operand = match code & 0x0F {
        0x00 | 0x08 => "B",
        0x01 | 0x09 => "C",
        0x02 | 0x0A => "D",
        0x03 | 0x0B => "E",
        0x04 | 0x0C => "H",
        0x05 | 0x0D => "L",
        0x06 | 0x0E => "(HL)",
        0x07 | 0x0F => "A",
        _ => panic!("Nope")
    };
    let operator = match instr {
        "RES" | "BIT" | "SET" => {format!("{} {}", instr, (code % 0x40) / 8)}
        _ => instr.to_string(),
    };
    (Some(format!("{}, {}", operator, operand)), 2)
}

#[test]
fn test_op_disassemble() {
    let ff = disassemble_00(0xFF).unwrap();
    assert_eq!("RST 38H".to_owned(), ff.0);
}

#[test]
fn test_cb_disassemble() {
    let ff = disassemble_cb(0xFF).unwrap();
    assert_eq!("SET 7, A".to_owned(), ff.0);
}
