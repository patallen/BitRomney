extern crate sdl2;

#[allow(non_snake_case)]
#[allow(unused_variables)]
mod bitty;
mod gameboy;
mod graphics;
mod debugger;


use debugger::Debugger;


fn main() {
    let filename = "/Users/patallen/Code/Emulators/GameRoy/resources/tetris1.1.gb";
    let mut debugger = Debugger::new(filename);
	  debugger.run();
}
