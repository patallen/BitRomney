use std::io::Read;
use std::fs::File;
use std::fmt;
use std::str;

const TITLE_START_ADDR: usize = 0x0134;

pub struct Rom {
	data: Vec<u8>,
	filename: String,
    pub size: usize,
}

impl Rom {
    pub fn new(filepath: &str) -> Rom {
        let filename = filepath.to_string();
        let mut file = File::open(filepath).unwrap();
        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data).unwrap();
        let size = data.len();

        Rom {
            data: data,
            filename: filename,
            size: size,
        }
    }
    pub fn read(&self, address: usize) -> u8{
        match address {
        0x0000...0x3FFF => self.data[address],
        0x0400...0x7FFF => self.data[address],
        _ => panic!("Memory Address {:X} does not belong to the ROM", address)
        }
    }
    pub fn write(&mut self, address: usize, byte: u8) {
        match address {
        0x0000...0x3FFF => self.data[address] = byte,
        0x0400...0x7FFF => self.data[address] = byte,
        _ => panic!("Memory Address {:X} does not belong to the ROM", address)
        }
    }
    pub fn read_raw(&self, address: usize) -> u8 {
        self.data[address]
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl fmt::Debug for Rom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Gameboy ROM:\n  RoFilename: {}\n", self.filename)
    }
}
