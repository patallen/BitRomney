use std::fmt;

use cpu::Cpu;
use mmu::Mmu;
use registers::Reg;

use bitty::{flag, LittleEndian, BitFlags};

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
        write!(f, "0x{:04X} -> {}.", self.code, self.dis)
    }
}

pub fn get_operation(code: u16) -> Operation {
    let prefix = code >> 8;
    let scode = code & 0x00F0;
    let lcode = code & 0x000F;

    match prefix {
        0x00 => match scode {   // No Prefix
            0x00 => match lcode {
                0x06 => Operation::new(code, opx06, 2, 8,  "LD B, d8"),
                0x0E => Operation::new(code, opx0E, 2, 8,  "LD C, d8"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            0x10 => match lcode {
                0x01 => Operation::new(code, opx11, 3, 12,  "LD DE, d16"),
                0x07 => Operation::new(code, opx17, 1, 4,  "RLA"),
                0x0A => Operation::new(code, opx1A, 1, 8,  "LD A, (DE)"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            0x20 => match lcode {
                0x00 => Operation::new(code, opx20, 2, 12, "JR NZ, r8"),
                0x01 => Operation::new(code, opx21, 3, 12, "LD HL, d16"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            0x30 => match lcode {
                0x01 => Operation::new(code, opx31, 3, 12, "LD SP, d16"),
                0x02 => Operation::new(code, opx32, 1, 8,  "LD (HL-), A"),
                0x0E => Operation::new(code, opx3E, 2, 8,  "LD A, d8"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            0x40 => match lcode {
                0x0F => Operation::new(code, opx4F, 1, 4,  "LD C, A"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            0x70 => match lcode {
                0x07 => Operation::new(code, opx77, 1, 8, "LD (HL), A"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            0xA0 => match lcode {
                0x0F => Operation::new(code, opxAF, 1, 4, "AND B"),
                0x0F => Operation::new(code, opxAF, 1, 4, "XOR A, A"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            0xC0 => match lcode {
                0x01 => Operation::new(code, opxC1, 1, 12, "POP BC"),
                0x05 => Operation::new(code, opxC5, 1, 16, "PUSH BC"),
                0x0D => Operation::new(code, opxCD, 3, 24, "CALL a16"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            0xE0 => match lcode {
                0x00 => Operation::new(code, opxE0, 2, 12, "LDH (a8), A"),
                0x02 => Operation::new(code, opxE2, 2, 8, "LD (C), A"),
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
pub fn opx20(cpu: &mut Cpu, mmu: &mut Mmu) {
    let flags = cpu.flags();
    let z = flags.get_bit(flag::Z);
    let to_jump = cpu.immediate_u8(mmu);
    let sign = cpu.flags().get_bit(flag::N);
    if z == 1 {
        cpu.pc = match sign {
            0 => cpu.pc + to_jump as usize,
            1 => cpu.pc - to_jump as usize,
            _ => panic!("Invalid sign from Z flag... not really possible.")
        };
    }
    println!("Sign: {}, Value {}", sign, to_jump);

}
pub fn opx21(cpu: &mut Cpu, mmu: &mut Mmu) {
    let new = cpu.immediate_u16(mmu);
    cpu.regs.set_u16(Reg::HL, new);
}

pub fn opx32(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl = cpu.regs.get_u16(Reg::HL);
    let new = hl.wrapping_sub(cpu.regs.get_u8(Reg::A) as u16);
    cpu.regs.set_u16(Reg::HL, new);
}
pub fn opx31(cpu: &mut Cpu, mmu: &mut Mmu) {
    let new = cpu.immediate_u16(mmu);
    cpu.regs.set_u16(Reg::SP, new);
}
pub fn opxAF(cpu: &mut Cpu, mmu: &mut Mmu) {
    let new = cpu.regs.get_u16(Reg::AF) & 0xFF00;
    cpu.regs.set_u16(Reg::AF, new);
}
pub fn opx0E(cpu: &mut Cpu, mmu: &mut Mmu) {
    let next = cpu.immediate_u8(mmu);
    cpu.regs.set_u8(Reg::C, next);
}
pub fn opx3E(cpu: &mut Cpu, mmu: &mut Mmu) {
    let val = cpu.immediate_u8(mmu);
    cpu.regs.set_u8(Reg::A, val);
}
pub fn opxE2(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.regs.get_u8(Reg::C);
    let a = cpu.regs.get_u8(Reg::A);
    mmu.write(c as usize, a);
}
pub fn gload(cpu: &mut Cpu, mmu: &mut Mmu) {

}
pub fn opx77(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD (HL), A
    let hl = cpu.regs.get_u16(Reg::HL) as usize;
    let a = cpu.regs.get_u8(Reg::A);
    mmu.write(hl, a);
}
pub fn opxE0(cpu: &mut Cpu, mmu: &mut Mmu) {
    let addr = (0xFF00 + cpu.immediate_u8(mmu) as u16) as usize;
    mmu.write(addr, cpu.regs.get_u8(Reg::A));
}
pub fn opx11(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD DE, d16
    let d16 = cpu.immediate_u16(mmu);
    cpu.regs.set_u16(Reg::DE, d16);
}
pub fn opx1A(cpu: &mut Cpu, mmu: &mut Mmu) {
    // "LD A, (DE)"
    let addr = cpu.regs.get_u16(Reg::DE) as u8;
    println!("value: {:04X}", addr);
    cpu.regs.set_u8(Reg::A, addr);
}

pub fn opxCD(cpu: &mut Cpu, mmu: &mut Mmu) {
    // Call a16
    let pc = cpu.pc;
    let nn = cpu.immediate_u16(mmu);
    mmu.write(cpu.regs.get_u16(Reg::SP) as usize, (pc & 0x00FF) as u8);
    cpu.regs.inc_sp();
    mmu.write(cpu.regs.get_u16(Reg::SP) as usize, (pc >> 8) as u8);
    cpu.regs.inc_sp();
    cpu.pc = (nn as usize) - 3;
}

pub fn opx4F(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD C, A
    let a = cpu.regs.get_u8(Reg::A);
    cpu.regs.set_u8(Reg::C, a);
}
pub fn opx06(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD B, d8
    let d8 = cpu.immediate_u8(mmu);
    cpu.regs.set_u8(Reg::B, d8);
}

pub fn opxC5(cpu: &mut Cpu, mmu: &mut Mmu) {
    // PUSH BC
    let bc = cpu.regs.get_u16(Reg::BC);
    mmu.write(cpu.regs.get_u16(Reg::SP) as usize, bc.get_msb());
    cpu.regs.inc_sp();
    mmu.write(cpu.regs.get_u16(Reg::SP) as usize, bc.get_lsb());
    cpu.regs.inc_sp();
}

pub fn opxC1(cpu: &mut Cpu, mmu: &mut Mmu) {
    // POP BC
    let b = mmu.read(cpu.regs.get_u16(Reg::SP) as usize);
    cpu.regs.dec_sp();
    let c = mmu.read(cpu.regs.get_u16(Reg::SP) as usize);
    cpu.regs.dec_sp();
    let mut bc: u16 = 0;
    bc.set_msb(b);
    bc.set_lsb(c);
    cpu.regs.set_u16(Reg::BC, bc);
}

pub fn opx17(cpu: &mut Cpu, mmu: &mut Mmu) {
    // RLA
    // Rotate A left one bit
    let mut rega = cpu.regs.get_u8(Reg::A);
    let msb = rega >> 7;
    let mut flags = cpu.flags();
    let carry = flags.get_bit(flag::C);
    rega = rega << 1;
    rega |= carry;
    flags.set_bit(flag::C, msb);
    cpu.regs.set_u8(Reg::A, rega);
    cpu.regs.set_u8(Reg::F, flags);
}

pub  fn cbx7C(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut flags = cpu.flags();
    let hbit = cpu.regs.get_u8(Reg::H).get_bit(7);
    flags.set_bit(flag::N as usize, hbit);
    cpu.regs.set_u8(Reg::F, flags);
}

pub  fn cbx11(cpu: &mut Cpu, mmu: &mut Mmu) {
    // RL C
    // Rotate register C one bit to the left
    // MSB goes into carry flag
    // cary flag goes into lsb of C
    // if the result is zero, set the zero flag to 1 else 0
    let mut regc = cpu.regs.get_u8(Reg::C);
    let msb = regc >> 7;
    let mut flags = cpu.flags();
    let carry = flags.get_bit(flag::C);
    regc = regc << 1;
    regc |= carry;
    flags.set_bit(flag::C, msb);
    cpu.regs.set_u8(Reg::C, regc);
    match regc {
        0 => { flags.set_bit(flag::Z, 1)},
        _ => { flags.set_bit(flag::Z, 0)},
    };
    cpu.regs.set_u8(Reg::F, flags);
}
