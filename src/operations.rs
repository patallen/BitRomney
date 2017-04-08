use std::fmt;

use cpu::Cpu;
use mmu::Mmu;

use bitty::{LittleEndian, BitFlags};

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
                0x02 => Operation::new(code, opx22, 3, 12, "LD (HL+), A"),
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
                // 0x0F => Operation::new(code, opxAF, 1, 4, "AND B"),
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
                0x02 => Operation::new(code, opxE2, 1, 8, "LD (C), A"),  // TODO: Check Size again ( one site says 1 other, 2)
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
    // JR RZ, r8
    // Jump Relative if not zero (signed immediate 8-bit)
    if cpu.regs.flags.z == false { return {} };
    let to_jump = cpu.immediate_u8(mmu);
    match cpu.regs.flags.n {
        true => { cpu.regs.pc -= cpu.immediate_u8(mmu) as usize },
        false => { cpu.regs.pc += cpu.immediate_u8(mmu) as usize },
    };
}
pub fn opx21(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD HL, d16
    // Load immediate 16-bit into HL register
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

pub fn opx32(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD (HL-), A
    // Load the value of register A into mem address HL
    // Decrement HL
    let addr = cpu.regs.hl() as usize;
    let a = cpu.regs.a;
    mmu.write(addr, a);
    cpu.regs.set_hl((addr - 1) as u16);
}
pub fn opx31(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD SP, d16
    // Load immediate 16-bit into Stack Pointer
    let sp = cpu.immediate_u16(mmu);
    cpu.regs.sp = sp as usize;
}
pub fn opxAF(cpu: &mut Cpu, mmu: &mut Mmu) {
    // XOR A
    // A ^= A and set zero flag if necessary
    let a = cpu.regs.a;
    cpu.regs.a ^= a;
    match cpu.regs.a {
        0 => cpu.regs.flags.z = true,
        _ => cpu.regs.flags.z = false,
    };
}
pub fn opx0E(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD C, d8
    // Load immediate 8-bit into register C
    let next = cpu.immediate_u8(mmu);
    cpu.regs.c = next;
}
pub fn opx3E(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD A, d8
    // Load immediate 8-bit into register A
    let next = cpu.immediate_u8(mmu);
    cpu.regs.a = next;
}
pub fn opxE2(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD (C), A
    // Load value from register A into mem at address specified by register C
    let c = cpu.regs.c as usize + 0xFF00;
    mmu.write(c as usize, cpu.regs.a);
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
    // "LD A, (DE)"
    // Load value of memory at address specified in DE into register A
    let addr = mmu.read(cpu.regs.de() as usize);
    println!("value: {:04X}", addr);
    cpu.regs.a = addr;
}

pub fn opxCD(cpu: &mut Cpu, mmu: &mut Mmu) {
    // Call a16
    // Set pc to value of immediate 16-bit
    // push both bytes of pc onto the stack
    // increment the sp by two
    let pc = cpu.regs.pc;
    let nn = cpu.immediate_u16(mmu);
    mmu.write(cpu.regs.sp as usize, (pc as u16).get_msb());
    cpu.regs.inc_sp();
    mmu.write(cpu.regs.sp as usize, (pc as u16).get_lsb());
    cpu.regs.inc_sp();
    cpu.pc = (nn as usize) - 3;
}

pub fn opx4F(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD C, A
    // Load the value of register A into register C
    let a = cpu.regs.a;
    cpu.regs.c = a;
}
pub fn opx06(cpu: &mut Cpu, mmu: &mut Mmu) {
    // LD B, d8
    // Load the value of immediate 8-bit into register B
    let d8 = cpu.immediate_u8(mmu);
    cpu.regs.b = d8;
}

pub fn opxC5(cpu: &mut Cpu, mmu: &mut Mmu) {
    // PUSH BC
    // Put both bytes of BC onto the stack
    // Increment the SP by two
    let bc = cpu.regs.bc();
    mmu.write(cpu.regs.sp, bc.get_msb());
    cpu.regs.inc_sp();
    mmu.write(cpu.regs.sp, bc.get_lsb());
    cpu.regs.inc_sp();
}

pub fn opxC1(cpu: &mut Cpu, mmu: &mut Mmu) {
    // POP BC
    // Remove the top two bytes from the stack and place in BC
    // Decrement the stack pointer by two
    let c = mmu.read(cpu.regs.sp);
    cpu.regs.dec_sp();
    let b = mmu.read(cpu.regs.sp);
    cpu.regs.dec_sp();
    let mut bc: u16 = 0;
    bc.set_msb(b);
    bc.set_lsb(c);
    cpu.regs.set_bc(bc);
}

pub fn opx17(cpu: &mut Cpu, mmu: &mut Mmu) {
    // RLA
    // Rotate A left one bit.
    // Place old MSB into carry flag
    // Place old Carry flag into bit 0 of A
    let msb = cpu.regs.a >> 7;
    cpu.regs.a = cpu.regs.a << 1;
    cpu.regs.a |= cpu.regs.flags.c as u8;
    cpu.regs.flags.c = msb == 1;
}

pub  fn cbx7C(cpu: &mut Cpu, mmu: &mut Mmu) {
    // BIT 7, H
    // Set Z flag if bit no. 7 of register H is zero
    // set N flag to 0 and H flag to 1
    cpu.regs.flags.z = cpu.regs.h.get_bit(7) == 0;
    cpu.regs.flags.n = false;
    cpu.regs.flags.h = true;
}

pub  fn cbx11(cpu: &mut Cpu, mmu: &mut Mmu) {
    // RL C
    // Rotate register C one bit to the left
    // MSB goes into carry flag
    // carry flag goes into lsb of C
    // if the result is zero, set the zero flag to 1 else 0
    let msb = cpu.regs.c >> 7;
    let carry = cpu.regs.flags.c as u8;
    cpu.regs.c = cpu.regs.c << 1;
    cpu.regs.c |= carry;
    cpu.regs.flags.c = msb == 1;

    cpu.regs.flags.z = match cpu.regs.c {
        0 => true,
        _ => false,
    };
}
