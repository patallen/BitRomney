use mmu::Mmu;
use registers::{Registers};
use operations::{get_operation, Operation};
use bitty::LittleEndian;


pub struct Cpu {
    pub regs: Registers,
    pub counter: u8,    // Will count down until next instruction
}


impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            regs: Registers::new(),
            counter: 0,
        }
    }
    pub fn cycle(&mut self, mmu: &mut Mmu) {
        let operation = self.get_operation(mmu);
        self.handle_operation(operation, mmu);

    }
    fn handle_operation(&mut self, operation: Operation, mmu: &mut Mmu) {
        (operation.func)(self, mmu);
        self.regs.pc += operation.size;
    }
    pub fn get_operation(&self, mmu: &mut Mmu) -> Operation {
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
}
