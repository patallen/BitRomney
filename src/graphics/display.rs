const DISPLAY_WIDTH_PIXELS: u32 = 160;
const DISPLAY_HEIGHT_PIXELS: u32 = 144;
const SCALE: u32 = 5;
const TITLE: &'static str = "BitRomney GB";

pub struct Dims {
    pub width: usize,
    pub height: usize,
}

impl Default for Dims {
    fn default() -> Dims {
        Self {
            width: 160,
            height: 144,
        }
    }
}

pub struct Display {
    window: minifb::Window,
    dims: Dims,
    scale: u8,
}

impl Display {
    pub fn new() -> Display {
        let dims = Dims::default();
        let window = minifb::Window::new(
            "GameBoy",
            dims.width,
            dims.height,
            minifb::WindowOptions {
                scale: minifb::Scale::X2,
                ..minifb::WindowOptions::default()
            },
        )
        .expect("Failed to create window.");

        Display {
            window,
            dims,
            scale: 2,
        }
    }

    pub fn draw_frame(&mut self, data: &[u32]) {
        self.window.update_with_buffer(&data);
    }
}
