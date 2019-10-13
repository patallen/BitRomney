pub mod display;
use std::fmt;

pub struct Control {
    pub lcd_enable: bool,        // Can only be done during V-Blank
    pub tilemap_select: bool,    // false=9800-9BFF, true=9C00-9FFF
    pub display_enable: bool,    // false=off, true=on
    pub bg_data_select: bool,    // false=8800-97FF, true=8000-8FFF
    pub bg_tilemap_select: bool, // false=9800-9BFF, true=9C00-9FFF
    pub obj_size: bool,          // false=8x8, true=8x16
    pub obj_enable: bool,        // false=off, true=on
    pub bg_display: bool,        // false=off, true=on -- When cleared, background is blank
}

impl Control {
    pub fn new() -> Control {
        Control {
            lcd_enable: false,
            tilemap_select: false,
            display_enable: false,
            bg_data_select: false,
            bg_tilemap_select: false,
            obj_size: false,
            obj_enable: false,
            bg_display: false,
        }
    }
    pub fn read_u8(&self) -> u8 {
        (self.lcd_enable as u8) << 7
            | (self.tilemap_select as u8) << 6
            | (self.display_enable as u8) << 5
            | (self.bg_data_select as u8) << 4
            | (self.bg_tilemap_select as u8) << 3
            | (self.obj_size as u8) << 2
            | (self.obj_enable as u8) << 1
            | (self.bg_display as u8)
    }
    pub fn write_u8(&mut self, byte: u8) {
        self.lcd_enable = (byte >> 7 & 0b1) == 1;
        self.tilemap_select = (byte >> 6 & 0b1) == 1;
        self.display_enable = (byte >> 5 & 0b1) == 1;
        self.bg_data_select = (byte >> 4 & 0b1) == 1;
        self.bg_tilemap_select = (byte >> 3 & 0b1) == 1;
        self.obj_size = (byte >> 2 & 0b1) == 1;
        self.obj_enable = (byte >> 1 & 0b1) == 1;
        self.bg_display = (byte & 0b1) == 1;
    }
}

pub struct Tile {
    pub lines: Vec<[u8; 8]>,
}

impl Tile {
    pub fn new(slice: &[u8]) -> Self {
        let mut vec: Vec<[u8; 8]> = Vec::new();
        for bytes in slice.chunks(2) {
            let mut line: [u8; 8] = [0; 8];
            for i in 0..8 {
                line[7 - i] = (bytes[0] >> i & 1) + (bytes[0] >> i & 1);
            }
            vec.push(line);
        }
        Tile { lines: vec }
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lines = &self.lines;
        let mut rv: Vec<String> = Vec::new();
        for line in lines.into_iter() {
            let l = line
                .into_iter()
                .map(|x| match *x {
                    0 => "_".to_string(),
                    _ => format!("{}", x),
                })
                .collect::<Vec<_>>()
                .join(" ");
            rv.push(l);
        }
        write!(f, "{}", rv.into_iter().collect::<Vec<_>>().join("\n"))
    }
}

pub enum StatMode {
    Hblank,   // LCD Controller is in H-Blank period
    Vblank,   // LCD Controller is in V-blank period
    Search,   // LCD Controller is reading from OAM/RAM
    Transfer, // LCD Controller is reading from OAM & RAM - CPU cannot access OAM
}

impl StatMode {
    fn from_u8(byte: u8) -> StatMode {
        match byte {
            0 => StatMode::Hblank,
            1 => StatMode::Vblank,
            2 => StatMode::Search,
            3 => StatMode::Transfer,
            _ => panic!("{} is an invalid StatMode value.", byte),
        }
    }
    fn to_u8(&self) -> u8 {
        match *self {
            StatMode::Hblank => 0,
            StatMode::Vblank => 1,
            StatMode::Search => 2,
            StatMode::Transfer => 3,
        }
    }
}

