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
    fn handle_debug(&mut self, operation: &Operation, mmu: &Mmu) {
        let mut input = String::new();
        if self.broken {
            io::stdin().read_line(&mut input).unwrap();
        }
        if self.regs.pc == 0x000C {
            self.broken = true;
        }
        self.print_info(&operation, mmu);
    }
    pub fn cycle(&mut self, mmu: &mut Mmu) {
        let operation = self.get_operation(mmu);
        self.handle_debug(&operation, mmu);
        self.handle_operation(operation, mmu);

    }
    fn handle_operation(&mut self, operation: Operation, mmu: &mut Mmu) {
        (operation.func)(self, mmu);
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
        let pc = self.regs.pc;
        let mut ret: u16 = 0;
        ret.set_msb(mmu.read(pc + 1));
        ret.set_lsb(mmu.read(pc + 2));
        ret
    }
    pub fn immediate_u8(&self, mmu: &Mmu) -> u8 {
        mmu.read(self.regs.pc + 1)
    }
    fn print_info(&self, operation: &Operation, mmu: &Mmu) {
        let sp = self.regs.sp;
        println!("(PC:{:04X}::SP:{:04X}) | {:?}", self.regs.pc, self.regs.sp, operation);
        print!("REGS: {:?}", self.regs);
        print!("STACK: ({:04X}): {:04X}", sp, mmu.read(sp));
        if sp < 0xFFFE {
            print!(" | ({:04X}): {:04X}", sp + 1, mmu.read(sp + 1));
            if sp < 0xFFFD {
                print!(" | ({:04X}): {:04X}", sp + 2, mmu.read(sp + 2));
            }
        }
        print!("\n");
    }
}
