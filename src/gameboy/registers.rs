use std::fmt;
use bitty::BitFlags;

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
    pub fn set_u8(&mut self, byte: u8) {
        self.z = ((byte >> 7) & 1) == 1;
        self.n = ((byte >> 6) & 1) == 1;
        self.h = ((byte >> 5) & 1) == 1;
        self.c = ((byte >> 4) & 1) == 1;
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
            sp: 0xFFFE,
            pc: 0x000,
        }
    }
    pub fn bc(&self) -> u16 {
        let mut bc = (self.b as u16) << 8;
        bc |= self.c as u16;
        bc
    }
    pub fn de(&self) -> u16 {
        let mut de = (self.d as u16) << 8;
        de |= self.e as u16;
        de
    }
    pub fn hl(&self) -> u16 {
        let mut hl = (self.h as u16) << 8;
        hl |= self.l as u16;
        hl
    }
    pub fn af(&self) -> u16 {
        let mut af = (self.a as u16) << 8;
        af |= self.flags.as_u8() as u16;
        af
    }
    pub fn set_bc(&mut self, val: u16) {
        self.b = ((val & 0xFF00) >> 8) as u8;
        self.c = (val & 0xFF) as u8;
    }
    pub fn set_de(&mut self, val: u16) {
        self.d = ((val & 0xFF00) >> 8) as u8;
        self.e = (val & 0xFF) as u8;
    }
    pub fn set_hl(&mut self, val: u16) {
        self.h = ((val & 0xFF00) >> 8) as u8;
        info!("Setting H to: {:02X}", self.h);
        self.l = (val & 0xFF) as u8;
        info!("Setting L to: {:02X}", self.l);
    }
    pub fn set_af(&mut self, val: u16) {
        self.a = (val & 0xFF00) as u8;
        self.flags.set_u8((val & 0xFF) as u8);
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}\nA:{:02X} | B:{:02X} | C:{:02X} | D:{:02X} | E:{:02X} | H:{:02X} | L:{:02X}\n",
               self.flags, self.a, self.b, self.c, self.d, self.e, self.h, self.l)
    }
}
