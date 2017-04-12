use std::fmt;

use cpu::Cpu;
use mmu::Mmu;
use registers::FlagRegister;

use bitty::BitFlags;

pub struct Operation {
    pub dis: &'static str,
    pub code: u16,
    pub func: Box<Fn(&mut Cpu, &mut Mmu)>,
    pub size: usize,   // Number of bytes including opcode
    pub cycles: u8,
}

impl Operation {
    pub fn new(code: u16, func: fn(&mut Cpu, &mut Mmu), size: usize, cycles: u8, dis: &'static str)
    -> Operation
{
        Operation {
            dis: dis,
            code: code,
            func: Box::new(func),
            size: size,
            cycles: cycles,
        }
    }
}

impl fmt::Debug for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:04X} -> {}", self.code, self.dis)
    }
}

pub fn get_operation(code: u16) -> Operation {
    let prefix = code >> 8;
    let scode = code & 0x00F0;
    let lcode = code & 0x000F;

    match prefix {
        0x00 => match scode {   // No Prefix
            0x00 => match lcode {
                0x00 => Operation::new(code, opx00, 1, 4,  "NOP"),
                0x04 => Operation::new(code, opx04, 1, 4,  "INC B"),
                0x05 => Operation::new(code, opx05, 1, 4,  "DEC B"),
                0x06 => Operation::new(code, opx06, 2, 8,  "LD B, d8"),
                0x08 => Operation::new(code, opx08, 3, 20, "LD (a16), SP"),
                0x0C => Operation::new(code, opx0C, 1, 4,  "INC C"),
                0x0D => Operation::new(code, opx0D, 1, 4,  "DEC C"),
                0x0E => Operation::new(code, opx0E, 2, 8,  "LD C, d8"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            0x10 => match lcode {
                0x01 => Operation::new(code, opx11, 3, 12,  "LD DE, d16"),
                0x03 => Operation::new(code, opx13, 1, 8,  "INC DE"),
                0x04 => Operation::new(code, opx14, 1, 4,  "INC D"),
                0x05 => Operation::new(code, opx15, 1, 4,  "DEC D"),
                0x06 => Operation::new(code, opx16, 2, 8,  "LD D, d8"),
                0x07 => Operation::new(code, opx17, 1, 4,  "RLA"),
                0x08 => Operation::new(code, opx18, 2, 8,  "JR r8"),
                0x0A => Operation::new(code, opx1A, 1, 8,  "LD A, (DE)"),
                0x0C => Operation::new(code, opx1C, 1, 4,  "INC E"),
                0x0D => Operation::new(code, opx1D, 1, 4,  "DEC E"),
                0x0E => Operation::new(code, opx1E, 2, 8,  "LD E, d8"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            0x20 => match lcode {
                0x00 => Operation::new(code, opx20, 2, 12, "JR NZ, r8"),
                0x01 => Operation::new(code, opx21, 3, 12, "LD HL, d16"),
                0x02 => Operation::new(code, opx22, 3, 12, "LD (HL+), A"),
                0x03 => Operation::new(code, opx23, 1,  8, "INC HL"),
                0x04 => Operation::new(code, opx24, 1,  4, "INC H"),
                0x05 => Operation::new(code, opx25, 1,  4, "DEC H"),
                0x06 => Operation::new(code, opx26, 2, 8,  "LD H, d8"),
                0x08 => Operation::new(code, opx28, 2, 12, "JR Z, r8"),
                0x0C => Operation::new(code, opx2C, 1,  4, "INC E"),
                0x0D => Operation::new(code, opx2D, 1,  4, "DEC L"),
                0x0E => Operation::new(code, opx2E, 2, 8,  "LD L, d8"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            0x30 => match lcode {
                0x01 => Operation::new(code, opx31, 3, 12, "LD SP, d16"),
                0x02 => Operation::new(code, opx32, 1,  8, "LD (HL-), A"),
                0x05 => Operation::new(code, opx35, 1, 12, "DEC (HL)"),
                0x0C => Operation::new(code, opx3C, 1,  4, "INC A"),
                0x0D => Operation::new(code, opx3D, 1,  4, "DEC A"),
                0x0E => Operation::new(code, opx3E, 2,  8, "LD A, d8"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            0x40 => match lcode {
                0x00 => Operation::new(code, opx40, 1, 4,  "LD B, B"),
                0x01 => Operation::new(code, opx41, 1, 4,  "LD B, C"),
                0x02 => Operation::new(code, opx42, 1, 4,  "LD B, D"),
                0x03 => Operation::new(code, opx43, 1, 4,  "LD B, E"),
                0x04 => Operation::new(code, opx44, 1, 4,  "LD B, H"),
                0x05 => Operation::new(code, opx45, 1, 4,  "LD B, L"),
                0x06 => Operation::new(code, opx46, 1, 8,  "LD B, (HL)"),
                0x07 => Operation::new(code, opx47, 1, 4,  "LD B, A"),
                0x08 => Operation::new(code, opx48, 1, 4,  "LD C, B"),
                0x09 => Operation::new(code, opx49, 1, 4,  "LD C, C"),
                0x0A => Operation::new(code, opx4A, 1, 4,  "LD C, D"),
                0x0B => Operation::new(code, opx4B, 1, 4,  "LD C, E"),
                0x0C => Operation::new(code, opx4C, 1, 4,  "LD C, H"),
                0x0D => Operation::new(code, opx4D, 1, 4,  "LD C, L"),
                0x0E => Operation::new(code, opx4E, 1, 8,  "LD C, (HL)"),
                0x0F => Operation::new(code, opx4F, 1, 4,  "LD C, A"),
                _   => Operation::new(code, unimplemented, 0, 0, "(unimplemented"),
            },
            0x50 => match lcode {
                0x00 => Operation::new(code, opx50, 1, 4,  "LD H, B"),
                0x01 => Operation::new(code, opx51, 1, 4,  "LD H, C"),
                0x02 => Operation::new(code, opx52, 1, 4,  "LD H, D"),
                0x03 => Operation::new(code, opx53, 1, 4,  "LD H, E"),
                0x04 => Operation::new(code, opx54, 1, 4,  "LD H, H"),
                0x05 => Operation::new(code, opx55, 1, 4,  "LD H, L"),
                0x07 => Operation::new(code, opx57, 1, 4,  "LD H, A"),
                0x08 => Operation::new(code, opx58, 1, 4,  "LD H, B"),
                0x09 => Operation::new(code, opx59, 1, 4,  "LD H, C"),
                0x0A => Operation::new(code, opx5A, 1, 4,  "LD H, D"),
                0x0B => Operation::new(code, opx5B, 1, 4,  "LD H, E"),
                0x0C => Operation::new(code, opx5C, 1, 4,  "LD H, H"),
                0x0D => Operation::new(code, opx5D, 1, 4,  "LD H, L"),
                0x0F => Operation::new(code, opx5F, 1, 4,  "LD H, A"),
                _   => Operation::new(code, unimplemented, 0, 0, "(unimplemented"),
            },
            0x60 => match lcode {
                0x00 => Operation::new(code, opx60, 1, 4,  "LD H, B"),
                0x01 => Operation::new(code, opx61, 1, 4,  "LD H, C"),
                0x02 => Operation::new(code, opx62, 1, 4,  "LD H, D"),
                0x03 => Operation::new(code, opx63, 1, 4,  "LD H, E"),
                0x04 => Operation::new(code, opx64, 1, 4,  "LD H, H"),
                0x05 => Operation::new(code, opx65, 1, 4,  "LD H, L"),
                0x07 => Operation::new(code, opx67, 1, 4,  "LD H, A"),
                0x08 => Operation::new(code, opx68, 1, 4,  "LD H, B"),
                0x09 => Operation::new(code, opx69, 1, 4,  "LD H, C"),
                0x0A => Operation::new(code, opx6A, 1, 4,  "LD H, D"),
                0x0B => Operation::new(code, opx6B, 1, 4,  "LD H, E"),
                0x0C => Operation::new(code, opx6C, 1, 4,  "LD H, H"),
                0x0D => Operation::new(code, opx6D, 1, 4,  "LD H, L"),
                0x0F => Operation::new(code, opx6F, 1, 4,  "LD H, A"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            0x70 => match lcode {
                0x07 => Operation::new(code, opx77, 1, 8, "LD (HL), A"),
                0x08 => Operation::new(code, opx78, 1, 4, "LD A, B"),
                0x09 => Operation::new(code, opx79, 1, 4, "LD A, C"),
                0x0A => Operation::new(code, opx7A, 1, 4, "LD A, D"),
                0x0B => Operation::new(code, opx7B, 1, 4, "LD A, E"),
                0x0C => Operation::new(code, opx7C, 1, 4, "LD A, H"),
                0x0D => Operation::new(code, opx7D, 1, 4, "LD A, L"),
                0x0F => Operation::new(code, opx7F, 1, 4, "LD A, A"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            0x80 => match lcode {
                0x00 => Operation::new(code, opx70, 1, 4,  "ADD A, B"),
                0x01 => Operation::new(code, opx71, 1, 4,  "ADD A, C"),
                0x02 => Operation::new(code, opx72, 1, 4,  "ADD A, D"),
                0x03 => Operation::new(code, opx73, 1, 4,  "ADD A, E"),
                0x04 => Operation::new(code, opx74, 1, 4,  "ADD A, H"),
                0x05 => Operation::new(code, opx75, 1, 4,  "ADD A, L"),
                0x06 => Operation::new(code, opx78, 1, 8,  "ADD A, (HL)"),
                0x07 => Operation::new(code, opx77, 1, 4,  "ADD A, A"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            0x90 => match lcode {
                0x00 => Operation::new(code, opx90, 1, 4, "SUB B"),
                0x01 => Operation::new(code, opx91, 1, 4, "SUB C"),
                0x02 => Operation::new(code, opx92, 1, 4, "SUB D"),
                0x03 => Operation::new(code, opx93, 1, 4, "SUB e"),
                0x04 => Operation::new(code, opx94, 1, 4, "SUB H"),
                0x05 => Operation::new(code, opx95, 1, 4, "SUB L"),
                0x07 => Operation::new(code, opx97, 1, 4, "SUB A"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            0xA0 => match lcode {
                0x00 => Operation::new(code, opxA0, 1, 4, "AND B"),
                0x07 => Operation::new(code, opxA7, 1, 4, "AND A"),
                0x0F => Operation::new(code, opxAF, 1, 4, "XOR A, A"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            0xB0 => match lcode {
                0x08 => Operation::new(code, opxB8, 1, 4, "CP B"),
                0x09 => Operation::new(code, opxB9, 1, 4, "CP C"),
                0x0A => Operation::new(code, opxBA, 1, 4, "CP D"),
                0x0B => Operation::new(code, opxBB, 1, 4, "CP E"),
                0x0C => Operation::new(code, opxBC, 1, 4, "CP H"),
                0x0D => Operation::new(code, opxBD, 1, 4, "CP L"),
                0x0E => Operation::new(code, opxBE, 1, 8, "CP (HL)"),
                0x0F => Operation::new(code, opxBF, 1, 4, "CP A"),
                _    => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            0xC0 => match lcode {
                0x01 => Operation::new(code, opxC1, 1, 12, "POP BC"),
                0x05 => Operation::new(code, opxC5, 1, 16, "PUSH BC"),
                0x09 => Operation::new(code, opxC9, 0, 16, "RET"),
                0x0D => Operation::new(code, opxCD, 0, 24, "CALL a16"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            0xE0 => match lcode {
                0x00 => Operation::new(code, opxE0, 2, 12, "LDH (a8), A"),
                0x02 => Operation::new(code, opxE2, 1, 8, "LD (C), A"),
                0x0A => Operation::new(code, opxEA, 3, 16, "LD (a16), A"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            0xF0 => match lcode {
                0x00 => Operation::new(code, opxF0, 2, 12, "LDH A, (a8)"),
                0x03 => Operation::new(code, opxF3, 1, 4, "DI"),
                0x0B => Operation::new(code, opxFB, 1, 4, "EI"),
                0x0E => Operation::new(code, opxFE, 2, 8, "CP d8"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
        },
        0xCB => match scode {   // CB Prefix
            0x10 => match lcode {
                0x01 => Operation::new(code, cbx11, 2, 8, "RL C"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            0x70 => match lcode {
                0x0C => Operation::new(code, cbx7C, 2, 8, "BIT 7, H"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
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
// pub fn opxCE(cpu: &mut Cpu, mmu: &mut Mmu) {
//     // ADC A, d8
//     // Z 0 H C
//     let ac = cpu.regs.flags.c as u8 + cpu.immediate_u8(mmu);
//     let a = cpu.regs.a;
//     let hc = (((a &0xF) + (ac &0xF)) & 0x10) == 0x10;
//     cpu.regs.a = a.wrapping_add(ac);
//     cpu.regs.flags.z = cpu.regs.a == 0;
//     cpu.regs.flags.n = false;
// }
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

pub fn opx70(cpu: &mut Cpu, mmu: &mut Mmu) {cp_x(cpu.regs.b, cpu)}
pub fn opx71(cpu: &mut Cpu, mmu: &mut Mmu) {cp_x(cpu.regs.c, cpu)}
pub fn opx72(cpu: &mut Cpu, mmu: &mut Mmu) {cp_x(cpu.regs.d, cpu)}
pub fn opx73(cpu: &mut Cpu, mmu: &mut Mmu) {cp_x(cpu.regs.e, cpu)}
pub fn opx74(cpu: &mut Cpu, mmu: &mut Mmu) {cp_x(cpu.regs.h, cpu)}
pub fn opx75(cpu: &mut Cpu, mmu: &mut Mmu) {cp_x(cpu.regs.l, cpu)}
pub fn add_a_x(val: u8, cpu: &mut Cpu) {
    let c = cpu.regs.a.checked_add(val).is_none();
    cpu.regs.flags.h = (((cpu.regs.a & 0xF) + (val & 0xF)) & 0x10) == 0x10;
    cpu.regs.a = cpu.regs.a.wrapping_add(val);
    cpu.regs.flags.z = val == val;
    cpu.regs.flags.c = c;
    cpu.regs.flags.n = false;
}
