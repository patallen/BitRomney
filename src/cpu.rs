use std::thread;
use std::time::Duration;
use std::io;

use mmu::Mmu;
use registers::{Registers};
use operations::{get_operation, Operation};
use bitty::LittleEndian;


pub struct Cpu {
    pub regs: Registers,
    pub counter: u8,    // Will count down until next instruction
    pub broken: bool,
}


impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            regs: Registers::new(),
            counter: 0,
            broken: false
        }
    }
    pub fn cycle(&mut self, mmu: &mut Mmu) {
        let operation = self.get_operation(mmu);
        if self.regs.pc == 0x000F {
            self.broken = true
        }
        let mut input = String::new();
        if self.broken {
            io::stdin().read_line(&mut input).unwrap();
        }
        self.handle_operation(operation, mmu);

        let duration = Duration::new(0, 10_000_000);
        thread::sleep(duration);
    }
    fn handle_operation(&mut self, operation: Operation, mmu: &mut Mmu) {
        println!("PC: {:04X} :: {:?}", self.regs.pc, operation);
        println!("  --> {:?}", self.regs);
        (operation.func)(self, mmu);
        let sp = self.regs.sp;
        if sp < 0xFFFE {
            println!("CUR: {:04X}, +1: {:04X}, +2 {:04X}",
                     mmu.read(sp), mmu.read(sp + 1), mmu.read(sp + 2));
        }
        self.regs.pc += operation.size;
    }
    fn get_operation(&self, mmu: &mut Mmu) -> Operation {
        let first = mmu.read(self.regs.pc) as u16;
        let code = match mmu.read(self.regs.pc) {
            0xCB => { first << 8 | mmu.read(self.regs.pc + 1) as u16 },
            _ => first
        };
        get_operation(code)
    }
    pub fn immediate_u16(&self, mmu: &Mmu) -> u16 {
        let a = mmu.read(self.regs.pc + 1);
        let b = mmu.read(self.regs.pc + 2);
        ((b as u16) << 8) | a as u16
    }
    pub fn immediate_u8(&self, mmu: &Mmu) -> u8 {
        mmu.read(self.regs.pc + 1)
    }
}
