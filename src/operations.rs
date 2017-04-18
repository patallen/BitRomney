use std::fmt;

use cpu::Cpu;
use mmu::Mmu;
use registers::FlagRegister;

use bitty::BitFlags;

pub struct Operation {
    pub dis: &'static str,
    pub func: Box<Fn(&mut Cpu, &mut Mmu)>,
    pub cycles: u8,
    pub mode: ValueMode,
}


impl Operation {
    pub fn new(func: fn(&mut Cpu, &mut Mmu),
               cycles: u8,
               dis: &'static str,
               mode: ValueMode
    ) -> Operation {
        Operation {
            func: Box::new(func),
            cycles: cycles,
            dis: dis,
            mode: mode,
        }
    }
    pub fn disassemble(&self, cpu: &Cpu, mmu: &Mmu) -> String {
        let val = match self.mode {
            ValueMode::A8    => Some(format!("${:02X}", cpu.immediate_u8(mmu) as u16)),
            ValueMode::A8Hi  => Some(format!("${:04X}", (cpu.immediate_u8(mmu) as u16) + 0xFF00)),
            ValueMode::A16   => Some(format!("${:04X}", cpu.immediate_u16(mmu))),
            ValueMode::D8    => Some(format!("${:02X}", cpu.immediate_u8(mmu) as u16)),
            ValueMode::D16   => Some(format!("${:04X}", cpu.immediate_u16(mmu))),
            ValueMode::R8    => Some(format!("{}", cpu.immediate_u8(mmu) as i8)),
            ValueMode::None  => None
        };
        match val {
            Some(v) => self.dis.replace("{}", &v),
            None => self.dis.to_string(),
        }
    }
}


pub enum ValueMode {
    A8,
    A8Hi,
    A16,
    D8,
    D16,
    R8,
    None
}

