use rom::Rom;
use ppu::Ppu;


const BOOT_ROM: [u8; 0x100] = [
    0x31, 0xfe, 0xff, 0xaf, 0x21, 0xff, 0x9f, 0x32,
    0xcb, 0x7c, 0x20, 0xfb, 0x21, 0x26, 0xff, 0x0e,
    0x11, 0x3e, 0x80, 0x32, 0xe2, 0x0c, 0x3e, 0xf3,
    0xe2, 0x32, 0x3e, 0x77, 0x77, 0x3e, 0xfc, 0xe0,
    0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1a,
    0xcd, 0x95, 0x00, 0xcd, 0x96, 0x00, 0x13, 0x7b,
    0xfe, 0x34, 0x20, 0xf3, 0x11, 0xd8, 0x00, 0x06,
    0x08, 0x1a, 0x13, 0x22, 0x23, 0x05, 0x20, 0xf9,
    0x3e, 0x19, 0xea, 0x10, 0x99, 0x21, 0x2f, 0x99,
    0x0e, 0x0c, 0x3d, 0x28, 0x08, 0x32, 0x0d, 0x20,
    0xf9, 0x2e, 0x0f, 0x18, 0xf3, 0x67, 0x3e, 0x64,
    0x57, 0xe0, 0x42, 0x3e, 0x91, 0xe0, 0x40, 0x04,
    0x1e, 0x02, 0x0e, 0x0c, 0xf0, 0x44, 0xfe, 0x90,
    0x20, 0xfa, 0x0d, 0x20, 0xf7, 0x1d, 0x20, 0xf2,
    0x0e, 0x13, 0x24, 0x7c, 0x1e, 0x83, 0xfe, 0x62,
    0x28, 0x06, 0x1e, 0xc1, 0xfe, 0x64, 0x20, 0x06,
    0x7b, 0xe2, 0x0c, 0x3e, 0x87, 0xe2, 0xf0, 0x42,
    0x90, 0xe0, 0x42, 0x15, 0x20, 0xd2, 0x05, 0x20,
    0x4f, 0x16, 0x20, 0x18, 0xcb, 0x4f, 0x06, 0x04,
    0xc5, 0xcb, 0x11, 0x17, 0xc1, 0xcb, 0x11, 0x17,
    0x05, 0x20, 0xf5, 0x22, 0x23, 0x22, 0x23, 0xc9,
    0xce, 0xed, 0x66, 0x66, 0xcc, 0x0d, 0x00, 0x0b,
    0x03, 0x73, 0x00, 0x83, 0x00, 0x0c, 0x00, 0x0d,
    0x00, 0x08, 0x11, 0x1f, 0x88, 0x89, 0x00, 0x0e,
    0xdc, 0xcc, 0x6e, 0xe6, 0xdd, 0xdd, 0xd9, 0x99,
    0xbb, 0xbb, 0x67, 0x63, 0x6e, 0x0e, 0xec, 0xcc,
    0xdd, 0xdc, 0x99, 0x9f, 0xbb, 0xb9, 0x33, 0x3e,
    0x3c, 0x42, 0xb9, 0xa5, 0xb9, 0xa5, 0x42, 0x3c,
    0x21, 0x04, 0x01, 0x11, 0xa8, 0x00, 0x1a, 0x13,
    0xbe, 0x20, 0xfe, 0x23, 0x7d, 0xfe, 0x34, 0x20,
    0xf5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20,
    0xfb, 0x86, 0x20, 0xfe, 0x3e, 0x01, 0xe0, 0x50,
];

pub struct Mmu  {
    rom:	  Rom,
    ppu:    Ppu,
    bios: 	Box<[u8]>,
    sram: 	Box<[u8]>,
    wramo: 	Box<[u8]>,
    wramx: 	Box<[u8]>,
    echo: 	Box<[u8]>,
    hram: 	Box<[u8]>,
    io: 	  Box<[u8]>,
    in_bios:bool,
	  ie: 	  u8,
    pub ime: bool,
}

impl Mmu {
    pub fn new(rom: Rom) -> Mmu {
        Mmu {
            rom: 	   rom,
            ppu:     Ppu::new(),
            bios: 	 Box::new(BOOT_ROM),
            sram: 	 Box::new([0; 0x2000]),
            wramo: 	 Box::new([0; 0x1000]),
            wramx: 	 Box::new([0; 0x1000]),
            echo: 	 Box::new([0; 0x1000]),
            hram: 	 Box::new([0; 0x80]),
            io: 	   Box::new([0; 0x80]),
            in_bios: true,
            ie: 	   0,
            ime:     false,
        }
    }
    pub fn read(&self, address: usize) -> u8 {
        match address {
            0x0000...0x00FF => match self.in_bios {
                true  	=> self.bios[address],
                false 	=> self.rom.read(address),
            },
            0x0000...0x7FFF	=> self.rom.read(address),
            0x8000...0x9FFF => self.ppu.read_u8(address),
            0xFE00...0xFE9F => self.ppu.read_u8(address),
            0xFF40...0xFF4B => self.ppu.read_u8(address),
            0xA000...0xBFFF => self.sram[address - 0xA000],
            0xC000...0xCFFF => self.wramo[address - 0xC000],
            0xD000...0xDFFF => self.wramx[address - 0xD000],
            0xE000...0xFDFF => self.echo[address - 0xE000],
            0xFF00...0xFF7F => self.io[address - 0xFF00],
            0xFF80...0xFFFE => self.hram[address - 0xFF80],
            0xFFFF 			    => self.ie,
            _				        => panic!("{:04X} is an unused address.", address),
        }
    }
    pub fn write(&mut self, address: usize, byte: u8) {
        match address {
            0x0000...0x7FFF => self.rom.write(address, byte),
            0x8000...0x9FFF => self.ppu.write_u8(address, byte),
            0xFE00...0xFE9F => self.ppu.write_u8(address, byte),
            0xFF40...0xFF4B => self.ppu.write_u8(address, byte),
            0xA000...0xBFFF => self.sram[address - 0xA000] = byte,
            0xC000...0xCFFF => self.wramo[address - 0xC000] = byte,
            0xD000...0xDFFF => self.wramx[address - 0xD000] = byte,
            0xE000...0xFDFF => self.echo[address - 0xE000] = byte,
            0xFF00...0xFF7F => self.io[address - 0xFF00] = byte,
            0xFF80...0xFFFE => self.hram[address - 0xFF80] = byte,
            0xFFFF 			    => self.ie = byte,
            _				        => {}
        }
    }
    pub fn read_range(&self, low: usize, high: usize) -> Vec<u8> {
        (low..high).into_iter().map(|x| self.read(x)).collect()
    }
    pub fn read_u16(&self, address: usize) -> u16 {
        let first = self.read(address) as u16;
        let second = self.read(address + 1) as u16;
        (second << 8) | first
    }
    pub fn write_u16(&mut self, address: usize, value: u16) {
        let first = (value & 0xFF) as u8;
        let second = (value >> 8) as u8;
        self.write(address, first);
        self.write(address + 1, second);
    }
    pub fn step(&mut self) {
        self.ppu.step();
    }
}
