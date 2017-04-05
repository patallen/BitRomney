pub mod flag {
    pub const Z: usize = 7; // Zero Flag - Set to 1 if result of operation = 0
    pub const N: usize = 6; // Negative Flag - Used for signed arithmetic
    pub const H: usize = 5; // Half Carry Flag
    pub const C: usize = 4; // Carry Flag
}


pub trait LittleEndian {
    fn get_msb(&self) -> u8;
    fn get_lsb(&self) -> u8;
    fn set_msb(&mut self, u8) -> ();
    fn set_lsb(&mut self, u8) -> ();
}

pub trait BigEndian {
    fn get_msb(&self) -> u8;
    fn get_lsb(&self) -> u8;
    fn set_msb(&mut self, u8) -> ();
    fn set_lsb(&mut self, u8) -> ();
}

impl LittleEndian for u16 {
    fn get_msb(&self) -> u8 { (*self & 0xFF) as u8 }

    fn get_lsb(&self) -> u8 { ((*self & 0xFF00) >> 8) as u8 }

    fn set_msb(&mut self, byte: u8) {
        *self &= 0xFF00;
        *self |= byte as u16;
    }

    fn set_lsb(&mut self, byte: u8) {
        *self &= 0x00FF;
        *self |= (byte as u16) << 8;
    }
}

impl BigEndian for u16 {
    fn get_lsb(&self) -> u8 { (*self & 0xFF) as u8 }

    fn get_msb(&self) -> u8 { ((*self & 0xFF00) >> 8) as u8 }

    fn set_lsb(&mut self, byte: u8) {
        *self &= 0xFF00;
        *self |= byte as u16;
    }

    fn set_msb(&mut self, byte: u8) {
        *self &= 0x00FF;
        *self |= (byte as u16) << 8;
    }
}


pub trait BitFlags {
    fn get_bit(&self, bitno: usize) -> u8;
    fn set_bit(&mut self, bitno: usize, bit: u8) -> ();
    fn flip_bit(&mut self, bitno: usize) -> ();
}

impl BitFlags for u8 {
    fn get_bit(&self, bitno: usize) -> u8 { (*self << bitno) & 0b1 }

    fn set_bit(&mut self, bitno: usize, bit: u8) {
        let bit = match bit {
            0 | 1 => bit,
            _ => panic!("Bit value must be a 0 or 1."),
        };
        let pushed = bit << bitno;
        *self |= pushed;
    }

    fn flip_bit(&mut self, bitno: usize) {
        let pushed = 1 << bitno;
        *self ^= pushed;
    }
}
