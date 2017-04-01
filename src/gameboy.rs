use rom::Rom;
use cpu::Cpu;
use mmu::Mmu;


// pub struct Interconnect {
// 	rom: Option<Rom>,
// 	ram: Box<[u8]>,
// 	mmu: Mmu,
// }

// impl Interconnect {
// 	pub fn new(rom: Rom) -> Interconnect {
// 		Interconnect {
// 			rom: rom,
// 			mmu: Mmu::new(),
// 		}
// 	}
// }
pub struct Gameboy {
	mmu: Mmu,
	cpu: Cpu,
}

impl Gameboy {
	pub fn new() -> Gameboy {
		Gameboy {
			cpu: Cpu::new(),
			mmu: Mmu::new(),
		}
	}
	pub fn run(&mut self) {
		loop {
			self.step();
		}
	}
	pub fn step(&mut self) {
		self.cpu.cycle();
		println!("Stepping!");
	}
}