#[allow(non_snake_case)]
#[allow(unused_variables)]
mod operations;

mod rom;
mod cpu;
mod mmu;
mod bitty;
mod gameboy;
mod debugger;
mod registers;


use debugger::Debugger;


fn main() {
    let filename = "/Users/patallen/Code/Emulators/GameRoy/resources/tetris1.1.gb";
    let mut debugger = Debugger::new(filename);
	  debugger.run();
}
