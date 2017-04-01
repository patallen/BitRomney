pub struct Mmu  {
	bios: 	Box<[u8]>,
	romo: 	Box<[u8]>,
	romx: 	Box<[u8]>,
	vram: 	Box<[u8]>,
	sram: 	Box<[u8]>,
	wramo: 	Box<[u8]>,
	wramx: 	Box<[u8]>,
	echo: 	Box<[u8]>,
	oam: 	Box<[u8]>,
	hram: 	Box<[u8]>,
	io: 	Box<[u8]>,
	in_bios: 	bool,
	ie: 	u8,
}

// pub struct Interconnect {
// 	rom: Rom,
// 	bios: Box<[u8]>,
// 	gpu: Gpu,
// 	sram: Box<[u8]>,

// }
impl Mmu {
	pub fn new() -> Mmu {
		Mmu {
			bios: 	 Box::new([0; 0x00FF]),
			romo: 	 Box::new([0; 0x4000]),
			romx: 	 Box::new([0; 0x4000]),
			vram: 	 Box::new([0; 0x2000]),
			sram: 	 Box::new([0; 0x2000]),
			wramo: 	 Box::new([0; 0x1000]),
			wramx: 	 Box::new([0; 0x1000]),
			echo: 	 Box::new([0; 0x1000]),
			oam: 	 Box::new([0; 0xa0]),
			hram: 	 Box::new([0; 0x80]),
			io: 	 Box::new([0; 0x80]),
			in_bios: true,
			ie: 	 0,
		}
	}
	pub fn read(&self, address: usize) -> u8 {
		match address {
			0x0000...0x00FF => match self.in_bios {
				true  	=> self.bios[address],
				false 	=> self.romo[address],
			},
			0x0000...0x3FFF	=> self.romo[address],
			0x4000...0x7FFF => self.romx[address - 0x4000],
			0x8000...0x9FFF => self.vram[address - 0x8000],
			0xA000...0xBFFF => self.sram[address - 0xA000],
			0xC000...0xCFFF => self.wramo[address - 0xC000],
			0xD000...0xDFFF => self.wramx[address - 0xD000],
			0xE000...0xFDFF => self.echo[address - 0xE000],
			0xFE00...0xFE9F => self.oam[address - 0xFE00],
			0xFF00...0xFF7F => self.io[address - 0xFF00],
			0xFF80...0xFFFE => self.hram[address - 0xFF80],
			0xFFFF 			=> self.ie,
			_				=> panic!("{:X} is and unused address."),
		}
	}
	pub fn write(&mut self, address: usize, byte: u8) {
		match address {
			0x0000...0x3FFF => self.romo[address] = byte,
			0x4000...0x7FFF => self.romx[address - 0x4000] = byte,
			0x8000...0x9FFF => self.vram[address - 0x8000] = byte,
			0xA000...0xBFFF => self.sram[address - 0xA000] = byte,
			0xC000...0xCFFF => self.wramo[address - 0xC000] = byte,
			0xD000...0xDFFF => self.wramx[address - 0xD000] = byte,
			0xE000...0xFDFF => self.echo[address - 0xE000] = byte,
			0xFE00...0xFE9F => self.oam[address - 0xFE00] = byte,
			0xFF00...0xFF7F => self.io[address - 0xFF00] = byte,
			0xFF80...0xFFFE => self.hram[address - 0xFF80] = byte,
			0xFFFF 			=> self.ie = byte,
			_				=> {}
		}
	}
}