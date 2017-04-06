#[allow(non_snake_case)]
#[allow(unused_variables)]
mod operations;

mod rom;
mod cpu;
mod mmu;
mod bitty;
mod gameboy;
mod registers;



use gameboy::Gameboy;


fn main() {
	let filename = "/Users/patallen/Code/Emulators/GameRoy/resources/pokemon_red.gb";
	let mut gb = Gameboy::new(filename);
	gb.run();
}
