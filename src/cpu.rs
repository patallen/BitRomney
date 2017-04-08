use std::thread;
use std::time::Duration;

use mmu::Mmu;
use registers::{Registers};
use operations::{get_operation, Operation};


pub struct Cpu {
    pub regs: Registers,
    pub pc: usize,
    pub counter: u8,    // Will count down until next instruction
}


impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0,
            regs: Registers::new(),
            counter: 0,
        }
    }
    pub fn cycle(&mut self, mmu: &mut Mmu) {
        let operation = self.get_operation(mmu);

        self.handle_operation(operation, mmu);

        let duration = Duration::new(0, 500_000_000);
        thread::sleep(duration);
    }
    fn handle_operation(&mut self, operation: Operation, mmu: &mut Mmu) {
        println!("PC: {:04X} :: {:?}", self.pc, operation);
        println!("  --> {:?}", self.regs);
        (operation.func)(self, mmu);
        self.pc += operation.size;
    }
    fn get_operation(&self, mmu: &mut Mmu) -> Operation {
        let first = mmu.read(self.pc) as u16;
        let code = match mmu.read(self.pc) {
            0xCB => { first << 8 | mmu.read(self.pc + 1) as u16 },
            _ => first
        };
        get_operation(code)
    }
    pub fn immediate_u16(&self, mmu: &Mmu) -> u16 {
        let a = mmu.read(self.pc + 1);
        let b = mmu.read(self.pc + 2);
        ((b as u16) << 8) | a as u16
    }
    pub fn immediate_u8(&self, mmu: &Mmu) -> u8 {
        mmu.read(self.pc + 1)
    }
}
