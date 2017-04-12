struct Control {
    lcd_enable:        bool, // Can only be done during V-Blank
    tilemap_select:    bool, // false=9800-9BFF, true=9C00-9FFF
    display_enable:    bool, // false=off, true=on
    bg_data_select:    bool, // false=8800-97FF, true=8000-8FFF
    bg_tilemap_select: bool, // false=9800-9BFF, true=9C00-9FFF
    obj_size:          bool, // false=8x8, true=8x16
    obj_enable:        bool, // false=off, true=on
    bg_display:        bool, // false=off, true=on -- When cleared, background is blank
}

impl Control {
    pub fn new() -> Control {
        Control {
            lcd_enable:        false,
            tilemap_select:    false,
            display_enable:    false,
            bg_data_select:    false,
            bg_tilemap_select: false,
            obj_size:          false,
            obj_enable:        false,
            bg_display:        false,
        }
    }
    pub fn read_u8(&self) -> u8 {
        (self.lcd_enable as u8)       << 7 |
        (self.tilemap_select as u8)   << 6 |
        (self.display_enable as u8)   << 5 |
        (self.bg_data_select as u8)   << 4 |
        (self.bg_tilemap_select as u8)<< 3 |
        (self.obj_size as u8)         << 2 |
        (self.obj_enable as u8)       << 1 |
        (self.bg_display as u8)
    }
    pub fn write_u8(&mut self, byte: u8) {
        self.lcd_enable =        (byte >> 7 & 0b1) == 1;
        self.tilemap_select =    (byte >> 6 & 0b1) == 1;
        self.display_enable =    (byte >> 5 & 0b1) == 1;
        self.bg_data_select =    (byte >> 4 & 0b1) == 1;
        self.bg_tilemap_select = (byte >> 3 & 0b1) == 1;
        self.obj_size =          (byte >> 2 & 0b1) == 1;
        self.obj_enable =        (byte >> 1 & 0b1) == 1;
        self.bg_display =        (byte      & 0b1) == 1;
    }
}

enum StatMode {
    Hblank,    // LCD Controller is in H-Blank period
    Vblank,    // LCD Controller is in V-blank period
    Search,    // LCD Controller is reading from OAM/RAM
    Transfer,  // LCD Controller is reading from OAM & RAM - CPU cannot access OAM
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
            StatMode::Hblank   => 0,
            StatMode::Vblank   => 1,
            StatMode::Search   => 2,
            StatMode::Transfer => 3,
        }
    }
}

struct Stat {
    lyc_int_enable:    bool, // false=disable, true=enable
    oam_int_enable:    bool, // false=disable, true=enable
    vblank_int_enable: bool, // false=disable, true=enable
    hblank_int_enable: bool, // false=disable, true=enable
    coincidence_flag:  bool, // false=lyc!=ly, true=lyc==ly
    mode: StatMode,
}

impl Stat {
    pub fn new() -> Self {
        Stat {
            lyc_int_enable:    false, // false=disable, true=enable
            oam_int_enable:    false, // false=disable, true=enable
            vblank_int_enable: false, // false=disable, true=enable
            hblank_int_enable: false, // false=disable, true=enable
            coincidence_flag:  false, // false=lyc!=ly, true=lyc==ly
            mode: StatMode::Hblank,
        }
    }
    pub fn read_u8(&self) -> u8 {
        (self.lyc_int_enable as u8)    << 6 | (self.oam_int_enable as u8)    << 5 |
        (self.vblank_int_enable as u8) << 4 | (self.hblank_int_enable as u8) << 3 |
        (self.coincidence_flag as u8)  << 2 | self.mode.to_u8()
    }
    pub fn write_u8(&mut self, byte: u8) {
        self.lyc_int_enable    = (byte >> 6 & 0b1) == 1; // false=disable, true=enable
        self.oam_int_enable    = (byte >> 5 & 0b1) == 1; // false=disable, true=enable
        self.vblank_int_enable = (byte >> 4 & 0b1) == 1; // false=disable, true=enable
        self.hblank_int_enable = (byte >> 3 & 0b1) == 1; // false=disable, true=enable
        self.coincidence_flag  = (byte >> 2 & 0b1) == 1; // false=lyc!=ly, true=lyc==ly
        self.mode              = StatMode::from_u8(byte & 0b11);
    }
}