pub fn get_operation(code: u16) -> Operation {
    let prefix = code >> 8;
    let scode = code & 0x00F0;
    let lcode = code & 0x000F;

    match prefix {
        0x00 => match scode {   // No Prefix
            0x00 => match lcode {
                0x00 => Operation::new(opx00,  4, "NOP",         ValueMode::None),
                0x01 => Operation::new(opx01, 12, "LD BC, d16",  ValueMode::D16),
                0x02 => Operation::new(panic,  8, "LD (BC), A",  ValueMode::None),
                0x03 => Operation::new(opx03,  8, "INC BC",      ValueMode::None),
                0x04 => Operation::new(opx04,  4, "INC B",       ValueMode::None),
                0x05 => Operation::new(opx05,  4, "DEC B",       ValueMode::None),
                0x06 => Operation::new(opx06,  8, "LD B, {}",    ValueMode::D8),
                0x07 => Operation::new(panic,  4, "RCLA",        ValueMode::None),
                0x08 => Operation::new(opx08, 20, "LD ({}), SP", ValueMode::A16),
                0x09 => Operation::new(opx09,  8, "ADD HL, BC",  ValueMode::None),
                0x0A => Operation::new(panic,  8, "LD A, (BC)",  ValueMode::None),
                0x0B => Operation::new(opx0B,  8, "DEC BC",      ValueMode::None),
                0x0C => Operation::new(opx0C,  4, "INC C",       ValueMode::None),
                0x0D => Operation::new(opx0D,  4, "DEC C",       ValueMode::None),
                0x0E => Operation::new(opx0E,  8, "LD C, {}",    ValueMode::D8),
                0x0F => Operation::new(opx0F,  4, "RRCA",        ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0x10 => match lcode {
                0x00 => Operation::new(opx00,  2, "STOP 0",      ValueMode::None),
                0x01 => Operation::new(opx11, 12, "LD DE, {}",   ValueMode::D16),
                0x02 => Operation::new(opx12,  8, "LD (DE), A",  ValueMode::None),
                0x03 => Operation::new(opx13,  8, "INC DE",      ValueMode::None),
                0x04 => Operation::new(opx14,  4, "INC D",       ValueMode::None),
                0x05 => Operation::new(opx15,  4, "DEC D",       ValueMode::None),
                0x06 => Operation::new(opx16,  8, "LD D, {}",    ValueMode::D8),
                0x07 => Operation::new(opx17,  4, "RLA",         ValueMode::None),
                0x08 => Operation::new(opx18,  8, "JR {}",       ValueMode::R8),
                0x09 => Operation::new(opx19,  8, "ADD HL, DE",  ValueMode::None),
                0x0A => Operation::new(opx1A,  8, "LD A, (DE)",  ValueMode::None),
                0x0B => Operation::new(panic,  8, "DEC DE",      ValueMode::None),
                0x0C => Operation::new(opx1C,  4, "INC E",       ValueMode::None),
                0x0D => Operation::new(opx1D,  4, "DEC E",       ValueMode::None),
                0x0E => Operation::new(opx1E,  8, "LD E, {}",    ValueMode::D8),
                0x0F => Operation::new(panic,  4, "RRA",         ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0x20 => match lcode {
                0x00 => Operation::new(opx20, 12, "JR NZ, {}",   ValueMode::R8),
                0x01 => Operation::new(opx21, 12, "LD HL, {}",   ValueMode::D16),
                0x02 => Operation::new(opx22, 12, "LD (HL+), A", ValueMode::None),
                0x03 => Operation::new(opx23,  8, "INC HL",      ValueMode::None),
                0x04 => Operation::new(opx24,  4, "INC H",       ValueMode::None),
                0x05 => Operation::new(opx25,  4, "DEC H",       ValueMode::None),
                0x06 => Operation::new(opx26,  8, "LD H, {}",    ValueMode::D8),
                0x07 => Operation::new(panic,  4, "DAA",         ValueMode::None),
                0x08 => Operation::new(opx28, 12, "JR Z, {}",    ValueMode::R8),
                0x09 => Operation::new(opx29,  8, "ADD HL, HL",  ValueMode::None),
                0x0A => Operation::new(opx2A,  8, "LD A, (HL+)", ValueMode::None),
                0x0B => Operation::new(panic,  8, "DEC HL",      ValueMode::None),
                0x0C => Operation::new(opx2C,  4, "INC L",       ValueMode::None),
                0x0D => Operation::new(opx2D,  4, "DEC L",       ValueMode::None),
                0x0E => Operation::new(opx2E,  8, "LD L, {}",    ValueMode::D8),
                0x0F => Operation::new(opx2F,  4, "CPL",         ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0x30 => match lcode {
                0x00 => Operation::new(panic, 12, "JR NC, r8",   ValueMode::R8),
                0x01 => Operation::new(opx31, 12, "LD SP, {}",   ValueMode::D16),
                0x02 => Operation::new(opx32,  8, "LD (HL-), A", ValueMode::None),
                0x03 => Operation::new(opx33,  8, "INC SP",      ValueMode::None),
                0x04 => Operation::new(panic, 12, "INC (HL)",    ValueMode::None),
                0x05 => Operation::new(opx35, 12, "DEC (HL)",    ValueMode::None),
                0x06 => Operation::new(opx36, 12, "LD (HL), d8", ValueMode::D8),
                0x07 => Operation::new(panic,  4, "SCF",         ValueMode::None),
                0x08 => Operation::new(panic, 12, "JR C, r8",    ValueMode::R8),
                0x09 => Operation::new(opx39,  8, "ADD HL, SP",  ValueMode::None),
                0x0A => Operation::new(opx3A,  8, "LD A, (HL-)", ValueMode::None),
                0x0B => Operation::new(panic,  8, "DEC SP",      ValueMode::None),
                0x0C => Operation::new(opx3C,  4, "INC A",       ValueMode::None),
                0x0D => Operation::new(opx3D,  4, "DEC A",       ValueMode::None),
                0x0E => Operation::new(opx3E,  8, "LD A, {}",    ValueMode::D8),
                0x0F => Operation::new(panic,  4, "CCF",         ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0x40 => match lcode {
                0x00 => Operation::new(opx40,  4, "LD B, B",     ValueMode::None),
                0x01 => Operation::new(opx41,  4, "LD B, C",     ValueMode::None),
                0x02 => Operation::new(opx42,  4, "LD B, D",     ValueMode::None),
                0x03 => Operation::new(opx43,  4, "LD B, E",     ValueMode::None),
                0x04 => Operation::new(opx44,  4, "LD B, H",     ValueMode::None),
                0x05 => Operation::new(opx45,  4, "LD B, L",     ValueMode::None),
                0x06 => Operation::new(opx46,  8, "LD B, (HL)",  ValueMode::None),
                0x07 => Operation::new(opx47,  4, "LD B, A",     ValueMode::None),
                0x08 => Operation::new(opx48,  4, "LD C, B",     ValueMode::None),
                0x09 => Operation::new(opx49,  4, "LD C, C",     ValueMode::None),
                0x0A => Operation::new(opx4A,  4, "LD C, D",     ValueMode::None),
                0x0B => Operation::new(opx4B,  4, "LD C, E",     ValueMode::None),
                0x0C => Operation::new(opx4C,  4, "LD C, H",     ValueMode::None),
                0x0D => Operation::new(opx4D,  4, "LD C, L",     ValueMode::None),
                0x0E => Operation::new(opx4E,  8, "LD C, (HL)",  ValueMode::None),
                0x0F => Operation::new(opx4F,  4, "LD C, A",     ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0x50 => match lcode {
                0x00 => Operation::new(opx50,  4, "LD D, B",     ValueMode::None),
                0x01 => Operation::new(opx51,  4, "LD D, C",     ValueMode::None),
                0x02 => Operation::new(opx52,  4, "LD D, D",     ValueMode::None),
                0x03 => Operation::new(opx53,  4, "LD D, E",     ValueMode::None),
                0x04 => Operation::new(opx54,  4, "LD D, H",     ValueMode::None),
                0x05 => Operation::new(opx55,  4, "LD D, L",     ValueMode::None),
                0x06 => Operation::new(opx56,  8, "LD D, (HL)",  ValueMode::None),
                0x07 => Operation::new(opx57,  4, "LD D, A",     ValueMode::None),
                0x08 => Operation::new(opx58,  4, "LD E, B",     ValueMode::None),
                0x09 => Operation::new(opx59,  4, "LD E, C",     ValueMode::None),
                0x0A => Operation::new(opx5A,  4, "LD E, D",     ValueMode::None),
                0x0B => Operation::new(opx5B,  4, "LD E, E",     ValueMode::None),
                0x0C => Operation::new(opx5C,  4, "LD E, H",     ValueMode::None),
                0x0D => Operation::new(opx5D,  4, "LD E, L",     ValueMode::None),
                0x0E => Operation::new(opx5E,  8, "LD E, (HL)",  ValueMode::None),
                0x0F => Operation::new(opx5F,  4, "LD E, A",     ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0x60 => match lcode {
                0x00 => Operation::new(opx60,  4, "LD H, B",     ValueMode::None),
                0x01 => Operation::new(opx61,  4, "LD H, C",     ValueMode::None),
                0x02 => Operation::new(opx62,  4, "LD H, D",     ValueMode::None),
                0x03 => Operation::new(opx63,  4, "LD H, E",     ValueMode::None),
                0x04 => Operation::new(opx64,  4, "LD H, H",     ValueMode::None),
                0x05 => Operation::new(opx65,  4, "LD H, L",     ValueMode::None),
                0x06 => Operation::new(opx66,  8, "LD H, (HL)",  ValueMode::None),
                0x07 => Operation::new(opx67,  4, "LD H, A",     ValueMode::None),
                0x08 => Operation::new(opx68,  4, "LD L, B",     ValueMode::None),
                0x09 => Operation::new(opx69,  4, "LD L, C",     ValueMode::None),
                0x0A => Operation::new(opx6A,  4, "LD L, D",     ValueMode::None),
                0x0B => Operation::new(opx6B,  4, "LD L, E",     ValueMode::None),
                0x0C => Operation::new(opx6C,  4, "LD L, H",     ValueMode::None),
                0x0D => Operation::new(opx6D,  4, "LD L, L",     ValueMode::None),
                0x0E => Operation::new(opx6E,  4, "LD L, (HL)",  ValueMode::None),
                0x0F => Operation::new(opx6F,  4, "LD L, A",     ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0x70 => match lcode {
                0x00 => Operation::new(panic,  8, "LD (HL), B",  ValueMode::None),
                0x01 => Operation::new(panic,  8, "LD (HL), C",  ValueMode::None),
                0x02 => Operation::new(panic,  8, "LD (HL), D",  ValueMode::None),
                0x03 => Operation::new(panic,  8, "LD (HL), E",  ValueMode::None),
                0x04 => Operation::new(panic,  8, "LD (HL), H",  ValueMode::None),
                0x05 => Operation::new(panic,  8, "LD (HL), L",  ValueMode::None),
                0x06 => Operation::new(panic,  4, "HALT",        ValueMode::None),
                0x07 => Operation::new(opx77,  8, "LD (HL), A",  ValueMode::None),
                0x08 => Operation::new(opx78,  4, "LD A, B",     ValueMode::None),
                0x09 => Operation::new(opx79,  4, "LD A, C",     ValueMode::None),
                0x0A => Operation::new(opx7A,  4, "LD A, D",     ValueMode::None),
                0x0B => Operation::new(opx7B,  4, "LD A, E",     ValueMode::None),
                0x0C => Operation::new(opx7C,  4, "LD A, H",     ValueMode::None),
                0x0D => Operation::new(opx7D,  4, "LD A, L",     ValueMode::None),
                0x0E => Operation::new(opx7E,  8, "LD A, (HL)",  ValueMode::None),
                0x0F => Operation::new(opx7F,  4, "LD A, A",     ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0x80 => match lcode {
                0x00 => Operation::new(opx80,  4, "ADD A, B",    ValueMode::None),
                0x01 => Operation::new(opx81,  4, "ADD A, C",    ValueMode::None),
                0x02 => Operation::new(opx82,  4, "ADD A, D",    ValueMode::None),
                0x03 => Operation::new(opx83,  4, "ADD A, E",    ValueMode::None),
                0x04 => Operation::new(opx84,  4, "ADD A, H",    ValueMode::None),
                0x05 => Operation::new(opx85,  4, "ADD A, L",    ValueMode::None),
                0x06 => Operation::new(opx86,  8, "ADD A, (HL)", ValueMode::None),
                0x07 => Operation::new(opx87,  4, "ADD A, A",    ValueMode::None),
                0x08 => Operation::new(panic,  4, "ADC A, B",    ValueMode::None),
                0x09 => Operation::new(panic,  4, "ADC A, C",    ValueMode::None),
                0x0A => Operation::new(panic,  4, "ADC A, D",    ValueMode::None),
                0x0B => Operation::new(panic,  4, "ADC A, E",    ValueMode::None),
                0x0C => Operation::new(panic,  4, "ADC A, H",    ValueMode::None),
                0x0D => Operation::new(panic,  4, "ADC A, L",    ValueMode::None),
                0x0E => Operation::new(panic,  8, "ADC A, (HL)", ValueMode::None),
                0x0F => Operation::new(panic,  4, "ADC A, A",    ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0x90 => match lcode {
                0x00 => Operation::new(opx90,  4, "SUB B",       ValueMode::None),
                0x01 => Operation::new(opx91,  4, "SUB C",       ValueMode::None),
                0x02 => Operation::new(opx92,  4, "SUB D",       ValueMode::None),
                0x03 => Operation::new(opx93,  4, "SUB e",       ValueMode::None),
                0x04 => Operation::new(opx94,  4, "SUB H",       ValueMode::None),
                0x05 => Operation::new(opx95,  4, "SUB L",       ValueMode::None),
                0x07 => Operation::new(opx97,  4, "SUB A",       ValueMode::None),
                0x08 => Operation::new(panic,  4, "SBC A, B",    ValueMode::None),
                0x09 => Operation::new(panic,  4, "SBC A, C",    ValueMode::None),
                0x0A => Operation::new(panic,  4, "SBC A, D",    ValueMode::None),
                0x0B => Operation::new(panic,  4, "SBC A, E",    ValueMode::None),
                0x0C => Operation::new(panic,  4, "SBC A, H",    ValueMode::None),
                0x0D => Operation::new(panic,  4, "SBC A, L",    ValueMode::None),
                0x0E => Operation::new(panic,  8, "SBC A, (HL)", ValueMode::None),
                0x0F => Operation::new(panic,  4, "SBC A, A",    ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0xA0 => match lcode {
                0x00 => Operation::new(opxA0,  4, "AND B",       ValueMode::None),
                0x01 => Operation::new(opxA1,  4, "AND C",       ValueMode::None),
                0x02 => Operation::new(opxA2,  4, "AND D",       ValueMode::None),
                0x03 => Operation::new(opxA3,  4, "AND E",       ValueMode::None),
                0x04 => Operation::new(opxA4,  4, "AND H",       ValueMode::None),
                0x05 => Operation::new(opxA5,  4, "AND L",       ValueMode::None),
                0x06 => Operation::new(opxA6,  8, "AND (HL)",    ValueMode::None),
                0x07 => Operation::new(opxA7,  4, "AND A",       ValueMode::None),
                0x08 => Operation::new(opxA8,  4, "XOR B",       ValueMode::None),
                0x09 => Operation::new(opxA9,  4, "XOR C",       ValueMode::None),
                0x0A => Operation::new(opxAA,  4, "XOR D",       ValueMode::None),
                0x0B => Operation::new(opxAB,  4, "XOR E",       ValueMode::None),
                0x0C => Operation::new(opxAC,  4, "XOR H",       ValueMode::None),
                0x0D => Operation::new(opxAD,  4, "XOR L",       ValueMode::None),
                0x0E => Operation::new(opxAE,  8, "XOR (HL)",    ValueMode::None),
                0x0F => Operation::new(opxAF,  4, "XOR A",       ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0xB0 => match lcode {
                0x00 => Operation::new(opxB0,  4, "OR B",        ValueMode::None),
                0x01 => Operation::new(opxB1,  4, "OR C",        ValueMode::None),
                0x02 => Operation::new(opxB2,  4, "OR D",        ValueMode::None),
                0x03 => Operation::new(opxB3,  4, "OR E",        ValueMode::None),
                0x04 => Operation::new(opxB4,  4, "OR H",        ValueMode::None),
                0x05 => Operation::new(opxB5,  4, "OR L",        ValueMode::None),
                0x06 => Operation::new(opxB6,  8, "OR (HL)",     ValueMode::None),
                0x07 => Operation::new(opxB7,  4, "OR A",        ValueMode::None),
                0x08 => Operation::new(opxB8,  4, "CP B",        ValueMode::None),
                0x09 => Operation::new(opxB9,  4, "CP C",        ValueMode::None),
                0x0A => Operation::new(opxBA,  4, "CP D",        ValueMode::None),
                0x0B => Operation::new(opxBB,  4, "CP E",        ValueMode::None),
                0x0C => Operation::new(opxBC,  4, "CP H",        ValueMode::None),
                0x0D => Operation::new(opxBD,  4, "CP L",        ValueMode::None),
                0x0E => Operation::new(opxBE,  8, "CP (HL)",     ValueMode::None),
                0x0F => Operation::new(opxBF,  4, "CP A",        ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0xC0 => match lcode {
                0x00 => Operation::new(opxC0, 24, "RET NZ",      ValueMode::None),
                0x01 => Operation::new(opxC1, 12, "POP BC",      ValueMode::None),
                0x02 => Operation::new(panic, 16, "JP NZ {}",    ValueMode::A16),
                0x03 => Operation::new(opxC3, 12, "JP {}",       ValueMode::A16),
                0x04 => Operation::new(opxC4, 24, "CALL NZ, {}", ValueMode::A16),
                0x05 => Operation::new(opxC5, 16, "PUSH BC",     ValueMode::None),
                0x06 => Operation::new(opxC6,  8, "ADD A, {}",   ValueMode::D8),
                0x07 => Operation::new(panic, 16, "RST 00H",     ValueMode::D8),
                0x08 => Operation::new(opxC8, 20, "RET Z",       ValueMode::None),
                0x09 => Operation::new(opxC9, 16, "RET",         ValueMode::None),
                0x0A => Operation::new(opxCA, 16, "JP Z, {}",    ValueMode::A16),
                0x0C => Operation::new(opxCC, 24, "CALL Z, {}",  ValueMode::A16),
                0x0D => Operation::new(opxCD, 24, "CALL {}",     ValueMode::A16),
                0x0E => Operation::new(opxCE,  8, "ADC A, {}",   ValueMode::D8),
                0x0F => Operation::new(opxCF, 16, "RST 08H",     ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0xD0 => match lcode {
                0x00 => Operation::new(panic, 20, "RET NC",      ValueMode::None),
                0x01 => Operation::new(opxD1, 12, "POP DE",      ValueMode::None),
                0x02 => Operation::new(panic, 16, "JP NC, {}",   ValueMode::A16),
                0x04 => Operation::new(panic, 24, "CALL NC, {}", ValueMode::A16),
                0x05 => Operation::new(opxD5, 16, "PUSH DE",     ValueMode::None),
                0x06 => Operation::new(opxD6,  8, "SUB {}",      ValueMode::D8),
                0x07 => Operation::new(panic, 16, "RST 10H",     ValueMode::None),
                0x08 => Operation::new(opxD8, 20, "RET C",       ValueMode::None),
                0x09 => Operation::new(panic, 16, "RETI",        ValueMode::None),
                0x0A => Operation::new(opxDA, 16, "JP C, {}",    ValueMode::A16),
                0x0C => Operation::new(panic, 24, "CALL C, {}",  ValueMode::A16),
                0x0E => Operation::new(panic, 16, "SBC A, {}",   ValueMode::D8),
                0x0F => Operation::new(opxDF, 16, "RST 18H",     ValueMode::D8),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0xE0 => match lcode {
                0x00 => Operation::new(opxE0, 12, "LDH ({}), A", ValueMode::A8Hi),
                0x01 => Operation::new(opxE1, 12, "POP HL",      ValueMode::None),
                0x02 => Operation::new(opxE2,  8, "LD (C), A",   ValueMode::None),
                0x05 => Operation::new(opxE5, 16, "PUSH HL",     ValueMode::None),
                0x06 => Operation::new(opxE6,  8, "AND d8",      ValueMode::D8),
                0x07 => Operation::new(panic, 16, "RST 20H",     ValueMode::None),
                0x08 => Operation::new(panic, 16, "ADD SP, {}",  ValueMode::R8),
                0x09 => Operation::new(opxE9,  4, "JP (HL)",     ValueMode::None),
                0x0A => Operation::new(opxEA, 16, "LD ({}), A",  ValueMode::A16),
                0x0E => Operation::new(panic,  8, "XOR {}",      ValueMode::D8),
                0x0F => Operation::new(opxEF, 16, "RST 28H",     ValueMode::D8),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0xF0 => match lcode {
                0x00 => Operation::new(opxF0, 12, "LDH A, ({})", ValueMode::A8Hi),
                0x01 => Operation::new(opxF1, 12, "POP AF",      ValueMode::None),
                0x02 => Operation::new(panic,  8, "LD A, (C)",   ValueMode::None),
                0x03 => Operation::new(opxF3,  4, "DI",          ValueMode::None),
                0x05 => Operation::new(opxF5, 16, "PUSH AF",     ValueMode::None),
                0x06 => Operation::new(panic,  8, "OR {}",       ValueMode::D8),
                0x07 => Operation::new(panic, 16, "RST 30H",     ValueMode::None),
                0x08 => Operation::new(panic, 12, "LD HL SP+{}", ValueMode::R8),
                0x09 => Operation::new(panic,  8, "LD SP, HL",   ValueMode::None),
                0x0A => Operation::new(opxFA, 16, "LD A, ({})",  ValueMode::A16),
                0x0B => Operation::new(opxFB,  4, "EI",          ValueMode::None),
                0x0E => Operation::new(opxFE,  8, "CP {}",       ValueMode::D8),
                0x0F => Operation::new(opxFF, 16, "RST 38H",     ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
        },
        0xCB => match scode {   // CB Prefix
            0x10 => match lcode {
                0x01 => Operation::new(cbx11,  8, "RL C",        ValueMode::None),
                0x08 => Operation::new(cbx18,  8, "RR B",        ValueMode::None),
                0x09 => Operation::new(cbx19,  8, "RR C",        ValueMode::None),
                0x0A => Operation::new(cbx1A,  8, "RR D",        ValueMode::None),
                0x0B => Operation::new(cbx1B,  8, "RR E",        ValueMode::None),
                0x0C => Operation::new(cbx1C,  8, "RR H",        ValueMode::None),
                0x0D => Operation::new(cbx1D,  8, "RR L",        ValueMode::None),
                0x0E => Operation::new(cbx1E,  8, "RR (HL)",     ValueMode::None),
                0x0F => Operation::new(cbx1F,  8, "RR A",        ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0x30 => match lcode {
                0x00 => Operation::new(cbx30,  8, "SWAP B",      ValueMode::None),
                0x01 => Operation::new(cbx31,  8, "SWAP C",      ValueMode::None),
                0x02 => Operation::new(cbx32,  8, "SWAP D",      ValueMode::None),
                0x03 => Operation::new(cbx33,  8, "SWAP E",      ValueMode::None),
                0x04 => Operation::new(cbx34,  8, "SWAP H",      ValueMode::None),
                0x05 => Operation::new(cbx35,  8, "SWAP L",      ValueMode::None),
                0x07 => Operation::new(cbx37,  8, "SWAP A",      ValueMode::None),
                0x08 => Operation::new(cbx38,  8, "SRL B",       ValueMode::None),
                0x09 => Operation::new(cbx39,  8, "SRL C",       ValueMode::None),
                0x0A => Operation::new(cbx3A,  8, "SRL D",       ValueMode::None),
                0x0B => Operation::new(cbx3B,  8, "SRL E",       ValueMode::None),
                0x0C => Operation::new(cbx3C,  8, "SRL H",       ValueMode::None),
                0x0D => Operation::new(cbx3D,  8, "SRL L",       ValueMode::None),
                0x0F => Operation::new(cbx3F,  8, "SRL A",       ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0x70 => match lcode {
                0x0C => Operation::new(cbx7C, 8, "BIT 7, H",     ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0x80 => match lcode {
                0x00 => Operation::new(cbx80, 8, "RES 0, B",     ValueMode::None),
                0x01 => Operation::new(cbx81, 8, "RES 0, C",     ValueMode::None),
                0x02 => Operation::new(cbx82, 8, "RES 0, D",     ValueMode::None),
                0x03 => Operation::new(cbx83, 8, "RES 0, E",     ValueMode::None),
                0x04 => Operation::new(cbx84, 8, "RES 0, H",     ValueMode::None),
                0x05 => Operation::new(cbx85, 8, "RES 0, L",     ValueMode::None),
                0x06 => Operation::new(cbx86, 8, "RES 0, (HL)",  ValueMode::None),
                0x07 => Operation::new(cbx87, 8, "RES 0, A",     ValueMode::None),
                0x08 => Operation::new(cbx88, 8, "RES 1, B",     ValueMode::None),
                0x09 => Operation::new(cbx89, 8, "RES 1, C",     ValueMode::None),
                0x0A => Operation::new(cbx8A, 8, "RES 1, D",     ValueMode::None),
                0x0B => Operation::new(cbx8B, 8, "RES 1, E",     ValueMode::None),
                0x0C => Operation::new(cbx8C, 8, "RES 1, H",     ValueMode::None),
                0x0D => Operation::new(cbx8D, 8, "RES 1, L",     ValueMode::None),
                0x0E => Operation::new(cbx8E, 8, "RES 1, (HL)",  ValueMode::None),
                0x0F => Operation::new(cbx8F, 8, "RES 1, A",     ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0x90 => match lcode {
                0x00 => Operation::new(cbx90, 8, "RES 2, B",     ValueMode::None),
                0x01 => Operation::new(cbx91, 8, "RES 2, C",     ValueMode::None),
                0x02 => Operation::new(cbx92, 8, "RES 2, D",     ValueMode::None),
                0x03 => Operation::new(cbx93, 8, "RES 2, E",     ValueMode::None),
                0x04 => Operation::new(cbx94, 8, "RES 2, H",     ValueMode::None),
                0x05 => Operation::new(cbx95, 8, "RES 2, L",     ValueMode::None),
                0x06 => Operation::new(cbx96, 8, "RES 2, (HL)",  ValueMode::None),
                0x07 => Operation::new(cbx97, 8, "RES 2, A",     ValueMode::None),
                0x08 => Operation::new(cbx98, 8, "RES 3, B",     ValueMode::None),
                0x09 => Operation::new(cbx99, 8, "RES 3, C",     ValueMode::None),
                0x0A => Operation::new(cbx9A, 8, "RES 3, D",     ValueMode::None),
                0x0B => Operation::new(cbx9B, 8, "RES 3, E",     ValueMode::None),
                0x0C => Operation::new(cbx9C, 8, "RES 3, H",     ValueMode::None),
                0x0D => Operation::new(cbx9D, 8, "RES 3, L",     ValueMode::None),
                0x0E => Operation::new(cbx9E, 8, "RES 3, (HL)",  ValueMode::None),
                0x0F => Operation::new(cbx9F, 8, "RES 3, A",     ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0xA0 => match lcode {
                0x00 => Operation::new(cbxA0, 8, "RES 4, B",     ValueMode::None),
                0x01 => Operation::new(cbxA1, 8, "RES 4, C",     ValueMode::None),
                0x02 => Operation::new(cbxA2, 8, "RES 4, D",     ValueMode::None),
                0x03 => Operation::new(cbxA3, 8, "RES 4, E",     ValueMode::None),
                0x04 => Operation::new(cbxA4, 8, "RES 4, H",     ValueMode::None),
                0x05 => Operation::new(cbxA5, 8, "RES 4, L",     ValueMode::None),
                0x06 => Operation::new(cbxA6, 8, "RES 4, (HL)",  ValueMode::None),
                0x07 => Operation::new(cbxA7, 8, "RES 4, A",     ValueMode::None),
                0x08 => Operation::new(cbxA8, 8, "RES 5, B",     ValueMode::None),
                0x09 => Operation::new(cbxA9, 8, "RES 5, C",     ValueMode::None),
                0x0A => Operation::new(cbxAA, 8, "RES 5, D",     ValueMode::None),
                0x0B => Operation::new(cbxAB, 8, "RES 5, E",     ValueMode::None),
                0x0C => Operation::new(cbxAC, 8, "RES 5, H",     ValueMode::None),
                0x0D => Operation::new(cbxAD, 8, "RES 5, L",     ValueMode::None),
                0x0E => Operation::new(cbxAE, 8, "RES 5, (HL)",  ValueMode::None),
                0x0F => Operation::new(cbxAF, 8, "RES 5, A",     ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0xB0 => match lcode {
                0x00 => Operation::new(cbxB0, 8, "RES 6, B",     ValueMode::None),
                0x01 => Operation::new(cbxB1, 8, "RES 6, C",     ValueMode::None),
                0x02 => Operation::new(cbxB2, 8, "RES 6, D",     ValueMode::None),
                0x03 => Operation::new(cbxB3, 8, "RES 6, E",     ValueMode::None),
                0x04 => Operation::new(cbxB4, 8, "RES 6, H",     ValueMode::None),
                0x05 => Operation::new(cbxB5, 8, "RES 6, L",     ValueMode::None),
                0x06 => Operation::new(cbxB6, 8, "RES 6, (HL)",  ValueMode::None),
                0x07 => Operation::new(cbxB7, 8, "RES 6, A",     ValueMode::None),
                0x08 => Operation::new(cbxB8, 8, "RES 7, B",     ValueMode::None),
                0x09 => Operation::new(cbxB9, 8, "RES 7, C",     ValueMode::None),
                0x0A => Operation::new(cbxBA, 8, "RES 7, D",     ValueMode::None),
                0x0B => Operation::new(cbxBB, 8, "RES 7, E",     ValueMode::None),
                0x0C => Operation::new(cbxBC, 8, "RES 7, H",     ValueMode::None),
                0x0D => Operation::new(cbxBD, 8, "RES 7, L",     ValueMode::None),
                0x0E => Operation::new(cbxBE, 8, "RES 7, (HL)",  ValueMode::None),
                0x0F => Operation::new(cbxBF, 8, "RES 7, A",     ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
        },
        _ => panic!("0x{:04X} is not a valid opcode.", code),
    }
}

pub fn panic(cpu: &mut Cpu, mmu: &mut Mmu) { panic!("THIS CODE NOT IMPLEMENTED!")}
pub fn opx00(cpu: &mut Cpu, mmu: &mut Mmu) { /* NOP */ }
pub fn opx01(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD BC, d16
    // Load d16 into the address specified at HL
    let d16 = cpu.immediate_u16_pc(mmu);
    cpu.regs.set_bc(d16);
}
pub fn opx03(cpu: &mut Cpu, mmu: &mut Mmu){
    let val = cpu.regs.bc();
    cpu.regs.set_bc(val.wrapping_add(1));
}
pub fn opx04(cpu: &mut Cpu, mmu: &mut Mmu){inc_x(&mut cpu.regs.b, &mut cpu.regs.flags)}
pub fn opx05(cpu: &mut Cpu, mmu: &mut Mmu){dec_x(&mut cpu.regs.b, &mut cpu.regs.flags)}
pub fn opx06(cpu: &mut Cpu, mmu: &mut Mmu){let v = cpu.immediate_u8_pc(mmu); ld_x_y(&mut cpu.regs.b, v)}
pub fn opx08(cpu: &mut Cpu, mmu: &mut Mmu){
    // LD (a16), SP
    // Load the 2-byte sp into 2-bytes of memory located at
    // the value specified by immediate_u16_pc
    let addr = cpu.immediate_u16_pc(mmu) as usize;
    mmu.write_u16(addr, cpu.regs.sp as u16);
}
pub fn opx09(cpu: &mut Cpu, mmu: &mut Mmu){let v=cpu.regs.bc(); add_hl(cpu, v)}
pub fn opx0B(cpu: &mut Cpu, mmu: &mut Mmu){let bc = cpu.regs.bc(); cpu.regs.set_bc(bc.wrapping_sub(1))}
pub fn opx0C(cpu: &mut Cpu, mmu: &mut Mmu){inc_x(&mut cpu.regs.c, &mut cpu.regs.flags)}
pub fn opx0D(cpu: &mut Cpu, mmu: &mut Mmu){dec_x(&mut cpu.regs.c, &mut cpu.regs.flags)}
pub fn opx0E(cpu: &mut Cpu, mmu: &mut Mmu){let v = cpu.immediate_u8_pc(mmu); ld_x_y(&mut cpu.regs.c, v)}
pub fn opx0F(cpu: &mut Cpu, mmu: &mut Mmu){
    let c = cpu.regs.flags.c;
    cpu.regs.flags.c = cpu.regs.a & 1 == 1;
    cpu.regs.a = cpu.regs.a >> 1;
    cpu.regs.a |= (c as u8) << 7;
}
pub fn opx18(cpu: &mut Cpu, mmu: &mut Mmu) {
    // JR r8
    let signed = cpu.immediate_u8_pc(mmu) as i8;
    match signed > 0 {
        true => cpu.regs.pc += signed.abs() as usize,
        _ => cpu.regs.pc -= signed.abs() as usize
    };
}
pub fn opx20(cpu: &mut Cpu, mmu: &mut Mmu) {
    // JR NZ, r8
    // Jump Relative if not zero (signed immediate 8-bit)
    let signed = cpu.immediate_u8_pc(mmu) as i8;
    if cpu.regs.flags.z == true {
        return {}
    };

    match signed > 0 {
        true => cpu.regs.pc += signed.abs() as usize,
        _ => cpu.regs.pc -= signed.abs() as usize
    };
}
pub fn opx21(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD HL, d16
    let new = cpu.immediate_u16_pc(mmu);
    cpu.regs.set_hl(new);
}

pub fn opx22(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD (HL+), A
    // Load the value of register A into mem address HL
    // Increment HL
    let hl = cpu.regs.hl() as usize;
    mmu.write(hl, cpu.regs.a);
    cpu.regs.set_hl((hl + 1) as u16);
}

pub fn opx23(cpu: &mut Cpu, mmu: &mut Mmu) {
    // INC HL
    // Increment HL by one.
    let new = cpu.regs.hl().wrapping_add(1);
    cpu.regs.set_hl(new);
}
pub fn opx13(cpu: &mut Cpu, mmu: &mut Mmu) {
    // INC DE
    // Increment DE by one.
    let new = cpu.regs.de().wrapping_add(1);
    cpu.regs.set_de(new);
}
pub fn opx32(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD (HL-), A
    // Load the value of register A into mem address HL
    // Decrement HL
    let addr = cpu.regs.hl() as usize;
    let a = cpu.regs.a;
    mmu.write(addr, a);
    cpu.regs.set_hl((addr as u16).wrapping_sub(1));
}
pub fn opx33(cpu: &mut Cpu, mmu: &mut Mmu){
    let val = cpu.regs.sp;
    cpu.regs.sp = val.wrapping_add(1)
}
pub fn opx31(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD SP, d16
    // Load immediate 16-bit into Stack Pointer
    let pc = cpu.regs.pc;
    let sp = cpu.immediate_u16_pc(mmu) as usize;
    cpu.regs.sp = sp;
}

pub fn opxE2(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD (C), A
    // Load value from register A into mem at address specified by register C
    let c = cpu.regs.c as usize + 0xFF00;
    mmu.write(c as usize, cpu.regs.a);
}
pub fn opxEA(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD (a16), A
    // Load the value of register A into memory at address specified by immediate 16
    let addr = cpu.immediate_u16_pc(mmu) as usize;
    mmu.write(addr, cpu.regs.a);
}
pub fn opx77(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD (HL), A
    // Load value of register A into mem at address specified by register HL
    let hl = cpu.regs.hl() as usize;
    mmu.write(hl, cpu.regs.a);
}

pub fn opxE0(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LDH (a8), A
    // Load the value of register A into mem at 0xFF00 + immediate 8-bit
    let addr = (0xFF00 + cpu.immediate_u8_pc(mmu) as u16) as usize;
    mmu.write(addr, cpu.regs.a);
}
pub fn opx11(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD DE, d16
    // Load immediate 16-bit into register DE
    let d16 = cpu.immediate_u16_pc(mmu);
    cpu.regs.set_de(d16);
}
pub fn opx1A(cpu: &mut Cpu, mmu: &mut Mmu) {
    // MARK
    // "LD A, (DE)"
    // Load value of memory at address specified in DE into register A
    let addr = mmu.read(cpu.regs.de() as usize);
    cpu.regs.a = addr;
}

pub fn opxCD(cpu: &mut Cpu, mmu: &mut Mmu) {
    // Call a16
    // Set pc to value of immediate 16-bit
    // push both bytes of pc onto the stack
    // increment the sp by two
    let a16 = cpu.immediate_u16_pc(mmu);
    let pc = cpu.regs.pc as u16;
    cpu.stack_push_u16(pc, mmu);
    cpu.regs.pc = a16 as usize;
}


pub fn opxCC(cpu: &mut Cpu, mmu: &mut Mmu) {
    // Call Z, a16
    // Set pc to value of immediate 16-bit
    // push both bytes of pc onto the stack
    // increment the sp by two
    if cpu.regs.flags.z == true {
        opxCD(cpu, mmu);
    } else {
        cpu.regs.pc += 2;
    }
}

pub fn opxC3(cpu: &mut Cpu, mmu: &mut Mmu) {
    let addr = cpu.immediate_u16_pc(mmu) as usize;
    cpu.regs.pc = addr;
}

pub fn opxC6(cpu: &mut Cpu, mmu: &mut Mmu) {
    let d8 = cpu.immediate_u8_pc(mmu);
    let hc = add_hc_u8(cpu.regs.a, d8);
    let c = add_c_u8(cpu.regs.a, d8);
    cpu.regs.a = cpu.regs.a.wrapping_add(d8);
    cpu.regs.flags.z = cpu.regs.a == 0;
    cpu.regs.flags.n = false;
    cpu.regs.flags.h = hc;
    cpu.regs.flags.c = c;
}

pub fn opxE9(cpu: &mut Cpu, mmu: &mut Mmu) {
    // JP (HL)
    let hl = cpu.regs.hl() as usize;
    cpu.regs.pc = hl;
}
pub fn opxE6(cpu: &mut Cpu, mmu: &mut Mmu) {
    // AND d8
    let d8 = cpu.immediate_u8(mmu);
    cpu.regs.a &= d8;
    cpu.regs.flags.z = cpu.regs.a == 0;
    cpu.regs.flags.h = true;
}

pub fn opx17(cpu: &mut Cpu, mmu: &mut Mmu) {
    // RLA - Shift A left, place lost bit into carry, and move carry to bit 0.
    let msb = cpu.regs.a >> 7;
    cpu.regs.a = cpu.regs.a << 1;
    cpu.regs.a |= cpu.regs.flags.c as u8;
    cpu.regs.flags.c = msb == 1;
}
pub fn opxC9(cpu: &mut Cpu, mmu: &mut Mmu) {
    // RET
    cpu.regs.pc = cpu.stack_pop_u16(mmu) as usize;
}
pub fn opxC0(cpu: &mut Cpu, mmu: &mut Mmu) {
    if cpu.regs.flags.z == false {
        let popped = cpu.stack_pop_u16(mmu) as usize;
        cpu.regs.pc = popped;
    }
}

pub fn opxFE(cpu: &mut Cpu, mmu: &mut Mmu) {
    // CP d8
    // Compare A with d8
    // set flags Z, H and C as required
    // set N flag to 1
    let a = cpu.regs.a;
    let d8 = cpu.immediate_u8_pc(mmu);
    let hc = sub_hc_u8(a, d8);
    cpu.regs.flags.z = a == d8;
    cpu.regs.flags.c = a < d8;
    cpu.regs.flags.n = true;
    cpu.regs.flags.h = hc;
}
pub fn opxCE(cpu: &mut Cpu, mmu: &mut Mmu) {
    // ADC A, d8
    // Z 0 H C
    let ac = cpu.regs.flags.c as u8 + cpu.immediate_u8_pc(mmu);
    let a = cpu.regs.a;
    let c = add_c_u8(a, ac);
    let hc = add_hc_u8(a, ac);
    cpu.regs.a = a.wrapping_add(ac);
    cpu.regs.flags.z = cpu.regs.a == 0;
    cpu.regs.flags.n = false;
    cpu.regs.flags.h = hc;
    cpu.regs.flags.c = c;
}
pub  fn cbx7C(cpu: &mut Cpu, mmu: &mut Mmu) {
    let reg = &mut cpu.regs.h;
    let flags = &mut cpu.regs.flags;
    bit_x_n(7, reg, flags);
}

pub  fn cbx11(cpu: &mut Cpu, mmu: &mut Mmu) {
    let reg = &mut cpu.regs.c;
    let flags = &mut cpu.regs.flags;
    rl_n(reg, flags);
}

fn rl_n(reg: &mut u8, flags: &mut FlagRegister) {
    // RL n
    // Rotate register n one bit to the left
    // MSB goes into carry flag
    // carry flag goes into lsbit of C
    // if the result is zero, set the zero flag to 1 else 0
    let msb = *reg >> 7;
    let carry = flags.c as u8;
    *reg = *reg << 1;
    *reg |= carry;
    flags.c = msb == 1;
    flags.z = match *reg {
        0 => true,
        _ => false,
    };
}

fn bit_x_n(bit_no: usize, reg: &mut u8, flags: &mut FlagRegister) {
    // BIT x, n
    // Clear the zero flag if bit x of register n == 1
    // set N flag to 0 and H flag to 1
    flags.z = reg.get_bit(bit_no) == 0;
    flags.n = false;
    flags.h = true;
}
fn dec_x(reg: &mut u8, flags: &mut FlagRegister) {
    let hc = sub_hc_u8(*reg, 1);
    *reg = reg.wrapping_sub(1);
    flags.h = hc;
    flags.n = true;
    flags.z = *reg == 0;
}

fn inc_x(reg: &mut u8, flags: &mut FlagRegister) {
    let hc = add_hc_u8(*reg, 1);
    *reg = reg.wrapping_add(1);
    flags.z = *reg == 0;
    flags.n = false;
    flags.h = hc;
}
pub fn opx14(cpu: &mut Cpu, mmu: &mut Mmu) {inc_x(&mut cpu.regs.d, &mut cpu.regs.flags)}
pub fn opx24(cpu: &mut Cpu, mmu: &mut Mmu) {inc_x(&mut cpu.regs.h, &mut cpu.regs.flags)}
pub fn opx1C(cpu: &mut Cpu, mmu: &mut Mmu) {inc_x(&mut cpu.regs.e, &mut cpu.regs.flags)}
pub fn opx2C(cpu: &mut Cpu, mmu: &mut Mmu) {inc_x(&mut cpu.regs.l, &mut cpu.regs.flags)}
pub fn opx3C(cpu: &mut Cpu, mmu: &mut Mmu) {inc_x(&mut cpu.regs.a, &mut cpu.regs.flags)}

pub fn opx15(cpu: &mut Cpu, mmu: &mut Mmu) {dec_x(&mut cpu.regs.d, &mut cpu.regs.flags)}
pub fn opx25(cpu: &mut Cpu, mmu: &mut Mmu) {dec_x(&mut cpu.regs.h, &mut cpu.regs.flags)}
pub fn opx1D(cpu: &mut Cpu, mmu: &mut Mmu) {dec_x(&mut cpu.regs.e, &mut cpu.regs.flags)}
pub fn opx2D(cpu: &mut Cpu, mmu: &mut Mmu) {dec_x(&mut cpu.regs.l, &mut cpu.regs.flags)}
pub fn opx3D(cpu: &mut Cpu, mmu: &mut Mmu) {dec_x(&mut cpu.regs.a, &mut cpu.regs.flags)}

pub fn opx35(cpu: &mut Cpu, mmu: &mut Mmu) {
    // DEC (HL)
    let addr = cpu.regs.hl() as usize;
    let mut v = mmu.read(addr);
    let hc = sub_hc_u8(v, 1);
    v = v.wrapping_sub(1);
    mmu.write(addr, v);
    cpu.regs.flags.h = hc;
    cpu.regs.flags.n = true;
    cpu.regs.flags.z = v == 0;
}
pub fn opx28(cpu: &mut Cpu, mmu: &mut Mmu) {
    // JR Z, r8
    // Jump relative if Z flag == true
    let signed = cpu.immediate_u8_pc(mmu) as i8;
    if cpu.regs.flags.z == false {return {}};
    match signed > 0 {
        true => cpu.regs.pc += signed.abs() as usize,
        _ => cpu.regs.pc -= signed.abs() as usize
    };
}
pub fn opxF3(cpu: &mut Cpu, mmu: &mut Mmu){
    mmu.ime = false;
}
pub fn opxFB(cpu: &mut Cpu, mmu: &mut Mmu){
    mmu.ime = true;
}
pub fn ld_x_y(regx: &mut u8, regy: u8) { *regx = regy }

pub fn opx1E(cpu: &mut Cpu, mmu: &mut Mmu){let v = cpu.immediate_u8_pc(mmu); ld_x_y(&mut cpu.regs.e, v)}
pub fn opx2E(cpu: &mut Cpu, mmu: &mut Mmu){let v = cpu.immediate_u8_pc(mmu); ld_x_y(&mut cpu.regs.l, v)}
pub fn opx3E(cpu: &mut Cpu, mmu: &mut Mmu){let v = cpu.immediate_u8_pc(mmu); ld_x_y(&mut cpu.regs.a, v)}
pub fn opx16(cpu: &mut Cpu, mmu: &mut Mmu){let v = cpu.immediate_u8_pc(mmu); ld_x_y(&mut cpu.regs.d, v)}
pub fn opx26(cpu: &mut Cpu, mmu: &mut Mmu){let v = cpu.immediate_u8_pc(mmu); ld_x_y(&mut cpu.regs.h, v)}

pub fn opxF0(cpu: &mut Cpu, mmu: &mut Mmu){
    let a = 0xFF00 + cpu.immediate_u8_pc(mmu) as usize;
    ld_x_y(&mut cpu.regs.a, mmu.read(a))
}
fn sub_a_x(val: u8, cpu: &mut Cpu) {
    let hc = sub_hc_u8(cpu.regs.a, val);
    let c = sub_c_u8(cpu.regs.a, val);
    cpu.regs.a = cpu.regs.a.wrapping_sub(val);
    cpu.regs.flags.z = cpu.regs.a == 0;
    cpu.regs.flags.n = true;
    cpu.regs.flags.h = hc;
    cpu.regs.flags.c = c;
}

pub fn opxD6(cpu: &mut Cpu, mmu: &mut Mmu){
    let d8 = cpu.immediate_u8_pc(mmu);
    sub_a_x(d8, cpu);
}
pub fn opx40(cpu: &mut Cpu, mmu: &mut Mmu){}
pub fn opx41(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.b, cpu.regs.c)}
pub fn opx42(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.b, cpu.regs.d)}
pub fn opx43(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.b, cpu.regs.e)}
pub fn opx44(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.b, cpu.regs.h)}
pub fn opx45(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.b, cpu.regs.l)}
pub fn opx46(cpu: &mut Cpu, mmu: &mut Mmu){let v=mmu.read(cpu.regs.hl() as usize);ld_x_y(&mut cpu.regs.b, v)}
pub fn opx47(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.b, cpu.regs.a)}
pub fn opx48(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.b, cpu.regs.a)}
pub fn opx49(cpu: &mut Cpu, mmu: &mut Mmu){}
pub fn opx4A(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.c, cpu.regs.d)}
pub fn opx4B(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.c, cpu.regs.e)}
pub fn opx4C(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.c, cpu.regs.h)}
pub fn opx4D(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.c, cpu.regs.l)}
pub fn opx4E(cpu: &mut Cpu, mmu: &mut Mmu){let v=mmu.read(cpu.regs.hl() as usize);ld_x_y(&mut cpu.regs.c, v)}
pub fn opx4F(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.c, cpu.regs.a)}

pub fn opx50(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.d, cpu.regs.b)}
pub fn opx51(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.d, cpu.regs.c)}
pub fn opx52(cpu: &mut Cpu, mmu: &mut Mmu){}
pub fn opx53(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.d, cpu.regs.e)}
pub fn opx54(cpu: &mut Cpu, mmu: &mut Mmu){}
pub fn opx55(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.d, cpu.regs.l)}
pub fn opx56(cpu: &mut Cpu, mmu: &mut Mmu){let v=mmu.read(cpu.regs.hl() as usize);ld_x_y(&mut cpu.regs.d, v)}
pub fn opx57(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.d, cpu.regs.a)}
pub fn opx58(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.e, cpu.regs.b)}
pub fn opx59(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.e, cpu.regs.c)}
pub fn opx5A(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.e, cpu.regs.d)}
pub fn opx5B(cpu: &mut Cpu, mmu: &mut Mmu){}
pub fn opx5C(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.e, cpu.regs.h)}
pub fn opx5D(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.e, cpu.regs.l)}
pub fn opx5E(cpu: &mut Cpu, mmu: &mut Mmu){let v=mmu.read(cpu.regs.hl() as usize);ld_x_y(&mut cpu.regs.e, v)}
pub fn opx5F(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.e, cpu.regs.a)}

pub fn opx60(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.h, cpu.regs.b)}
pub fn opx61(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.h, cpu.regs.c)}
pub fn opx62(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.h, cpu.regs.d)}
pub fn opx63(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.h, cpu.regs.e)}
pub fn opx64(cpu: &mut Cpu, mmu: &mut Mmu){}
pub fn opx65(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.h, cpu.regs.l)}
pub fn opx66(cpu: &mut Cpu, mmu: &mut Mmu){let v=mmu.read(cpu.regs.hl() as usize);ld_x_y(&mut cpu.regs.h, v)}
pub fn opx67(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.h, cpu.regs.a)}
pub fn opx68(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.l, cpu.regs.b)}
pub fn opx69(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.l, cpu.regs.c)}
pub fn opx6A(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.l, cpu.regs.d)}
pub fn opx6B(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.l, cpu.regs.e)}
pub fn opx6C(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.l, cpu.regs.h)}
pub fn opx6D(cpu: &mut Cpu, mmu: &mut Mmu){}
pub fn opx6E(cpu: &mut Cpu, mmu: &mut Mmu){let v=mmu.read(cpu.regs.hl() as usize);ld_x_y(&mut cpu.regs.l, v)}
pub fn opx6F(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.l, cpu.regs.a)}

pub fn opx78(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.a, cpu.regs.b)}
pub fn opx79(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.a, cpu.regs.c)}
pub fn opx7A(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.a, cpu.regs.d)}
pub fn opx7B(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.a, cpu.regs.e)}
pub fn opx7C(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.a, cpu.regs.h)}
pub fn opx7D(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.a, cpu.regs.l)}
pub fn opx7E(cpu: &mut Cpu, mmu: &mut Mmu){let v=mmu.read(cpu.regs.hl() as usize);ld_x_y(&mut cpu.regs.a, v)}
pub fn opx7F(cpu: &mut Cpu, mmu: &mut Mmu){}

pub fn opx80(cpu: &mut Cpu, mmu: &mut Mmu) {add_a_x(cpu.regs.b, cpu)}
pub fn opx81(cpu: &mut Cpu, mmu: &mut Mmu) {add_a_x(cpu.regs.c, cpu)}
pub fn opx82(cpu: &mut Cpu, mmu: &mut Mmu) {add_a_x(cpu.regs.d, cpu)}
pub fn opx83(cpu: &mut Cpu, mmu: &mut Mmu) {add_a_x(cpu.regs.e, cpu)}
pub fn opx84(cpu: &mut Cpu, mmu: &mut Mmu) {add_a_x(cpu.regs.h, cpu)}
pub fn opx85(cpu: &mut Cpu, mmu: &mut Mmu) {add_a_x(cpu.regs.l, cpu)}
pub fn opx86(cpu: &mut Cpu, mmu: &mut Mmu){let v=mmu.read(cpu.regs.hl() as usize); add_a_x(v, cpu)}
pub fn opx87(cpu: &mut Cpu, mmu: &mut Mmu) {add_a_x(cpu.regs.a, cpu)}
pub fn add_a_x(val: u8, cpu: &mut Cpu) {
    let c = add_c_u8(cpu.regs.a, val);
    let hc = add_hc_u8(cpu.regs.a, val);
    cpu.regs.flags.h = hc;
    cpu.regs.a = cpu.regs.a.wrapping_add(val);
    cpu.regs.flags.z = val == val;
    cpu.regs.flags.c = c;
    cpu.regs.flags.n = false;
}

pub fn opx90(cpu: &mut Cpu, mmu: &mut Mmu) {sub_a_x(cpu.regs.b, cpu)}
pub fn opx91(cpu: &mut Cpu, mmu: &mut Mmu) {sub_a_x(cpu.regs.c, cpu)}
pub fn opx92(cpu: &mut Cpu, mmu: &mut Mmu) {sub_a_x(cpu.regs.d, cpu)}
pub fn opx93(cpu: &mut Cpu, mmu: &mut Mmu) {sub_a_x(cpu.regs.e, cpu)}
pub fn opx94(cpu: &mut Cpu, mmu: &mut Mmu) {sub_a_x(cpu.regs.h, cpu)}
pub fn opx95(cpu: &mut Cpu, mmu: &mut Mmu) {sub_a_x(cpu.regs.l, cpu)}
pub fn opx97(cpu: &mut Cpu, mmu: &mut Mmu) {sub_a_x(cpu.regs.a, cpu)}

fn cp_x(val: u8, cpu: &mut Cpu) {
    let a = cpu.regs.a;
    let hc = sub_hc_u8(a, val);
    cpu.regs.flags.z = a == val;
    cpu.regs.flags.c = a < val;
    cpu.regs.flags.n = true;
    cpu.regs.flags.h = hc;
}

pub fn opxB8(cpu: &mut Cpu, mmu: &mut Mmu) {cp_x(cpu.regs.b, cpu)}
pub fn opxB9(cpu: &mut Cpu, mmu: &mut Mmu) {cp_x(cpu.regs.c, cpu)}
pub fn opxBA(cpu: &mut Cpu, mmu: &mut Mmu) {cp_x(cpu.regs.d, cpu)}
pub fn opxBB(cpu: &mut Cpu, mmu: &mut Mmu) {cp_x(cpu.regs.e, cpu)}
pub fn opxBC(cpu: &mut Cpu, mmu: &mut Mmu) {cp_x(cpu.regs.h, cpu)}
pub fn opxBD(cpu: &mut Cpu, mmu: &mut Mmu) {cp_x(cpu.regs.l, cpu)}
pub fn opxBF(cpu: &mut Cpu, mmu: &mut Mmu) {cp_x(cpu.regs.a, cpu)}
pub fn opxBE(cpu: &mut Cpu, mmu: &mut Mmu) {let a=cpu.regs.hl() as usize; cp_x(mmu.read(a), cpu)}

pub fn opx2A(cpu: &mut Cpu, mmu: &mut Mmu) {
    let addr = cpu.regs.hl();
    let hl = mmu.read(addr as usize);
    ld_x_y(&mut cpu.regs.a, hl);
    cpu.regs.set_hl(addr.wrapping_add(1));
}
pub fn opx3A(cpu: &mut Cpu, mmu: &mut Mmu) {
    let addr = cpu.regs.hl();
    let hl = mmu.read(addr as usize);
    ld_x_y(&mut cpu.regs.a, hl);
    cpu.regs.set_hl(addr.wrapping_sub(1));
}
pub fn opx12(cpu: &mut Cpu, mmu: &mut Mmu) {
    let addr = cpu.regs.de() as usize;
    mmu.write(addr, cpu.regs.a);
}

pub fn opx36(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD (HL), d8
    // Load d8 into the address specified at HL
    let addr = cpu.regs.hl() as usize;
    let d8 = cpu.immediate_u8_pc(mmu);
    mmu.write(addr, d8);
}
pub fn or_x(val: u8, cpu: &mut Cpu) {
    cpu.regs.a = cpu.regs.a | val;
    cpu.regs.flags.z = cpu.regs.a == 0;
    cpu.regs.flags.n = false;
    cpu.regs.flags.h = false;
    cpu.regs.flags.c = false;
}

pub fn opxB0(cpu: &mut Cpu, mmu: &mut Mmu) {or_x(cpu.regs.b, cpu)}
pub fn opxB1(cpu: &mut Cpu, mmu: &mut Mmu) {or_x(cpu.regs.c, cpu)}
pub fn opxB2(cpu: &mut Cpu, mmu: &mut Mmu) {or_x(cpu.regs.d, cpu)}
pub fn opxB3(cpu: &mut Cpu, mmu: &mut Mmu) {or_x(cpu.regs.e, cpu)}
pub fn opxB4(cpu: &mut Cpu, mmu: &mut Mmu) {or_x(cpu.regs.h, cpu)}
pub fn opxB5(cpu: &mut Cpu, mmu: &mut Mmu) {or_x(cpu.regs.l, cpu)}
pub fn opxB6(cpu: &mut Cpu, mmu: &mut Mmu) {let hl = cpu.regs.hl() as usize; or_x(mmu.read(hl), cpu)}
pub fn opxB7(cpu: &mut Cpu, mmu: &mut Mmu) {or_x(cpu.regs.a, cpu)}

pub fn opx2F(cpu: &mut Cpu, mmu: &mut Mmu) {
    // CPL - Complement A (flip all bits)
    let a = cpu.regs.a;
    cpu.regs.a = !a;
    cpu.regs.flags.n = true;
    cpu.regs.flags.h = true;
}

pub fn cbx30(cpu: &mut Cpu, mmu: &mut Mmu){swap(&mut cpu.regs.b, &mut cpu.regs.flags)}
pub fn cbx31(cpu: &mut Cpu, mmu: &mut Mmu){swap(&mut cpu.regs.c, &mut cpu.regs.flags)}
pub fn cbx32(cpu: &mut Cpu, mmu: &mut Mmu){swap(&mut cpu.regs.d, &mut cpu.regs.flags)}
pub fn cbx33(cpu: &mut Cpu, mmu: &mut Mmu){swap(&mut cpu.regs.e, &mut cpu.regs.flags)}
pub fn cbx34(cpu: &mut Cpu, mmu: &mut Mmu){swap(&mut cpu.regs.h, &mut cpu.regs.flags)}
pub fn cbx35(cpu: &mut Cpu, mmu: &mut Mmu){swap(&mut cpu.regs.l, &mut cpu.regs.flags)}
pub fn cbx37(cpu: &mut Cpu, mmu: &mut Mmu){swap(&mut cpu.regs.a, &mut cpu.regs.flags)}
pub fn swap(reg: &mut u8, flags: &mut FlagRegister) {
    *reg = (*reg << 4) | (*reg >> 4);
    flags.z = *reg == 0;
    flags.n = false;
    flags.h = false;
    flags.c = false;
}

pub fn opxA8(cpu: &mut Cpu, mmu: &mut Mmu){xor_a(cpu.regs.b, cpu)}
pub fn opxA9(cpu: &mut Cpu, mmu: &mut Mmu){xor_a(cpu.regs.c, cpu)}
pub fn opxAA(cpu: &mut Cpu, mmu: &mut Mmu){xor_a(cpu.regs.d, cpu)}
pub fn opxAB(cpu: &mut Cpu, mmu: &mut Mmu){xor_a(cpu.regs.e, cpu)}
pub fn opxAC(cpu: &mut Cpu, mmu: &mut Mmu){xor_a(cpu.regs.h, cpu)}
pub fn opxAD(cpu: &mut Cpu, mmu: &mut Mmu){xor_a(cpu.regs.l, cpu)}
pub fn opxAE(cpu: &mut Cpu, mmu: &mut Mmu){let x = mmu.read(cpu.regs.hl() as usize); xor_a(x, cpu)}
pub fn opxAF(cpu: &mut Cpu, mmu: &mut Mmu){xor_a(cpu.regs.a, cpu)}
pub fn xor_a(val: u8, cpu: &mut Cpu) {
    cpu.regs.a ^= val;
    cpu.regs.flags.z = cpu.regs.a == 0;
    cpu.regs.flags.n = false;
    cpu.regs.flags.h = false;
    cpu.regs.flags.c = false;
}


pub fn opxA0(cpu: &mut Cpu, mmu: &mut Mmu){and_x(cpu.regs.b, cpu)}
pub fn opxA1(cpu: &mut Cpu, mmu: &mut Mmu){and_x(cpu.regs.c, cpu)}
pub fn opxA2(cpu: &mut Cpu, mmu: &mut Mmu){and_x(cpu.regs.d, cpu)}
pub fn opxA3(cpu: &mut Cpu, mmu: &mut Mmu){and_x(cpu.regs.e, cpu)}
pub fn opxA4(cpu: &mut Cpu, mmu: &mut Mmu){and_x(cpu.regs.h, cpu)}
pub fn opxA5(cpu: &mut Cpu, mmu: &mut Mmu){and_x(cpu.regs.l, cpu)}
pub fn opxA6(cpu: &mut Cpu, mmu: &mut Mmu){let x = mmu.read(cpu.regs.hl() as usize); and_x(x, cpu)}
pub fn opxA7(cpu: &mut Cpu, mmu: &mut Mmu){and_x(cpu.regs.a, cpu)}
pub fn and_x(val: u8, cpu: &mut Cpu) {
    cpu.regs.a &= val;
    cpu.regs.flags.z = cpu.regs.a == 0;
    cpu.regs.flags.n = false;
    cpu.regs.flags.h = true;
    cpu.regs.flags.c = false;
}
pub fn opxCF(cpu: &mut Cpu, mmu: &mut Mmu){restart(cpu, mmu, 0x08)}
pub fn opxDF(cpu: &mut Cpu, mmu: &mut Mmu){restart(cpu, mmu, 0x18)}
pub fn opxEF(cpu: &mut Cpu, mmu: &mut Mmu){restart(cpu, mmu, 0x28)}
pub fn opxFF(cpu: &mut Cpu, mmu: &mut Mmu){restart(cpu, mmu, 0x38)}
pub fn restart(cpu: &mut Cpu, mmu: &mut Mmu, addr: usize){
    let pc = cpu.regs.pc as u16;
    cpu.stack_push_u16(pc, mmu);
    cpu.regs.pc = addr;
}

pub fn opxC1(cpu: &mut Cpu, mmu: &mut Mmu){let v = cpu.stack_pop_u16(mmu); cpu.regs.set_bc(v)}
pub fn opxD1(cpu: &mut Cpu, mmu: &mut Mmu){let v = cpu.stack_pop_u16(mmu); cpu.regs.set_de(v)}
pub fn opxE1(cpu: &mut Cpu, mmu: &mut Mmu){let v = cpu.stack_pop_u16(mmu); cpu.regs.set_hl(v)}
pub fn opxF1(cpu: &mut Cpu, mmu: &mut Mmu){let v = cpu.stack_pop_u16(mmu); cpu.regs.set_af(v)}

pub fn add_hl(cpu: &mut Cpu, val: u16){
    let hl = cpu.regs.hl();
    let hc = add_hc_u16(hl, val);
    let c = add_c_u16(hl, val);
    cpu.regs.set_hl(hl.wrapping_add(val));
    cpu.regs.flags.n = false;
    cpu.regs.flags.h = hc;
    cpu.regs.flags.c = c;
}

pub fn opx19(cpu: &mut Cpu, mmu: &mut Mmu){let v=cpu.regs.de(); add_hl(cpu, v)}
pub fn opx29(cpu: &mut Cpu, mmu: &mut Mmu){let v=cpu.regs.hl(); add_hl(cpu, v)}
pub fn opx39(cpu: &mut Cpu, mmu: &mut Mmu){let v=cpu.regs.sp as u16; add_hl(cpu, v)}


fn push_r16(cpu: &mut Cpu, mmu: &mut Mmu, val: u16) {
    cpu.stack_push_u16(val, mmu);
}

pub fn opxC5(cpu: &mut Cpu, mmu: &mut Mmu){let v=cpu.regs.bc();push_r16(cpu, mmu, v)}
pub fn opxD5(cpu: &mut Cpu, mmu: &mut Mmu){let v=cpu.regs.de();push_r16(cpu, mmu, v)}
pub fn opxE5(cpu: &mut Cpu, mmu: &mut Mmu){let v=cpu.regs.hl();push_r16(cpu, mmu, v)}
pub fn opxF5(cpu: &mut Cpu, mmu: &mut Mmu){let v=cpu.regs.af();push_r16(cpu, mmu, v)}

pub fn add_hc_u16(a: u16, b: u16) -> bool {
    (((a & 0xFF) + (b & 0xFF)) > 0xFF)
}

pub fn add_hc_u8(a: u8, b: u8) -> bool {
    (((a & 0xF) + (b & 0xF)) > 0xF)
}

pub fn add_c_u16(a: u16, b: u16) -> bool {
    let biga: u32 = a as u32;
    let bigb: u32 = b as u32;
    (biga + bigb) > 0xFFFF
}
pub fn add_c_u8(a: u8, b: u8) -> bool {
    let biga: u16 = a as u16;
    let bigb: u16 = b as u16;
    (biga + bigb) > 0xFF
}
fn sub_hc_u8(a: u8, b: u8) -> bool {
    (b & 0xF) < (b & 0xF)
}

fn sub_hc_u16(a: u16, b: u16) -> bool {
    (b & 0xFF) < (b & 0xFF)
}

fn sub_c_u8(a: u8, b: u8) -> bool {
    a < b
}

pub fn opxFA(cpu: &mut Cpu, mmu: &mut Mmu){
    // LD A, (a16)
    let addr = cpu.immediate_u16_pc(mmu) as usize;
    cpu.regs.a = mmu.read(addr);
}

pub fn opxCA(cpu: &mut Cpu, mmu: &mut Mmu){
    // JP Z, a16
    let addr = cpu.immediate_u16_pc(mmu) as usize;
    if cpu.regs.flags.z {
        cpu.regs.pc = addr;
    }
}
pub fn opxDA(cpu: &mut Cpu, mmu: &mut Mmu){
    // JP C, a16
    let addr = cpu.immediate_u16_pc(mmu) as usize;
    if cpu.regs.flags.c {
        cpu.regs.pc = addr;
    }
}

pub fn opxC8(cpu: &mut Cpu, mmu: &mut Mmu){
    // RET Z
    if cpu.regs.flags.z{
        cpu.regs.pc = cpu.stack_pop_u16(mmu) as usize;
    }
}

pub fn opxD8(cpu: &mut Cpu, mmu: &mut Mmu){
    // RET C
    if cpu.regs.flags.c{
        cpu.regs.pc = cpu.stack_pop_u16(mmu) as usize;
    }
}

pub fn opxC4(cpu: &mut Cpu, mmu: &mut Mmu){
    let addr = cpu.immediate_u16_pc(mmu) as usize;
    if cpu.regs.flags.z == false {
        cpu.regs.pc = addr;
    }
}

pub fn srl(reg: &mut u8, flags: &mut FlagRegister) {
    let val = *reg;
    *reg = *reg >> 1;
    flags.z = *reg == 0;
    flags.c = val & 1 != 0;
    flags.n = false;
    flags.h = false;
}

pub fn cbx38(cpu: &mut Cpu, mmu: &mut Mmu){let f = &mut cpu.regs.flags; srl(&mut cpu.regs.b, f)}
pub fn cbx39(cpu: &mut Cpu, mmu: &mut Mmu){let f = &mut cpu.regs.flags; srl(&mut cpu.regs.c, f)}
pub fn cbx3A(cpu: &mut Cpu, mmu: &mut Mmu){let f = &mut cpu.regs.flags; srl(&mut cpu.regs.d, f)}
pub fn cbx3B(cpu: &mut Cpu, mmu: &mut Mmu){let f = &mut cpu.regs.flags; srl(&mut cpu.regs.e, f)}
pub fn cbx3C(cpu: &mut Cpu, mmu: &mut Mmu){let f = &mut cpu.regs.flags; srl(&mut cpu.regs.h, f)}
pub fn cbx3D(cpu: &mut Cpu, mmu: &mut Mmu){let f = &mut cpu.regs.flags; srl(&mut cpu.regs.l, f)}
pub fn cbx3F(cpu: &mut Cpu, mmu: &mut Mmu){let f = &mut cpu.regs.flags; srl(&mut cpu.regs.a, f)}

pub fn cbx80(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.b, 0)}
pub fn cbx81(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.c, 0)}
pub fn cbx82(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.d, 0)}
pub fn cbx83(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.e, 0)}
pub fn cbx84(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.h, 0)}
pub fn cbx85(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.l, 0)}
pub fn cbx86(cpu: &mut Cpu, mmu: &mut Mmu){let r = &mut mmu.read(cpu.regs.hl() as usize); cb_res(r, 0)}
pub fn cbx87(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.a, 0)}
pub fn cbx88(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.b, 1)}
pub fn cbx89(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.c, 1)}
pub fn cbx8A(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.d, 1)}
pub fn cbx8B(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.e, 1)}
pub fn cbx8C(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.h, 1)}
pub fn cbx8D(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.l, 1)}
pub fn cbx8E(cpu: &mut Cpu, mmu: &mut Mmu){let r = &mut mmu.read(cpu.regs.hl() as usize); cb_res(r, 1)}
pub fn cbx8F(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.a, 1)}

