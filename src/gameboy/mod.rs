pub mod rom;
mod cpu;
mod mmu;
mod ppu;
pub mod operations;
mod registers;

use std::thread::sleep;
use std::time::Duration;

use self::rom::Rom;
use self::cpu::Cpu;
use self::registers::Registers;
use self::mmu::Mmu;

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
