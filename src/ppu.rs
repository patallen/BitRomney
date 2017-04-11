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
}

enum StatMode {
    Hblank   = 0,  // LCD Controller is in H-Blank period
    Vblank   = 1,  // LCD Controller is in V-blank period
    Search   = 2,  // LCD Controller is reading from OAM/RAM
    Transfer = 3,  // LCD Controller is reading from OAM & RAM - CPU cannot access OAM
}

struct Stat {
    lyc_int_enable:    bool, // false=disable, true=enable
    oam_int_enable:    bool, // false=disable, true=enable
    vblank_int_enable: bool, // false=disable, true=enable
    hblank_int_enable: bool, // false=disable, true=enable
    coincidence_flag:  bool, // false=lyc!=ly, true=lyc==ly
    mode: StatMode,
}

enum Shade {
    White     = 0,
    LightGray = 1,
    DarkGray  = 2,
    Black     = 3,
}
pub struct Ppu {
    vram:         Box<[u8]>,
    oam:          Box<[u8]>,
    control:      Control,   // FF40
    stat:         StatMode,  // FF41
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

    bg_palette:   Shade,     // FF47
    obj0_palette: Shade,     // FF48
    obj1_palette: Shade,     // FF49
    window_y:     u8,        // FF4A
    window_x:     u8,        // FF4B
}
impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            vram:         Box::new([0; 0x2000]),
            oam:          Box::new([0; 0xA0]),
            control:      Control::new(),
            stat:         StatMode::Hblank,
            scroll_x:     0,
            scroll_y:     0,
            ly:           0,
            lyc:          0,
            dma_address:  0,
            bg_palette:   Shade::White,
            obj0_palette: Shade::White,
            obj1_palette: Shade::White,
            window_y:     0,
            window_x:     0,
        }
    }
    pub fn write_vram(&mut self, loc: usize, byte: u8) {
        self.vram[loc] = byte;
    }
    pub fn read_vram(&self, loc: usize) -> u8 {
        self.vram[loc]
    }
    pub fn write_oam(&mut self, loc: usize, byte: u8) {
        self.oam[loc] = byte;
    }
    pub fn read_oam(&self, loc: usize) -> u8 {
        self.oam[loc]
    }
    pub fn step(&mut self) {
    }
}