pub fn cbx90(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.b, 2)}
pub fn cbx91(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.c, 2)}
pub fn cbx92(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.d, 2)}
pub fn cbx93(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.e, 2)}
pub fn cbx94(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.h, 2)}
pub fn cbx95(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.l, 2)}
pub fn cbx96(cpu: &mut Cpu, mmu: &mut Mmu){let r = &mut mmu.read(cpu.regs.hl() as usize); cb_res(r, 2)}
pub fn cbx97(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.a, 2)}
pub fn cbx98(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.b, 3)}
pub fn cbx99(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.c, 3)}
pub fn cbx9A(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.d, 3)}
pub fn cbx9B(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.e, 3)}
pub fn cbx9C(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.h, 3)}
pub fn cbx9D(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.l, 3)}
pub fn cbx9E(cpu: &mut Cpu, mmu: &mut Mmu){let r = &mut mmu.read(cpu.regs.hl() as usize); cb_res(r, 3)}
pub fn cbx9F(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.a, 3)}

pub fn cbxA0(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.b, 4)}
pub fn cbxA1(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.c, 4)}
pub fn cbxA2(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.d, 4)}
pub fn cbxA3(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.e, 4)}
pub fn cbxA4(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.h, 4)}
pub fn cbxA5(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.l, 4)}
pub fn cbxA6(cpu: &mut Cpu, mmu: &mut Mmu){let r = &mut mmu.read(cpu.regs.hl() as usize); cb_res(r, 4)}
pub fn cbxA7(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.a, 4)}
pub fn cbxA8(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.b, 5)}
pub fn cbxA9(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.c, 5)}
pub fn cbxAA(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.d, 5)}
pub fn cbxAB(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.e, 5)}
pub fn cbxAC(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.h, 5)}
pub fn cbxAD(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.l, 5)}
pub fn cbxAE(cpu: &mut Cpu, mmu: &mut Mmu){let r = &mut mmu.read(cpu.regs.hl() as usize); cb_res(r, 5)}
pub fn cbxAF(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.a, 5)}

