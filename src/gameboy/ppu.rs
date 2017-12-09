use graphics::{Control, Palette, Stat, Tile, Shade};


const FRAMEBUFFER_SIZE: usize = 92160;


pub struct Ppu {
    framebuffer: [u8; FRAMEBUFFER_SIZE],
    on_refresh: Option<Box<FnMut([u8; FRAMEBUFFER_SIZE])>>,
    vram: Box<[u8]>,
    oam: Box<[u8]>,
    control: Control, // FF40
    pub stat: Stat, // FF41
    scroll_y: usize, // FF42
    scroll_x: usize, // FF43

    // Vertical line to which we are transferring data
    ly: usize, // FF44

    // Compares this to ly. When equal, set the coincident
    // bit and request a STAT interrupt
    lyc: usize, // FF45

    // Writing to dma_address launches a DMA transfer from
    // ROM/RAM to OAM.  The value that is set specifies the
    // transfer source address divided by 0x100.
    dma_address: usize, // FF46

    bg_palette: Palette, // FF47
    obj0_palette: Palette, // FF48
    obj1_palette: Palette, // FF49
    window_y: u8, // FF4A
    window_x: u8, // FF4B
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            framebuffer: [0; FRAMEBUFFER_SIZE],
            on_refresh: None,
            vram: Box::new([0; 0x2000]),
            oam: Box::new([0; 0xA0]),
            control: Control::new(),
            stat: Stat::new(),
            scroll_x: 0,
            scroll_y: 0,
            ly: 0,
            lyc: 0,
            dma_address: 0,
            bg_palette: Palette::new(),
            obj0_palette: Palette::new(),
            obj1_palette: Palette::new(),
            window_y: 0,
            window_x: 0,
        }
    }

    fn get_tile(&self, tile_no: usize) -> Tile {
        let idx = match self.control.bg_data_select {
            false => ((tile_no.wrapping_add(128) as u16) * 16 + 0x800) as usize,
            true => (tile_no * 16) as usize,
        };
        let slice = &self.vram[idx..idx + 16];
        Tile::new(slice)
    }
    pub fn read_u8(&self, loc: usize) -> u8 {
        let result = match loc {
            0x8000...0x9FFF => self.vram[loc - 0x8000],
            0xFE00...0xFE9F => self.oam[loc - 0xFE00],
            0xFF40 => self.control.read_u8(),
            0xFF41 => self.stat.read_u8(),
            0xFF42 => self.scroll_y as u8,
            0xFF43 => self.scroll_x as u8,
            0xFF44 => self.ly as u8,
            0xFF45 => self.lyc as u8,
            0xFF46 => self.dma_address as u8,
            0xFF47 => self.bg_palette.read_u8(),
            0xFF48 => self.obj0_palette.read_u8(),
            0xFF49 => self.obj1_palette.read_u8(),
            0xFF4A => self.window_y,
            0xFF4B => self.window_x,
            _ => panic!("{} is not a valid Ppu-mapped address.", loc),
        };
        info!("Memory Read: {:04X} @ Loc:{:04X}", result, loc);
        result
    }
    pub fn write_u8(&mut self, loc: usize, value: u8) {
        match loc {
            0x8000...0x9FFF => self.vram[loc - 0x8000] = value,
            0xFE00...0xFE9F => self.oam[loc - 0xFE00] = value,
            0xFF40 => self.control.write_u8(value),
            0xFF41 => self.stat.write_u8(value),
            0xFF42 => self.scroll_y = value as usize,
            0xFF43 => self.scroll_x = value as usize,
            0xFF44 => self.ly = value as usize,
            0xFF45 => self.lyc = value as usize,
            0xFF46 => self.dma_address = value as usize,
            0xFF47 => self.bg_palette.write_u8(value),
            0xFF48 => self.obj0_palette.write_u8(value),
            0xFF49 => self.obj1_palette.write_u8(value),
            0xFF4A => self.window_y = value,
            0xFF4B => self.window_x = value,
            _ => panic!("{} is not a valid Ppu-mapped address.", loc),
        };
    }

    pub fn tile_line(&self, x: usize) -> &[u8] {
        let base = match self.control.bg_tilemap_select {
            true => (0x1C00 + x),
            false => (0x1800 + x),
        };
        &self.vram[base..base + 20]
    }
    fn update_framebuffer(&mut self) {
        let nth_tile = ((self.scroll_y as usize + self.ly) / 8) * 32 + (self.scroll_x / 8);
        let tiles: Vec<Tile> = self.tile_line(nth_tile)
            .into_iter()
            .map(|x| self.get_tile(*x as usize))
            .collect();

        let row = (self.scroll_y + self.ly) % 8;
        let lines: Vec<u8> = tiles.iter().fold(Vec::new(), |mut v, x| {
            v.extend(&x.lines[row]);
            v
        });
        let shades: Vec<Shade> = lines.into_iter().map(|x| Shade::from_u8(x)).collect();

        let offset = (self.ly as usize * 20 * 8 * 4) as usize;
        for (i, shade) in shades.into_iter().enumerate() {
            let b = offset + i * 4;
            self.framebuffer[b..b + 4].copy_from_slice(&shade.to_rgba());
        }
    }
    pub fn step(&mut self) {
        match self.ly {
            0...143 => {
                self.stat.vblank_int_enable = false;
                if self.ly == 0 {
                    if let Some(ref mut cb) = self.on_refresh {
                        cb(self.framebuffer)
                    }
                }
                self.update_framebuffer();
                self.ly += 1;
            }
            144...153 => {
                self.stat.vblank_int_enable = true;
                self.ly += 1;
            }
            154 => self.ly = 0,
            _ => panic!("LY out of range."),
        }
    }
    pub fn set_on_refresh(&mut self, callback: Box<FnMut([u8; 23_040 * 4])>) {
        self.on_refresh = Some(callback);
    }
}
