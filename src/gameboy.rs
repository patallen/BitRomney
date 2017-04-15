use rom::Rom;
use cpu::Cpu;
use mmu::Mmu;

extern crate sdl2;
use self::sdl2::render::{Renderer, Texture};
use self::sdl2::pixels::{Color, PixelFormatEnum};

const DISPLAY_WIDTH: u32 = 320;
const DISPLAY_HEIGHT: u32 = 288;


struct Display {
    renderer: Renderer<'static>,
    texture: Texture
}

impl Display {
    pub fn new(context: sdl2::Sdl) -> Display {
        let video = context.video().unwrap();
        let window = video.window("BitRomney", DISPLAY_WIDTH, DISPLAY_HEIGHT)
            .position_centered().opengl()
            .build().unwrap();
        let mut renderer = window.renderer().accelerated().build().unwrap();
        renderer.set_draw_color(Color::RGB(123, 123, 123));
        renderer.clear();
        renderer.present();
        let texture = renderer.create_texture_streaming(
            PixelFormatEnum::ARGB8888, DISPLAY_WIDTH, DISPLAY_HEIGHT).unwrap();
        Display {
            renderer: renderer,
            texture: texture,
        }
    }
    pub fn draw_frame(&mut self, data: [u8; 23_040 * 4]){
        self.renderer.clear();
        self.renderer.copy(&self.texture, None, None).unwrap();
        self.texture.update(None, &data, 100).unwrap();
        self.renderer.present();
    }
}
pub struct Gameboy {
    pub mmu: Mmu,
    pub cpu: Cpu,
}

impl Gameboy {
    pub fn new(rompath: &str) -> Gameboy {
        let context = sdl2::init().unwrap();
        let rom = Rom::new(rompath);
        let mut display = Display::new(context);
        let mut gb = Gameboy {
            cpu: Cpu::new(),
            mmu: Mmu::new(rom),
        };
        gb.mmu.ppu.set_on_refresh(Box::new(move | arr | {
            display.draw_frame(arr);
        }));
        gb
    }
    pub fn run(&mut self) {
        loop {
            self.step();
        }
    }
    pub fn step(&mut self) {
        self.cpu.cycle(&mut self.mmu);
    }
}
