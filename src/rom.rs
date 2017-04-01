use std::io::Read;
use std::fs::File;
use std::fmt;
use std::str;

const TITLE_START_ADDR: usize = 0x0134;

pub struct Rom {
	data: Vec<u8>,
	filename: String,
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
		}
	}
}

impl fmt::Debug for Rom {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Gameboy ROM:\n  RoFilename: {}\n", self.filename)
	}
}