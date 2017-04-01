use rom::Rom;
// use gameboy::Interconnect;

pub struct Cpu {
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
    af: u16,  // 2 8-bit registers (Accumulator & flags)
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0,
            af: 0,
        }
    }
    pub fn cycle(&mut self) {
        println!("Cycling!");
    }
}