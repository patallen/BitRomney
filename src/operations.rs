use std::fmt;

use cpu::Cpu;
use mmu::Mmu;
use registers::FlagRegister;

use bitty::BitFlags;

pub struct Operation {
    pub dis: &'static str,
    pub func: Box<Fn(&mut Cpu, &mut Mmu)>,
    pub size: usize,   // Number of bytes including opcode
    pub cycles: u8,
    pub mode: ValueMode,
}


impl Operation {
    pub fn new(func: fn(&mut Cpu, &mut Mmu),
               size: usize,
               cycles: u8,
               dis: &'static str,
               mode: ValueMode
    ) -> Operation {
        Operation {
            func: Box::new(func),
            cycles: cycles,
            size: size,
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
            ValueMode::R8    => Some(format!("${:02X}", cpu.immediate_u8(mmu) as i8)),
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
                0x00 => Operation::new(opx00, 1, 4,  "NOP", ValueMode::None),
                0x04 => Operation::new(opx04, 1, 4,  "INC B", ValueMode::None),
                0x05 => Operation::new(opx05, 1, 4,  "DEC B", ValueMode::None),
                0x06 => Operation::new(opx06, 2, 8,  "LD B, {}", ValueMode::D8),
                0x08 => Operation::new(opx08, 3, 20, "LD ({}), SP", ValueMode::A16),
                0x0C => Operation::new(opx0C, 1, 4,  "INC C", ValueMode::None),
                0x0D => Operation::new(opx0D, 1, 4,  "DEC C", ValueMode::None),
                0x0E => Operation::new(opx0E, 2, 8,  "LD C, {}", ValueMode::D8),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0x10 => match lcode {
                0x01 => Operation::new(opx11, 3, 12,  "LD DE, {}", ValueMode::D16),
                0x02 => Operation::new(opx12, 1, 8,  "LD (DE), A", ValueMode::None),
                0x03 => Operation::new(opx13, 1, 8,  "INC DE", ValueMode::None),
                0x04 => Operation::new(opx14, 1, 4,  "INC D", ValueMode::None),
                0x05 => Operation::new(opx15, 1, 4,  "DEC D", ValueMode::None),
                0x06 => Operation::new(opx16, 2, 8,  "LD D, {}", ValueMode::D8),
                0x07 => Operation::new(opx17, 1, 4,  "RLA", ValueMode::None),
                0x08 => Operation::new(opx18, 2, 8,  "JR {}", ValueMode::R8),
                0x0A => Operation::new(opx1A, 1, 8,  "LD A, (DE)", ValueMode::None),
                0x0C => Operation::new(opx1C, 1, 4,  "INC E", ValueMode::None),
                0x0D => Operation::new(opx1D, 1, 4,  "DEC E", ValueMode::None),
                0x0E => Operation::new(opx1E, 2, 8,  "LD E, {}", ValueMode::D8),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0x20 => match lcode {
                0x00 => Operation::new(opx20, 2, 12, "JR NZ, {}", ValueMode::R8),
                0x01 => Operation::new(opx21, 3, 12, "LD HL, {}", ValueMode::D16),
                0x02 => Operation::new(opx22, 1, 12, "LD (HL+), A", ValueMode::None),
                0x03 => Operation::new(opx23, 1,  8, "INC HL", ValueMode::None),
                0x04 => Operation::new(opx24, 1,  4, "INC H", ValueMode::None),
                0x05 => Operation::new(opx25, 1,  4, "DEC H", ValueMode::None),
                0x06 => Operation::new(opx26, 2, 8,  "LD H, {}", ValueMode::D8),
                0x08 => Operation::new(opx28, 2, 12, "JR Z, {}", ValueMode::R8),
                0x0A => Operation::new(opx2A, 1,  4, "STOP 0", ValueMode::None),
                0x0C => Operation::new(opx2C, 1,  4, "INC E", ValueMode::None),
                0x0D => Operation::new(opx2D, 1,  4, "DEC L", ValueMode::None),
                0x0E => Operation::new(opx2E, 2, 8,  "LD L, {}", ValueMode::D8),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0x30 => match lcode {
                0x01 => Operation::new(opx31, 3, 12, "LD SP, {}", ValueMode::D16),
                0x02 => Operation::new(opx32, 1,  8, "LD (HL-), A", ValueMode::None),
                0x05 => Operation::new(opx35, 1, 12, "DEC (HL)", ValueMode::None),
                0x0C => Operation::new(opx3C, 1,  4, "INC A", ValueMode::None),
                0x0D => Operation::new(opx3D, 1,  4, "DEC A", ValueMode::None),
                0x0E => Operation::new(opx3E, 2,  8, "LD A, {}", ValueMode::D8),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0x40 => match lcode {
                0x00 => Operation::new(opx40, 1, 4,  "LD B, B", ValueMode::None),
                0x01 => Operation::new(opx41, 1, 4,  "LD B, C", ValueMode::None),
                0x02 => Operation::new(opx42, 1, 4,  "LD B, D", ValueMode::None),
                0x03 => Operation::new(opx43, 1, 4,  "LD B, E", ValueMode::None),
                0x04 => Operation::new(opx44, 1, 4,  "LD B, H", ValueMode::None),
                0x05 => Operation::new(opx45, 1, 4,  "LD B, L", ValueMode::None),
                0x06 => Operation::new(opx46, 1, 8,  "LD B, (HL)", ValueMode::None),
                0x07 => Operation::new(opx47, 1, 4,  "LD B, A", ValueMode::None),
                0x08 => Operation::new(opx48, 1, 4,  "LD C, B", ValueMode::None),
                0x09 => Operation::new(opx49, 1, 4,  "LD C, C", ValueMode::None),
                0x0A => Operation::new(opx4A, 1, 4,  "LD C, D", ValueMode::None),
                0x0B => Operation::new(opx4B, 1, 4,  "LD C, E", ValueMode::None),
                0x0C => Operation::new(opx4C, 1, 4,  "LD C, H", ValueMode::None),
                0x0D => Operation::new(opx4D, 1, 4,  "LD C, L", ValueMode::None),
                0x0E => Operation::new(opx4E, 1, 8,  "LD C, (HL)", ValueMode::None),
                0x0F => Operation::new(opx4F, 1, 4,  "LD C, A", ValueMode::None),
                _   => Operation::new(unimplemented, 0, 0, "(unimplemented", ValueMode::None),
            },
            0x50 => match lcode {
                0x00 => Operation::new(opx50, 1, 4,  "LD H, B", ValueMode::None),
                0x01 => Operation::new(opx51, 1, 4,  "LD H, C", ValueMode::None),
                0x02 => Operation::new(opx52, 1, 4,  "LD H, D", ValueMode::None),
                0x03 => Operation::new(opx53, 1, 4,  "LD H, E", ValueMode::None),
                0x04 => Operation::new(opx54, 1, 4,  "LD H, H", ValueMode::None),
                0x05 => Operation::new(opx55, 1, 4,  "LD H, L", ValueMode::None),
                0x07 => Operation::new(opx57, 1, 4,  "LD H, A", ValueMode::None),
                0x08 => Operation::new(opx58, 1, 4,  "LD H, B", ValueMode::None),
                0x09 => Operation::new(opx59, 1, 4,  "LD H, C", ValueMode::None),
                0x0A => Operation::new(opx5A, 1, 4,  "LD H, D", ValueMode::None),
                0x0B => Operation::new(opx5B, 1, 4,  "LD H, E", ValueMode::None),
                0x0C => Operation::new(opx5C, 1, 4,  "LD H, H", ValueMode::None),
                0x0D => Operation::new(opx5D, 1, 4,  "LD H, L", ValueMode::None),
                0x0F => Operation::new(opx5F, 1, 4,  "LD H, A", ValueMode::None),
                _   => Operation::new(unimplemented, 0, 0, "(unimplemented", ValueMode::None),
            },
            0x60 => match lcode {
                0x00 => Operation::new(opx60, 1, 4,  "LD H, B", ValueMode::None),
                0x01 => Operation::new(opx61, 1, 4,  "LD H, C", ValueMode::None),
                0x02 => Operation::new(opx62, 1, 4,  "LD H, D", ValueMode::None),
                0x03 => Operation::new(opx63, 1, 4,  "LD H, E", ValueMode::None),
                0x04 => Operation::new(opx64, 1, 4,  "LD H, H", ValueMode::None),
                0x05 => Operation::new(opx65, 1, 4,  "LD H, L", ValueMode::None),
                0x06 => Operation::new(opx66, 1, 8,  "LD H, (HL)", ValueMode::None),
                0x07 => Operation::new(opx67, 1, 4,  "LD H, A", ValueMode::None),
                0x08 => Operation::new(opx68, 1, 4,  "LD H, B", ValueMode::None),
                0x09 => Operation::new(opx69, 1, 4,  "LD H, C", ValueMode::None),
                0x0A => Operation::new(opx6A, 1, 4,  "LD H, D", ValueMode::None),
                0x0B => Operation::new(opx6B, 1, 4,  "LD H, E", ValueMode::None),
                0x0C => Operation::new(opx6C, 1, 4,  "LD H, H", ValueMode::None),
                0x0D => Operation::new(opx6D, 1, 4,  "LD H, L", ValueMode::None),
                0x0F => Operation::new(opx6F, 1, 4,  "LD H, A", ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0x70 => match lcode {
                0x07 => Operation::new(opx77, 1, 8, "LD (HL), A", ValueMode::None),
                0x08 => Operation::new(opx78, 1, 4, "LD A, B", ValueMode::None),
                0x09 => Operation::new(opx79, 1, 4, "LD A, C", ValueMode::None),
                0x0A => Operation::new(opx7A, 1, 4, "LD A, D", ValueMode::None),
                0x0B => Operation::new(opx7B, 1, 4, "LD A, E", ValueMode::None),
                0x0C => Operation::new(opx7C, 1, 4, "LD A, H", ValueMode::None),
                0x0D => Operation::new(opx7D, 1, 4, "LD A, L", ValueMode::None),
                0x0F => Operation::new(opx7F, 1, 4, "LD A, A", ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0x80 => match lcode {
                0x00 => Operation::new(opx70, 1, 4,  "ADD A, B", ValueMode::None),
                0x01 => Operation::new(opx71, 1, 4,  "ADD A, C", ValueMode::None),
                0x02 => Operation::new(opx72, 1, 4,  "ADD A, D", ValueMode::None),
                0x03 => Operation::new(opx73, 1, 4,  "ADD A, E", ValueMode::None),
                0x04 => Operation::new(opx74, 1, 4,  "ADD A, H", ValueMode::None),
                0x05 => Operation::new(opx75, 1, 4,  "ADD A, L", ValueMode::None),
                0x06 => Operation::new(opx78, 1, 8,  "ADD A, (HL)", ValueMode::None),
                0x07 => Operation::new(opx77, 1, 4,  "ADD A, A", ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0x90 => match lcode {
                0x00 => Operation::new(opx90, 1, 4, "SUB B", ValueMode::None),
                0x01 => Operation::new(opx91, 1, 4, "SUB C", ValueMode::None),
                0x02 => Operation::new(opx92, 1, 4, "SUB D", ValueMode::None),
                0x03 => Operation::new(opx93, 1, 4, "SUB e", ValueMode::None),
                0x04 => Operation::new(opx94, 1, 4, "SUB H", ValueMode::None),
                0x05 => Operation::new(opx95, 1, 4, "SUB L", ValueMode::None),
                0x07 => Operation::new(opx97, 1, 4, "SUB A", ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0xA0 => match lcode {
                0x00 => Operation::new(opxA0, 1, 4, "AND B", ValueMode::None),
                0x07 => Operation::new(opxA7, 1, 4, "AND A", ValueMode::None),
                0x0F => Operation::new(opxAF, 1, 4, "XOR A, A", ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0xB0 => match lcode {
                0x08 => Operation::new(opxB8, 1, 4, "CP B", ValueMode::None),
                0x09 => Operation::new(opxB9, 1, 4, "CP C", ValueMode::None),
                0x0A => Operation::new(opxBA, 1, 4, "CP D", ValueMode::None),
                0x0B => Operation::new(opxBB, 1, 4, "CP E", ValueMode::None),
                0x0C => Operation::new(opxBC, 1, 4, "CP H", ValueMode::None),
                0x0D => Operation::new(opxBD, 1, 4, "CP L", ValueMode::None),
                0x0E => Operation::new(opxBE, 1, 8, "CP (HL)", ValueMode::None),
                0x0F => Operation::new(opxBF, 1, 4, "CP A", ValueMode::None),
                _    => Operation::new(unimplemented, 0, 0, "unimplemented", ValueMode::None),
            },
            0xC0 => match lcode {
                0x00 => Operation::new(opxC0, 1, 24, "RET NZ", ValueMode::None),
                0x01 => Operation::new(opxC1, 1, 12, "POP BC", ValueMode::None),
                0x03 => Operation::new(opxC3, 3, 12, "JP {}", ValueMode::A16),
                0x05 => Operation::new(opxC5, 1, 16, "PUSH BC", ValueMode::None),
                0x09 => Operation::new(opxC9, 0, 16, "RET", ValueMode::None),
                0x0C => Operation::new(opxCC, 0, 24, "CALL Z, {}", ValueMode::A16),
                0x0D => Operation::new(opxCD, 0, 24, "CALL {}", ValueMode::A16),
                0x0E => Operation::new(opxCE, 2, 8,  "ADC A, {}", ValueMode::D8),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0xE0 => match lcode {
                0x00 => Operation::new(opxE0, 2, 12, "LDH ({}), A", ValueMode::A8Hi),
                0x02 => Operation::new(opxE2, 1, 8, "LD (C), A", ValueMode::None),
                0x0A => Operation::new(opxEA, 3, 16, "LD ({}), A", ValueMode::A16),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0xF0 => match lcode {
                0x00 => Operation::new(opxF0, 2, 12, "LDH A, ({})", ValueMode::A8Hi),
                0x03 => Operation::new(opxF3, 1, 4, "DI", ValueMode::None),
                0x0B => Operation::new(opxFB, 1, 4, "EI", ValueMode::None),
                0x0E => Operation::new(opxFE, 2, 8, "CP {}", ValueMode::D8),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
        },
        0xCB => match scode {   // CB Prefix
            0x10 => match lcode {
                0x01 => Operation::new(cbx11, 2, 8, "RL C", ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            0x70 => match lcode {
                0x0C => Operation::new(cbx7C, 2, 8, "BIT 7, H", ValueMode::None),
                _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
            },
            _   => panic!(format!("Opcode 0x{:04x} is not yet implemented.", code)),
        },
        _ => panic!("0x{:04X} is not a valid opcode.", code),
    }
}

pub fn unimplemented(cpu: &mut Cpu, mmu: &mut Mmu) {}
pub fn opx00(cpu: &mut Cpu, mmu: &mut Mmu) {}
pub fn opx18(cpu: &mut Cpu, mmu: &mut Mmu) {
    // JR r8
    let signed = cpu.immediate_u8(mmu) as i8;
    match signed > 0 {
        true => cpu.regs.pc += signed.abs() as usize,
        _ => cpu.regs.pc -= signed.abs() as usize
    };
}
pub fn opx20(cpu: &mut Cpu, mmu: &mut Mmu) {
    // JR NZ, r8
    // Jump Relative if not zero (signed immediate 8-bit)
    if cpu.regs.flags.z == true { return {} };
    let signed = cpu.immediate_u8(mmu) as i8;
    match signed > 0 {
        true => cpu.regs.pc += signed.abs() as usize,
        _ => cpu.regs.pc -= signed.abs() as usize
    };
}
pub fn opx21(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD HL, d16
    let new = cpu.immediate_u16(mmu);
    cpu.regs.set_hl(new);
}

pub fn opx22(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD (HL+), A
    // Load the value of register A into mem address HL
    // Increment HL
    let addr = cpu.regs.hl() as usize;
    mmu.write(addr, cpu.regs.a);
    cpu.regs.set_hl((addr + 1) as u16);
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
pub fn opx31(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD SP, d16
    // Load immediate 16-bit into Stack Pointer
    let pc = cpu.regs.pc;
    let sp = cpu.immediate_u16(mmu);
    cpu.regs.sp = sp as usize;
}

pub fn opxA7(cpu: &mut Cpu, mmu: &mut Mmu) {
    // AND A
    // Set register A to A & A
    // Set zero if necessary
    // Set n & c = 0 and set h = 1
    let a = cpu.regs.a;
    cpu.regs.a &= a;
    cpu.regs.flags.n = false;
    cpu.regs.flags.c = false;
    cpu.regs.flags.h = true;
    cpu.regs.flags.z = cpu.regs.a == 0;
}
pub fn opxAF(cpu: &mut Cpu, mmu: &mut Mmu) {
    // XOR A
    // A ^= A and set zero flag if necessary
    let a = cpu.regs.a;
    cpu.regs.a ^= a;
    cpu.regs.flags.z = cpu.regs.a == 0;
}
pub fn opxE2(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD (C), A
    // Load value from register A into mem at address specified by register C
    let c = cpu.regs.c as usize + 0xFF00;
    mmu.write(c as usize, cpu.regs.a);
}
pub fn opxEA(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD (a16), A
    // Load the value of register 16 into memory at addres specified by immediate 16
    let a16 = cpu.immediate_u16(mmu) as usize;
    mmu.write(a16, cpu.regs.a);
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
    let addr = (0xFF00 + cpu.immediate_u8(mmu) as u16) as usize;
    mmu.write(addr, cpu.regs.a);
}
pub fn opx11(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD DE, d16
    // Load immediate 16-bit into register DE
    let d16 = cpu.immediate_u16(mmu);
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
    let pc = (cpu.regs.pc + 3) as u16;
    let nn = cpu.immediate_u16(mmu);
    cpu.stack_push_u16(pc, mmu);
    cpu.regs.pc = nn as usize;
}


pub fn opxCC(cpu: &mut Cpu, mmu: &mut Mmu) {
    // Call Z, a16
    // Set pc to value of immediate 16-bit
    // push both bytes of pc onto the stack
    // increment the sp by two
    if cpu.regs.flags.z == true {
        opxCD(cpu, mmu);
    }
}

pub fn opxC5(cpu: &mut Cpu, mmu: &mut Mmu) {
    // PUSH BC
    let bc = cpu.regs.bc();
    cpu.stack_push_u16(bc, mmu);
}

pub fn opxC1(cpu: &mut Cpu, mmu: &mut Mmu) {
    // POP BC
    let bc = cpu.stack_pop_u16(mmu);
    cpu.regs.set_bc(bc);
}
pub fn opxC3(cpu: &mut Cpu, mmu: &mut Mmu) {
    let addr = cpu.immediate_u16(mmu) as usize;
    cpu.regs.pc = addr;
}
pub fn opxA0(cpu: &mut Cpu, mmu: &mut Mmu) {
    // AND B - set N & C = 0, H = 1, Z conditionally
    cpu.regs.flags.c = false;
    cpu.regs.flags.n = false;
    cpu.regs.flags.h = false;
    cpu.regs.b &= cpu.regs.b;
    cpu.regs.flags.z = cpu.regs.b == 1;
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
    if cpu.regs.flags.z != false {
        opxC9(cpu, mmu);
    }
}
pub fn opxFE(cpu: &mut Cpu, mmu: &mut Mmu) {
    // CP d8
    // Compare A with d8
    // set flags Z, H and C as required
    // set N flag to 1
    let a = cpu.regs.a;
    let d8 = cpu.immediate_u8(mmu);
    cpu.regs.flags.z = a == d8;
    cpu.regs.flags.c = a < d8;
    cpu.regs.flags.n = true;
    cpu.regs.flags.h = (((a &0xF) + (d8 &0xF)) & 0x10) == 0x10;
}
pub fn opxCE(cpu: &mut Cpu, mmu: &mut Mmu) {
    // ADC A, d8
    // Z 0 H C
    let ac = cpu.regs.flags.c as u8 + cpu.immediate_u8(mmu);
    let a = cpu.regs.a;
    let test = a as u16;
    if test + ac as u16 > 255 {
        cpu.regs.flags.c = true;
    }
    let hc = (((a &0xF) + (ac &0xF)) & 0x10) == 0x10;
    cpu.regs.a = a.wrapping_add(ac);
    cpu.regs.flags.z = cpu.regs.a == 0;
    cpu.regs.flags.n = false;
    cpu.regs.flags.h = hc;
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
    let hc = (*reg as i16 & 0xF) - (1 & 0xF) < 0;
    *reg = reg.wrapping_sub(1);
    flags.h = hc;
    flags.n = true;
    flags.z = *reg == 0;
}

fn inc_x(reg: &mut u8, flags: &mut FlagRegister) {
    let hc = (((*reg &0xF) + (1 &0xF)) & 0x10) == 0x10;
    *reg = reg.wrapping_add(1);
    flags.z = *reg == 0;
    flags.n = false;
    flags.h = hc;
}
pub fn opx04(cpu: &mut Cpu, mmu: &mut Mmu) {inc_x(&mut cpu.regs.b, &mut cpu.regs.flags)}
pub fn opx14(cpu: &mut Cpu, mmu: &mut Mmu) {inc_x(&mut cpu.regs.d, &mut cpu.regs.flags)}
pub fn opx24(cpu: &mut Cpu, mmu: &mut Mmu) {inc_x(&mut cpu.regs.h, &mut cpu.regs.flags)}
pub fn opx0C(cpu: &mut Cpu, mmu: &mut Mmu) {inc_x(&mut cpu.regs.c, &mut cpu.regs.flags)}
pub fn opx1C(cpu: &mut Cpu, mmu: &mut Mmu) {inc_x(&mut cpu.regs.e, &mut cpu.regs.flags)}
pub fn opx2C(cpu: &mut Cpu, mmu: &mut Mmu) {inc_x(&mut cpu.regs.l, &mut cpu.regs.flags)}
pub fn opx3C(cpu: &mut Cpu, mmu: &mut Mmu) {inc_x(&mut cpu.regs.a, &mut cpu.regs.flags)}

pub fn opx05(cpu: &mut Cpu, mmu: &mut Mmu) {dec_x(&mut cpu.regs.b, &mut cpu.regs.flags)}
pub fn opx15(cpu: &mut Cpu, mmu: &mut Mmu) {dec_x(&mut cpu.regs.d, &mut cpu.regs.flags)}
pub fn opx25(cpu: &mut Cpu, mmu: &mut Mmu) {dec_x(&mut cpu.regs.h, &mut cpu.regs.flags)}
pub fn opx0D(cpu: &mut Cpu, mmu: &mut Mmu) {dec_x(&mut cpu.regs.c, &mut cpu.regs.flags)}
pub fn opx1D(cpu: &mut Cpu, mmu: &mut Mmu) {dec_x(&mut cpu.regs.e, &mut cpu.regs.flags)}
pub fn opx2D(cpu: &mut Cpu, mmu: &mut Mmu) {dec_x(&mut cpu.regs.l, &mut cpu.regs.flags)}
pub fn opx3D(cpu: &mut Cpu, mmu: &mut Mmu) {dec_x(&mut cpu.regs.a, &mut cpu.regs.flags)}

pub fn opx35(cpu: &mut Cpu, mmu: &mut Mmu) {
    // DEC (HL)
    let addr = cpu.regs.hl() as usize;
    let mut hl = mmu.read(addr);
    let hc = (hl as i16 & 0xF) - (1 & 0xF) < 0;
    hl = hl.wrapping_sub(1);
    mmu.write(addr, hl);
    cpu.regs.flags.h = hc;
    cpu.regs.flags.n = true;
    cpu.regs.flags.z = hl == 0;
}
pub fn opx28(cpu: &mut Cpu, mmu: &mut Mmu) {
    // JR Z, r8
    // Jump relative if Z flag == true
    if cpu.regs.flags.z == false { return {} };
    let signed = cpu.immediate_u8(mmu) as i8;
    match signed > 0 {
        true => cpu.regs.pc += signed.abs() as usize,
        _ => cpu.regs.pc -= signed.abs() as usize
    };
}
pub fn opx08(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD (a16), SP
    // Load the 2-byte sp into 2-bytes of memory located at
    // the value specified by immediate_u16
    let a16 = cpu.immediate_u16(mmu) as usize;
    mmu.write_u16(a16, cpu.regs.sp as u16);
}
pub fn opxF3(cpu: &mut Cpu, mmu: &mut Mmu){
    mmu.ime = false;
}
pub fn opxFB(cpu: &mut Cpu, mmu: &mut Mmu){
    mmu.ime = true;
}
pub fn ld_x_y(regx: &mut u8, regy: u8) { *regx = regy }

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

pub fn opx54(cpu: &mut Cpu, mmu: &mut Mmu){}
pub fn opx5D(cpu: &mut Cpu, mmu: &mut Mmu){}
pub fn opx50(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.h, cpu.regs.b)}
pub fn opx51(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.h, cpu.regs.c)}
pub fn opx52(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.h, cpu.regs.d)}
pub fn opx53(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.h, cpu.regs.e)}
pub fn opx55(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.h, cpu.regs.l)}
pub fn opx57(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.h, cpu.regs.a)}
pub fn opx58(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.l, cpu.regs.b)}
pub fn opx59(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.l, cpu.regs.c)}
pub fn opx5A(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.l, cpu.regs.d)}
pub fn opx5B(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.l, cpu.regs.e)}
pub fn opx5C(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.l, cpu.regs.h)}
pub fn opx5F(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.l, cpu.regs.a)}

pub fn opx60(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.h, cpu.regs.b)}
pub fn opx61(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.h, cpu.regs.c)}
pub fn opx62(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.h, cpu.regs.d)}
pub fn opx63(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.h, cpu.regs.e)}
pub fn opx64(cpu: &mut Cpu, mmu: &mut Mmu){}
pub fn opx65(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.h, cpu.regs.l)}
pub fn opx66(cpu: &mut Cpu, mmu: &mut Mmu){
    let addr = cpu.regs.hl() as usize + 0xFF00;
    let val = mmu.read(addr) as u8;
    ld_x_y(&mut cpu.regs.h, val)}
pub fn opx67(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.h, cpu.regs.a)}
pub fn opx68(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.l, cpu.regs.b)}
pub fn opx69(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.l, cpu.regs.c)}
pub fn opx6A(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.l, cpu.regs.d)}
pub fn opx6B(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.l, cpu.regs.e)}
pub fn opx6C(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.l, cpu.regs.h)}
pub fn opx6D(cpu: &mut Cpu, mmu: &mut Mmu){}
pub fn opx6F(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.l, cpu.regs.a)}

