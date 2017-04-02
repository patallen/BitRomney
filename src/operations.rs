use std::fmt;

use cpu::Cpu;
use mmu::Mmu;


pub struct Operation<'op> {
    pub dis: &'op str,
    pub code: u8,
    pub func: fn(&mut Cpu, &mut Mmu),
    pub size: usize,   // Number of bytes including opcode
}

impl<'op> Operation<'op> {
    pub fn new(code: u8, func: fn(&mut Cpu, &mut Mmu), size: usize, dis: &'op str)
    -> Operation
{
        Operation {
            dis: dis,
            code: code,
            func: func,
            size: size,
        }
    }
}

impl<'op> fmt::Debug for Operation<'op> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:02X} -> {}.", self.code, self.dis)
    }
}

pub fn get_operation<'op>(code: u8, cb: bool) -> Operation<'op> {
    let scode = code & 0xF0;
    let lcode = code & 0x0F;

    match scode {
        0x20 => match lcode {
            0x01 => Operation::new(code, opx21, 3, "x21"),
            _    => Operation::new(code, opx00, 0, "unimplemented"),

        },
        0x30 => match lcode {
            0x01 => Operation::new(code, opx31, 3, "x31"),
            0x02 => Operation::new(code, opx32, 1, "x32"),
            _    => Operation::new(code, opx00, 0, "unimplemented"),

        },
        0xA0 => match lcode {
            0x0F => Operation::new(code, opxAF, 1, "xAF"),
            _    => Operation::new(code, opx00, 0, "unimplemented"),
        },
        _ => Operation::new(code, opx00, 0, "unimplemented"),
    }
}

pub fn opx00(cpu: &mut Cpu, mmu: &mut Mmu) {}
pub fn opx21(cpu: &mut Cpu, mmu: &mut Mmu) {
    let a = mmu.read(cpu.pc + 1);
    let b = mmu.read(cpu.pc + 2);
    cpu.hl = ((b as u16) << 8) | a as u16;
}
pub fn opx32(cpu: &mut Cpu, mmu: &mut Mmu) {
    let a = cpu.af & 0x00FF;
    cpu.hl = cpu.hl.wrapping_sub(a);
}
pub fn opx31(cpu: &mut Cpu, mmu: &mut Mmu) {
    let a = mmu.read(cpu.pc + 1);
    let b = mmu.read(cpu.pc + 2);
    cpu.sp = ((b as u16) << 8) | a as u16;
}
pub fn opxAF(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.af &= 0xFF00;
}

