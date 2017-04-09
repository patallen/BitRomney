use std::fmt;
use bitty::{LittleEndian, BitFlags};


pub struct FlagRegister {
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool,
}

impl FlagRegister {
    pub fn new() -> FlagRegister {
        FlagRegister {
            z: false,
            n: false,
            h: false,
            c: false,
        }
    }
    pub fn as_u8(&self) -> u8 {
        let mut flags: u8 = 0;
        flags.set_bit(7, self.z as u8);
        flags.set_bit(6, self.n as u8);
        flags.set_bit(5, self.h as u8);
        flags.set_bit(4, self.c as u8);
        flags
    }
}

impl fmt::Debug for FlagRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Z: {} | N: {} | H: {} | C: {}",
               self.z as u8, self.n as u8, self.h as u8, self.c as u8)
    }
}
pub struct Registers {
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub a: u8,
    pub flags: FlagRegister,
    pub sp: usize,
    pub pc: usize,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            a: 0,
            flags: FlagRegister::new(),
            sp: 0,
            pc: 0,
        }
    }
    pub fn bc(&self) -> u16 {
        let mut bc: u16 = 0;
        bc.set_msb(self.b);
        bc.set_lsb(self.c);
        bc
    }
    pub fn de(&self) -> u16 {
        let mut de: u16 = 0;
        de.set_msb(self.d);
        de.set_lsb(self.e);
        de
    }
    pub fn hl(&self) -> u16 {
        let mut hl: u16 = 0;
        hl.set_msb(self.h);
        hl.set_lsb(self.l);
        hl
    }
    pub fn af(&self) -> u16 {
        let mut af: u16 = 0;
        af.set_msb(self.a);
        af.set_lsb(self.flags.as_u8());
        af
    }
    pub fn set_bc(&mut self, val: u16) {
        self.b = val.get_msb();
        self.c = val.get_lsb();
    }
    pub fn set_de(&mut self, val: u16) {
        self.d = val.get_msb();
        self.e = val.get_lsb();
    }
    pub fn set_hl(&mut self, val: u16) {
        self.h = val.get_msb();
        self.l = val.get_lsb();
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
        write!(f, "{:?}\nA:{:02X} | B:{:02X} | C:{:02X} | D:{:02X} | E:{:02X} | H:{:02X} | L:{:02X}\n",
               self.flags, self.a, self.b, self.c, self.d, self.e, self.h, self.l)
    }
}