pub fn opx78(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.a, cpu.regs.b)}
pub fn opx79(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.a, cpu.regs.c)}
pub fn opx7A(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.a, cpu.regs.d)}
pub fn opx7B(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.a, cpu.regs.e)}
pub fn opx7C(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.a, cpu.regs.h)}
pub fn opx7D(cpu: &mut Cpu, mmu: &mut Mmu){ld_x_y(&mut cpu.regs.a, cpu.regs.l)}
pub fn opx7F(cpu: &mut Cpu, mmu: &mut Mmu){}

pub fn opx0E(cpu: &mut Cpu, mmu: &mut Mmu){let v = cpu.immediate_u8(mmu); ld_x_y(&mut cpu.regs.c, v)}
pub fn opx1E(cpu: &mut Cpu, mmu: &mut Mmu){let v = cpu.immediate_u8(mmu); ld_x_y(&mut cpu.regs.e, v)}
pub fn opx2E(cpu: &mut Cpu, mmu: &mut Mmu){let v = cpu.immediate_u8(mmu); ld_x_y(&mut cpu.regs.l, v)}
pub fn opx3E(cpu: &mut Cpu, mmu: &mut Mmu){let v = cpu.immediate_u8(mmu); ld_x_y(&mut cpu.regs.a, v)}
pub fn opx06(cpu: &mut Cpu, mmu: &mut Mmu){let v = cpu.immediate_u8(mmu); ld_x_y(&mut cpu.regs.b, v)}
pub fn opx16(cpu: &mut Cpu, mmu: &mut Mmu){let v = cpu.immediate_u8(mmu); ld_x_y(&mut cpu.regs.d, v)}
pub fn opx26(cpu: &mut Cpu, mmu: &mut Mmu){let v = cpu.immediate_u8(mmu); ld_x_y(&mut cpu.regs.h, v)}