pub struct Stat {
    pub lyc_int_enable: bool,    // false=disable, true=enable
    pub oam_int_enable: bool,    // false=disable, true=enable
    pub vblank_int_enable: bool, // false=disable, true=enable
    pub hblank_int_enable: bool, // false=disable, true=enable
    pub coincidence_flag: bool,  // false=lyc!=ly, true=lyc==ly
    pub mode: StatMode,
}

impl Stat {
    pub fn new() -> Self {
        Stat {
            lyc_int_enable: false,    // false=disable, true=enable
            oam_int_enable: false,    // false=disable, true=enable
            vblank_int_enable: false, // false=disable, true=enable
            hblank_int_enable: false, // false=disable, true=enable
            coincidence_flag: false,  // false=lyc!=ly, true=lyc==ly
            mode: StatMode::Hblank,
        }
    }
    pub fn read_u8(&self) -> u8 {
        (self.lyc_int_enable as u8) << 6
            | (self.oam_int_enable as u8) << 5
            | (self.vblank_int_enable as u8) << 4
            | (self.hblank_int_enable as u8) << 3
            | (self.coincidence_flag as u8) << 2
            | self.mode.to_u8()
    }
    pub fn write_u8(&mut self, byte: u8) {
        self.lyc_int_enable = (byte >> 6 & 0b1) == 1; // false=disable, true=enable
        self.oam_int_enable = (byte >> 5 & 0b1) == 1; // false=disable, true=enable
        self.vblank_int_enable = (byte >> 4 & 0b1) == 1; // false=disable, true=enable
        self.hblank_int_enable = (byte >> 3 & 0b1) == 1; // false=disable, true=enable
        self.coincidence_flag = (byte >> 2 & 0b1) == 1; // false=lyc!=ly, true=lyc==ly
        self.mode = StatMode::from_u8(byte & 0b11);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Shade {
    White,
    LightGray,
    DarkGray,
    Black,
}

impl Shade {
    pub fn from_u8(byte: u8) -> Shade {
        match byte {
            3 => Shade::White,
            2 => Shade::LightGray,
            1 => Shade::DarkGray,
            0 => Shade::Black,
            _ => panic!("{} is an invalid Shade value.", byte),
        }
    }
}

impl From<u8> for Shade {
    fn from(value: u8) -> Shade {
        match value {
            3 => Shade::White,
            2 => Shade::LightGray,
            1 => Shade::DarkGray,
            0 => Shade::Black,
            _ => panic!("{} is not a valid integer value for Shade", value),
        }
    }
}

impl Into<u8> for Shade {
    fn into(self) -> u8 {
        match self {
            Shade::White => 0,
            Shade::LightGray => 1,
            Shade::DarkGray => 2,
            Shade::Black => 3,
        }
    }
}

impl Into<u32> for Shade {
    fn into(self) -> u32 {
        let int: u8 = self.into();
        let int = int as u32;
        (((int << 8) | int) << 8) | int
    }
}

pub struct Palette {
    color_3: Shade,
    color_2: Shade,
    color_1: Shade,
    color_0: Shade,
}

impl Palette {
    pub fn new() -> Self {
        Palette {
            color_3: Shade::White,
            color_2: Shade::White,
            color_1: Shade::White,
            color_0: Shade::White,
        }
    }

    pub fn write_u8(&mut self, byte: u8) {
        self.color_3 = ((byte >> 6) & 0b11).into();
        self.color_2 = ((byte >> 4) & 0b11).into();
        self.color_1 = ((byte >> 2) & 0b11).into();
        self.color_0 = (byte & 0b11).into();
    }

    pub fn read_u8(&self) -> u8 {
        let c3: u8 = self.color_3.into();
        let c2: u8 = self.color_2.into();
        let c1: u8 = self.color_1.into();
        let c0: u8 = self.color_0.into();
        c3 << 6 | c2 << 4 | c1 << 2 | c0
    }
}
