use std::io::Read;
use std::fs::File;
use std::fmt;
use std::str;

const TITLE_START_ADDR: usize = 0x0134;

pub struct Rom {
	data: Vec<u8>,
	filename: String,
	ramo: Box<[u8]>,
	ramx: Box<[u8]>,
}

impl Rom {
	pub fn new(filepath: &str) -> Rom {
		let filename = filepath.to_string();
		let mut file = File::open(filepath).unwrap();
		let mut data: Vec<u8> = Vec::new();
		file.read_to_end(&mut data);

		Rom {
			data: data,
			filename: filename,
			ramo: Box::new([0; 0x4000]),
			ramx: Box::new([0; 0x4000]),
		}
	}
	pub fn read(&self, address: usize) -> u8{
		match address {
			0x0000...0x3FFF => self.ramo[address],
			0x0400...0x7FFF => self.ramx[address - 0x4000],
			_ => panic!("Memory Address {:X} does not belong to the ROM")
		}
	}
	pub fn write(&mut self, address: usize, byte: u8) {
		match address {
			0x0000...0x3FFF => self.ramo[address] = byte,
			0x0400...0x7FFF => self.ramx[address - 0x4000] = byte,
			_ => panic!("Memory Address {:X} does not belong to the ROM")
		}
	}
}

impl fmt::Debug for Rom {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Gameboy ROM:\n  RoFilename: {}\n", self.filename)
	}
}