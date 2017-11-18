extern crate sdl2;

#[allow(non_snake_case)]
#[allow(unused_variables)]
mod bitty;
mod gameboy;
mod graphics;
mod debugger;


use debugger::Debugger;
use gameboy::Gameboy;
use gameboy::rom::Rom;


fn main() {
    use graphics::display::Display;
    let context = ::sdl2::init().unwrap();
    let event_pump = context.event_pump();
    let mut display = Display::new(context);
    let filename = "/Users/patallen/Code/Emulators/GameRoy/resources/tetris1.1.gb";
    let rom = Rom::new(filename);
    let mut gameboy = Gameboy::new(rom);
    gameboy.mmu.ppu.set_on_refresh(Box::new(move | arr | {
        display.draw_frame(arr);
    }));
    let mut debugger = Debugger::new(gameboy, event_pump.unwrap());
	debugger.run();
}
