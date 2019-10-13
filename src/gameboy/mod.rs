mod cpu;
mod mmu;
pub mod operations;
mod ppu;
mod registers;
pub mod rom;

use std::thread::sleep;
use std::time::Duration;

use self::cpu::Cpu;
use self::mmu::Mmu;
use self::registers::Registers;
use self::rom::Rom;

pub struct Gameboy {
    pub mmu: Mmu,
    pub cpu: Cpu,
}

impl Gameboy {
    pub fn new(rom: Rom) -> Gameboy {
        let registers = Registers::new();
        let gb = Gameboy {
            cpu: Cpu::new(registers),
            mmu: Mmu::new(rom),
        };
        gb
    }

    pub fn step(&mut self) {
        self.cpu.cycle(&mut self.mmu);
        sleep(Duration::from_millis(0));
    }
}