pub fn opxF0(cpu: &mut Cpu, mmu: &mut Mmu){
    let a = 0xFF00 + cpu.immediate_u8(mmu) as usize;
    ld_x_y(&mut cpu.regs.a, mmu.read(a))
}
fn sub_a_x(val: u8, cpu: &mut Cpu) {
    let hc = (cpu.regs.a as i16 & 0xF) - (val as i16 & 0xF) < 0;  // TODO: This probably isn't right.
    let c = cpu.regs.a < val;
    cpu.regs.a = cpu.regs.a.wrapping_sub(val);
    cpu.regs.flags.z = cpu.regs.a == 0;
    cpu.regs.flags.n = true;
    cpu.regs.flags.h = hc;
    cpu.regs.flags.c = c;
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
    cpu.regs.flags.z = a == val;
    cpu.regs.flags.c = a < val;
    cpu.regs.flags.n = true;
    cpu.regs.flags.h = (((a &0xF) + (val &0xF)) & 0x10) == 0x10;
}

pub fn opxB8(cpu: &mut Cpu, mmu: &mut Mmu) {cp_x(cpu.regs.b, cpu)}
pub fn opxB9(cpu: &mut Cpu, mmu: &mut Mmu) {cp_x(cpu.regs.c, cpu)}
pub fn opxBA(cpu: &mut Cpu, mmu: &mut Mmu) {cp_x(cpu.regs.d, cpu)}
pub fn opxBB(cpu: &mut Cpu, mmu: &mut Mmu) {cp_x(cpu.regs.e, cpu)}
pub fn opxBC(cpu: &mut Cpu, mmu: &mut Mmu) {cp_x(cpu.regs.h, cpu)}
pub fn opxBD(cpu: &mut Cpu, mmu: &mut Mmu) {cp_x(cpu.regs.l, cpu)}
pub fn opxBF(cpu: &mut Cpu, mmu: &mut Mmu) {cp_x(cpu.regs.a, cpu)}
pub fn opxBE(cpu: &mut Cpu, mmu: &mut Mmu) {let a=cpu.regs.hl() as usize; cp_x(mmu.read(a), cpu)}

