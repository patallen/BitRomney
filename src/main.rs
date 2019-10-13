#[macro_use]
extern crate log;

mod bitty;
mod debugger;
mod gameboy;
mod graphics;

use debugger::Debugger;
use gameboy::rom::Rom;
use gameboy::Gameboy;

use graphics::display::Display;
use std::env;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let filepath = format!("./{}", filename);
    let rom = Rom::new(&*filepath);
    let mut gameboy = Gameboy::new(rom);

    let mut display = Display::new();
    gameboy.mmu.ppu.set_on_refresh(Box::new(move |arr| {
        let data = arr
            .iter()
            .map(|d| {
                let d = *d as u32 * 64;
                (((d << 8) | d) << 8) | d
            })
            .collect::<Vec<u32>>();
        display.draw_frame(&data);
    }));

    let mut debugger = Debugger::new(gameboy);
    debugger.run();
}