pub fn cbxB0(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.b, 6)}
pub fn cbxB1(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.c, 6)}
pub fn cbxB2(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.d, 6)}
pub fn cbxB3(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.e, 6)}
pub fn cbxB4(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.h, 6)}
pub fn cbxB5(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.l, 6)}
pub fn cbxB6(cpu: &mut Cpu, mmu: &mut Mmu){let r = &mut mmu.read(cpu.regs.hl() as usize); cb_res(r, 6)}
pub fn cbxB7(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.a, 6)}
pub fn cbxB8(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.b, 7)}
pub fn cbxB9(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.c, 7)}
pub fn cbxBA(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.d, 7)}
pub fn cbxBB(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.e, 7)}
pub fn cbxBC(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.h, 7)}
pub fn cbxBD(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.l, 7)}
pub fn cbxBE(cpu: &mut Cpu, mmu: &mut Mmu){let r = &mut mmu.read(cpu.regs.hl() as usize); cb_res(r, 7)}
pub fn cbxBF(cpu: &mut Cpu, mmu: &mut Mmu){cb_res(&mut cpu.regs.a, 7)}

fn cb_res(reg: &mut u8, bit: usize) {
    let mut r = *reg;
    r.set_bit(bit, 0);
    *reg = r;
}

pub fn cbx18(cpu: &mut Cpu, mmu: &mut Mmu){let f = &mut cpu.regs.flags; cb_rr(&mut cpu.regs.b, f)}
pub fn cbx19(cpu: &mut Cpu, mmu: &mut Mmu){let f = &mut cpu.regs.flags; cb_rr(&mut cpu.regs.c, f)}
pub fn cbx1A(cpu: &mut Cpu, mmu: &mut Mmu){let f = &mut cpu.regs.flags; cb_rr(&mut cpu.regs.d, f)}
pub fn cbx1B(cpu: &mut Cpu, mmu: &mut Mmu){let f = &mut cpu.regs.flags; cb_rr(&mut cpu.regs.e, f)}
pub fn cbx1C(cpu: &mut Cpu, mmu: &mut Mmu){let f = &mut cpu.regs.flags; cb_rr(&mut cpu.regs.h, f)}
pub fn cbx1D(cpu: &mut Cpu, mmu: &mut Mmu){let f = &mut cpu.regs.flags; cb_rr(&mut cpu.regs.l, f)}
pub fn cbx1E(cpu: &mut Cpu, mmu: &mut Mmu){let h = cpu.regs.hl() as usize;let f = &mut cpu.regs.flags;cb_rr(&mut mmu.read(h), f)}
pub fn cbx1F(cpu: &mut Cpu, mmu: &mut Mmu){let f = &mut cpu.regs.flags; cb_rr(&mut cpu.regs.a, f)}

fn cb_rr(x: &mut u8, flags: &mut FlagRegister) {
    let r = *x;
    let c = r & 1;
    *x = (r >> 1) | ((flags.c as u8) << 7);
    flags.c = c == 1;
    flags.z = *x == 0;
    flags.h = false;
    flags.n = false;
}
