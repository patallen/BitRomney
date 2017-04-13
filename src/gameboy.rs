use rom::Rom;
use cpu::Cpu;
use mmu::Mmu;

extern crate sdl2;
use self::sdl2::render::Renderer;
use self::sdl2::pixels::Color;

const DISPLAY_WIDTH: u32 = 320;
const DISPLAY_HEIGHT: u32 = 288;


struct Display {
    renderer: Renderer<'static>,
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
        Display {
            renderer: renderer,
        }
    }
    pub fn draw_frame(&mut self, data: [u8; 23_040]){
        println!("CALLED BACK");
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
