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

use graphics::display::Display;

pub struct Gameboy {
    pub mmu: Mmu,
    pub cpu: Cpu,
}

impl Gameboy {
    pub fn new(rompath: &str) -> Gameboy {
        let context = ::sdl2::init().unwrap();
        let rom = Rom::new(rompath);
        let registers = Registers::new();
        let mut display = Display::new(context);
        let mut gb = Gameboy {
            cpu: Cpu::new(registers),
            mmu: Mmu::new(rom),
        };
        gb.mmu.ppu.set_on_refresh(Box::new(move | arr | {
            display.draw_frame(arr);
        }));
        gb
    }
    pub fn run(&mut self) {
        loop {
            self.step();
        }
    }
    pub fn step(&mut self) {
        self.cpu.cycle(&mut self.mmu);
        sleep(Duration::from_millis(0));
    }
}
