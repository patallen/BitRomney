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
                0x0E => Operation::new(code, opx0E, 2, 8,  "LD C, d8"),
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
            0x70 => match lcode {
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            0xA0 => match lcode {
                0x0F => Operation::new(code, opxAF, 1, 4, "XOR A, A"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            0xE0 => match lcode {
                0x02 => Operation::new(code, opxE2, 2, 8, "LD (C), A"),
                _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
            },
            _   => Operation::new(code, unimplemented, 0, 0, "unimplemented"),
        },
        0xCB => match scode {   // CB Prefix
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

pub  fn cbx7C(cpu: &mut Cpu, mmu: &mut Mmu) {
    // Test bit 7 in register H & if set, set z to 1
    // always set flags N=0 and H=1
    let mut flags = cpu.flags();
    let hbit = cpu.regs.get_u8(Reg::H).get_bit(7);
    flags.set_bit(flag::N as usize, hbit);
    cpu.regs.set_u8(Reg::F, flags);
}

pub fn opx0E(cpu: &mut Cpu, mmu: &mut Mmu) {
    let next = cpu.immediate_u8(mmu);
    cpu.regs.set_u8(Reg::C, next);
}

pub fn opx3E(cpu: &mut Cpu, mmu: &mut Mmu) {
    // load A, d8
    let val = cpu.immediate_u8(mmu);
    cpu.regs.set_u8(Reg::A, val);
}

pub fn opxE2(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD (C), A (LD the value of A into location specified in C)
    let c = cpu.regs.get_u8(Reg::C);
    let a = cpu.regs.get_u8(Reg::A);
    mmu.write(c as usize, a);
}

pub fn gload(cpu: &mut Cpu, mmu: &mut Mmu) {

}
