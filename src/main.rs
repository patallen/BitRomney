mod rom;
mod cpu;
mod mmu;
mod gameboy;
mod operations;


use rom::Rom;
use gameboy::Gameboy;


fn main() {
	let filename = "/Users/patallen/Code/Emulators/GameRoy/resources/pokemon_red.gb";
	let mut gb = Gameboy::new(filename);
	gb.run();
}
