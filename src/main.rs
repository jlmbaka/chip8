// #[macro_use]
// extern crate glium;
extern crate rand;

// use glium::DisplayBuild;

mod chip8;

fn setup_graphics(width: usize, height: usize) {
	// clean screen
	// create a texture
	// setup a texture
	// enable textures
}

fn draw_graphics() {
	// update pixels
	// update textures
}

fn setup_input() {

}

fn main() {
	// setup render system and register input callback
	
	// setup_input();

	// initialise the chip8 system 
	let mut my_chip8 = chip8::Chip8::new();

	// load the game into the memory
	my_chip8.load_game("games/TICTAC");

	// emulation loop
	loop {
		// emulate one cycle
		my_chip8.emulate_cycle();

		// if the draw flag is set, update the screen
		if my_chip8.draw_flag {
			draw_graphics();
		}

		// store key press stage (Press and Release)
		my_chip8.set_keys();
	}
}