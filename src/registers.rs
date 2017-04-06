use std::fmt;
use bitty::LittleEndian;


pub enum Reg {
    BC,
    DE,
    HL,
    SP,
    AF,
    B,
    C,
    D,
    E,
    H,
    L,
    S,
    P,
    A,
    F,
}

pub struct Registers {
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    af: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            af: 0,
        }
    }
    pub fn get_u16(&self, register: Reg) -> u16 {
        match register {
            Reg::BC => { self.bc },
            Reg::DE => { self.de },
            Reg::HL => { self.hl },
            Reg::SP => { self.sp },
            Reg::AF => { self.af },
            _ => panic!("Cannot pass an 8-bit register to 'get_u16'"),
        }
    }
    pub fn get_u8(&self, register: Reg) -> u8 {
       match register {
            Reg::B => { self.bc.get_msb() },
            Reg::C => { self.bc.get_lsb() },
            Reg::D => { self.de.get_msb() },
            Reg::E => { self.de.get_lsb() },
            Reg::H => { self.hl.get_msb() },
            Reg::L => { self.hl.get_lsb() },
            Reg::S => { self.sp.get_msb() },
            Reg::P => { self.sp.get_lsb() },
            Reg::A => { self.af.get_msb() },
            Reg::F => { self.af.get_lsb() },
            _ => panic!("Cannot pass a 16-bit register to 'get_u8'"),
       }
    }
    pub fn set_u16(&mut self, register: Reg, value: u16) {
        match register {
            Reg::BC => { self.bc = value },
            Reg::DE => { self.de = value },
            Reg::HL => { self.hl = value },
            Reg::SP => { self.sp = value },
            Reg::AF => { self.af = value },
            _ => panic!("Cannot pass an 8-bit register to 'get_u16'"),
        };
    }
    pub fn set_u8(&mut self, register: Reg, value: u8) {
       match register {
            Reg::B => { self.bc.set_msb(value) },
            Reg::C => { self.bc.set_lsb(value) },
            Reg::D => { self.de.set_msb(value) },
            Reg::E => { self.de.set_lsb(value) },
            Reg::H => { self.hl.set_msb(value) },
            Reg::L => { self.hl.set_lsb(value) },
            Reg::S => { self.sp.set_msb(value) },
            Reg::P => { self.sp.set_lsb(value) },
            Reg::A => { self.af.set_msb(value) },
            Reg::F => { self.af.set_lsb(value) },
            _ => panic!("Cannot pass a 16-bit register to 'get_u8'"),
       };
    }
    pub fn inc_sp(&mut self) {
        self.sp -= 1;  // "Increment is subraction because the stack is top-down"
    }
    pub fn dec_sp(&mut self) {
        self.sp += 1;  // "Decrement is addition because the stack is top-down"
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BC: 0x{:04X} | DE: 0x{:04X} | HC: 0x{:04X} | SP: 0x{:04X} | AF: 0x{:04X}", self.bc, self.de, self.hl, self.sp, self.af)
    }
}
