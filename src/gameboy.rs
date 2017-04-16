use rom::Rom;
use cpu::Cpu;
use mmu::Mmu;

use ::sdl2::render::{Renderer, Texture, BlendMode};
use ::sdl2::pixels::{Color, PixelFormatEnum};


const DISPLAY_WIDTH  : u32 = 160;
const DISPLAY_HEIGHT : u32 = 144;
const SCALE          : u32 = 2;
const TITLE          : &'static str = "BitRomney GB";
const BACKGROUND     : (u8, u8, u8) = (155, 188, 15);


pub struct Gameboy {
    pub mmu: Mmu,
    pub cpu: Cpu,
}

impl Gameboy {
    pub fn new(rompath: &str) -> Gameboy {
        let context = ::sdl2::init().unwrap();
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


pub struct Display {
    renderer: Renderer<'static>,
    texture: Texture,
    prev_texture: Texture,
}

impl Display {
    pub fn new(context: ::sdl2::Sdl) -> Display {
        let width = DISPLAY_WIDTH;
        let height = DISPLAY_HEIGHT;

        let video = context.video().unwrap();
        let window = video.window(TITLE, width * SCALE, height * SCALE)
                          .position_centered().opengl()
                          .build().unwrap();
        let mut renderer = window.renderer().accelerated().build().unwrap();

        let (r, g, b) = BACKGROUND;
        renderer.set_draw_color(Color::RGB(r, g, b));
        renderer.clear();
        renderer.present();

        let texture = renderer
            .create_texture_streaming(PixelFormatEnum::ARGB8888, width, height)
            .unwrap();
        let prev_texture = renderer
            .create_texture_streaming(PixelFormatEnum::ARGB8888, width, height)
            .unwrap();
        Display {
            renderer: renderer,
            texture: texture,
            prev_texture: prev_texture,
        }
    }
    pub fn draw_frame(&mut self, data: [u8; 23_040 * 4]){
        self.renderer.clear();
        self.renderer.copy(&self.prev_texture, None, None).unwrap();
        self.texture.update(None, &data, DISPLAY_WIDTH as usize * 4).unwrap();
        self.texture.set_alpha_mod(127);
        self.texture.set_blend_mode(BlendMode::Blend);
        self.renderer.copy(&self.texture, None, None).unwrap();
        self.renderer.present();
        self.prev_texture.update(None, &data, DISPLAY_WIDTH as usize * 4).unwrap();
    }
}
