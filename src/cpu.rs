use mmu::Mmu;
use registers::{Registers};
use operations::{get_operation, Operation};


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
        (operation.func)(self, mmu);
        mmu.step();
    }
    pub fn get_operation(&mut self, mmu: &mut Mmu) -> Operation {
        let first = self.immediate_u8_pc(mmu) as u16;
        let code = match first {
            0xCB => { first << 8 | self.immediate_u8_pc(mmu) as u16 },
            _ => first
        };
        get_operation(code)
    }
    pub fn immediate_u16(&self, mmu: &Mmu) -> u16 {
        let pc = self.regs.pc;
        let mut ret: u16 = (mmu.read(pc + 2) as u16) << 8;
        ret |= mmu.read(pc + 1) as u16;
        ret
    }
    pub fn immediate_u8(&self, mmu: &Mmu) -> u8 {
        mmu.read(self.regs.pc + 1)
    }
    pub fn immediate_u8_pc(&mut self, mmu: &Mmu) -> u8 {
        let res = mmu.read(self.regs.pc);
        self.regs.pc += 1;
        res
    }
    pub fn immediate_u16_pc(&mut self, mmu: &Mmu) -> u16 {
        let pc = self.regs.pc;
        let mut res: u16 = (mmu.read(pc + 1) as u16) << 8;
        res |= mmu.read(pc) as u16;
        self.regs.pc += 2;
        res
    }
    pub fn stack_pop_u8(&mut self, mmu: &mut Mmu) -> u8 {
        self.regs.sp += 1;
        let sp = self.regs.sp;
        mmu.read(sp)
    }
    pub fn stack_push_u8(&mut self, val: u8, mmu: &mut Mmu) {
        let sp = self.regs.sp;
        mmu.write(sp, val);
        self.regs.sp -= 1;
    }
    pub fn stack_pop_u16(&mut self, mmu: &mut Mmu) -> u16 {
        self.regs.sp += 1;
        let sp = self.regs.sp;
        let ret = mmu.read_u16(sp);
        self.regs.sp += 1;
        ret
    }
    pub fn stack_push_u16(&mut self, val: u16, mmu: &mut Mmu) {
        self.regs.sp -= 1;
        let sp = self.regs.sp;
        mmu.write_u16(sp, val);
        self.regs.sp -= 1;
    }

}
