use sdl2::render::Canvas;
use sdl2::pixels::PixelFormatEnum;
use sdl2::video::Window;
use sdl2::rect::Rect;

const DISPLAY_WIDTH: u32 = 160;
const DISPLAY_HEIGHT: u32 = 144;
const SCALE: u32 = 4;
const TITLE: &'static str = "BitRomney GB";
// const BACKGROUND: (u8, u8, u8) = (155, 188, 15);


pub struct Display {
    canvas: Canvas<Window>,
    width: u32,
    height: u32,
    scale: u32,
}

impl Display {
    pub fn new(context: ::sdl2::Sdl) -> Display {
        let window = context
            .video()
            .unwrap()
            .window(TITLE, DISPLAY_WIDTH * SCALE, DISPLAY_HEIGHT * SCALE)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();

        Display {
            canvas: canvas,
            width: DISPLAY_WIDTH,
            height: DISPLAY_HEIGHT,
            scale: SCALE,
        }
    }

    pub fn draw_frame(&mut self, data: [u8; 23_040 * 4]) {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::ARGB8888, self.width, self.height)
            .unwrap();

        texture
            .update(Rect::new(0, 0, self.width, self.height), &data, 1)
            .unwrap();
        self.canvas
            .copy(
                &texture,
                None,
                Some(Rect::new(
                    0,
                    0,
                    self.width * self.scale,
                    self.height * self.scale,
                )),
            )
            .unwrap();
        self.canvas.present()
    }
}