enum Shade {
    White,
    LightGray,
    DarkGray,
    Black,
}
impl Shade {
    fn from_u8(byte: u8) -> Shade {
        match byte {
            0 => Shade::White,
            1 => Shade::LightGray,
            2 => Shade::DarkGray,
            3 => Shade::Black,
            _ => panic!("{} is an invalid Shade value.", byte),
        }
    }
    fn to_u8(&self) -> u8 {
        match *self {
            Shade::White     => 0,
            Shade::LightGray => 1,
            Shade::DarkGray  => 2,
            Shade::Black     => 3,
        }
    }
}
struct Palette {
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
        self.color_3 = Shade::from_u8((byte >> 6) & 0b11);
        self.color_2 = Shade::from_u8((byte >> 4) & 0b11);
        self.color_1 = Shade::from_u8((byte >> 2) & 0b11);
        self.color_0 = Shade::from_u8( byte       & 0b11);
    }
    pub fn read_u8(&self) -> u8 {
        let c3 = self.color_3.to_u8();
        let c2 = self.color_2.to_u8();
        let c1 = self.color_1.to_u8();
        let c0 = self.color_0.to_u8();
        c3 << 6 | c2 << 4 | c1 << 2 | c0
    }
}
pub struct Ppu {
    vram:         Box<[u8]>,
    oam:          Box<[u8]>,
    control:      Control,   // FF40
    stat:         Stat,      // FF41
    scroll_x:     u8,        // FF42
    scroll_y:     u8,        // FF43

    // Vertical line to which we are transferring data
    ly:           u8,        // FF44

    // Compares this to ly. When equal, set the coincident
    // bit and request a STAT interrupt
    lyc:          u8,        // FF45

    // Writing to dma_address launches a DMA transfer from
    // ROM/RAM to OAM.  The value that is set specifies the
    // transfer source address divided by 0x100.
    dma_address:  usize,     // FF46

    bg_palette:   Palette,   // FF47
    obj0_palette: Palette,   // FF48
    obj1_palette: Palette,   // FF49
    window_y:     u8,        // FF4A
    window_x:     u8,        // FF4B
}
impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            vram:         Box::new([0; 0x2000]),
            oam:          Box::new([0; 0xA0]),
            control:      Control::new(),
            stat:         Stat::new(),
            scroll_x:     0,
            scroll_y:     0,
            ly:           0,
            lyc:          0,
            dma_address:  0,
            bg_palette:   Palette::new(),
            obj0_palette: Palette::new(),
            obj1_palette: Palette::new(),
            window_y:     0,
            window_x:     0,
        }
    }
    pub fn read_u8(&self, loc: usize) -> u8 {
        match loc {
            0x8000...0x9FFF => self.vram[loc - 0x8000],
            0xFE00...0xFE9F => self.oam[loc - 0xFE00],
            0xFF40 => self.control.read_u8(),
            0xFF41 => self.stat.read_u8(),
            0xFF42 => self.scroll_x,
            0xFF43 => self.scroll_y,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => self.dma_address as u8,
            0xFF47 => self.bg_palette.read_u8(),
            0xFF48 => self.obj0_palette.read_u8(),
            0xFF49 => self.obj1_palette.read_u8(),
            0xFF4A => self.window_y,
            0xFF4B => self.window_x,
            _      => panic!("{} is not a valid Ppu-mapped address.", loc),
        }
    }
    pub fn write_u8(&mut self, loc: usize, value: u8) {
        match loc {
            0x8000...0x9FFF => self.vram[loc - 0x8000] = value,
            0xFE00...0xFE9F => self.oam[loc - 0xFE00] = value,
            0xFF40 => self.control.write_u8(value),
            0xFF41 => self.stat.write_u8(value),
            0xFF42 => self.scroll_x = value,
            0xFF43 => self.scroll_y = value,
            0xFF44 => self.ly = value,
            0xFF45 => self.lyc = value,
            0xFF46 => self.dma_address = value as usize,
            0xFF47 => self.bg_palette.write_u8(value),
            0xFF48 => self.obj0_palette.write_u8(value),
            0xFF49 => self.obj1_palette.write_u8(value),
            0xFF4A => self.window_y = value,
            0xFF4B => self.window_x = value,
            _      => panic!("{} is not a valid Ppu-mapped address.", loc),
        };
    }
    pub fn step(&mut self) {
        self.ly += 1;
        if self.ly > 153 {
            self.ly = 0;
        }
    }
}
