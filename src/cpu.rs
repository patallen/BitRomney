use std::thread;
use std::time::Duration;

use rom::Rom;
use mmu::Mmu;
use operations::{get_operation, Operation};
// use gameboy::Interconnect;

pub struct Cpu {
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    pub sp: u16,
    pub pc: usize,
    pub af: u16,        // 2 8-bit registers (Accumulator & flags)
    pub counter: u8,    // Will count down until next instruction
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
            counter: 0,
        }
    }
    pub fn cycle(&mut self, mmu: &mut Mmu) {
        let code = mmu.read(self.pc);
        let operation = get_operation(code, false);

        self.handle_operation(operation, mmu);

        let duration = Duration::new(1, 0);
        thread::sleep(duration);
    }
    fn handle_operation(&mut self, operation: Operation, mmu: &mut Mmu) {
        println!("{:?}", operation);
        (operation.func)(self, mmu);
        self.pc += operation.size;
    }
    fn cb(&mut self) {
        self.pc += 1;
    }
}
