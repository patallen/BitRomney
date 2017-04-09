use rom::Rom;
use cpu::Cpu;
use mmu::Mmu;


pub struct Gameboy {
    pub mmu: Mmu,
    pub cpu: Cpu,
}

impl Gameboy {
    pub fn new(rompath: &str) -> Gameboy {
        let rom = Rom::new(rompath);
        Gameboy {
            cpu: Cpu::new(),
            mmu: Mmu::new(rom),
        }
    }
    pub fn run(&mut self) {
        loop {
            self.step();
        }
    }
    pub fn step(&mut self) {
        self.cpu.cycle(&mut self.mmu);
    }
}