pub fn opx70(cpu: &mut Cpu, mmu: &mut Mmu) {add_a_x(cpu.regs.b, cpu)}
pub fn opx71(cpu: &mut Cpu, mmu: &mut Mmu) {add_a_x(cpu.regs.c, cpu)}
pub fn opx72(cpu: &mut Cpu, mmu: &mut Mmu) {add_a_x(cpu.regs.d, cpu)}
pub fn opx73(cpu: &mut Cpu, mmu: &mut Mmu) {add_a_x(cpu.regs.e, cpu)}
pub fn opx74(cpu: &mut Cpu, mmu: &mut Mmu) {add_a_x(cpu.regs.h, cpu)}
pub fn opx75(cpu: &mut Cpu, mmu: &mut Mmu) {add_a_x(cpu.regs.l, cpu)}
pub fn add_a_x(val: u8, cpu: &mut Cpu) {
    let c = cpu.regs.a.checked_add(val).is_none();
    cpu.regs.flags.h = (((cpu.regs.a & 0xF) + (val & 0xF)) & 0x10) == 0x10;
    cpu.regs.a = cpu.regs.a.wrapping_add(val);
    cpu.regs.flags.z = val == val;
    cpu.regs.flags.c = c;
    cpu.regs.flags.n = false;
}

pub fn opx2A(cpu: &mut Cpu, mmu: &mut Mmu) {}

pub fn opx12(cpu: &mut Cpu, mmu: &mut Mmu) {
    let addr = mmu.read(cpu.regs.de() as usize) as usize;
    mmu.write(addr, cpu.regs.a);
}
