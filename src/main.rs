#[macro_use]
extern crate glium;
extern crate rand;

use glium::DisplayBuild;
use glium::Surface;
use std::env;
use std::path::Path;

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

	///////////////////// COMMAND LINE ARGS //////////////////////////////////
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		println!("Usage: chip8 [PATH_TO_ROM]");
		return;
	}
	let path_to_game = &args[1];

	/////////////////////// CREATE DISPLAY /////////////////////////////////
	let display = glium::glutin::WindowBuilder::new()
		.with_title(format!("Rusty Chip-8"))
		.build_glium()
		.unwrap();

	////////////////////// SETUP VERTEXES ///////////////////////////////

	#[derive(Copy, Clone)]
	struct Vertex {
	    position: [f32; 2],
	    tex_coords: [f32; 2], // this is new
	}
	implement_vertex!(Vertex, position, tex_coords);
	let vertex1 = Vertex { position: [-0.5, 0.5], tex_coords: [0.0, 1.0]};
	let vertex2 = Vertex { position: [ 0.5,  0.5], tex_coords: [1.0, 1.0]};
	let vertex3 = Vertex { position: [ 0.5, -0.5], tex_coords: [1.0, 0.0]};
	let vertex4 = Vertex { position: [ -0.5, -0.5], tex_coords: [0.0, 0.0]};
	let shape = vec![vertex1, vertex2, vertex3, vertex4];

	// uploading this shape to the memory of our video card in what is called a vertex buffer
	let vertex_buffer = glium::VertexBuffer::new(&display, shape);

	// to tell OpenGL how to link these vertices together to obtain triangles.
	// let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);
	let indices = glium::IndexBuffer::new(&display, 
		glium::index::TriangleStrip(vec![1 as u16, 2, 0, 3]));

	/////////////////// SHADERS /////////////////////////////////////////
	let vertex_shader_src = r#"
	    #version 140

	    in vec2 position;
	    in vec2 tex_coords;

	    out vec2 v_tex_coords;

	    void main() {
	    	v_tex_coords = tex_coords; // just pass the texture coordinates through

	    	gl_Position = vec4(position, 0.0, 1.0);
	    }
		"#;

	let fragment_shader_src = r#"
	    #version 140

	    in vec2 v_tex_coords;

	    out vec4 color;

	    uniform sampler2D tex;

	    void main() {
	    	// texture() is an openGL method.
	        color = texture(tex, v_tex_coords);
	    }
		"#;

	// send shaders source code to the glium library
	let program = glium::Program::from_source(&display, vertex_shader_src, 
		fragment_shader_src, None)
		.unwrap();

	//////////////// INPUT /////////////////////////////
	setup_input();
	////////////////////////////////////////////////////


	//////////// INITIALISE CHIP 8 SYSTEM ////////////// 
	// initialise the chip8 system 
	let mut my_chip8 = chip8::Chip8::new();
	// load the game into the memory
	my_chip8.load_game(path_to_game);
	///////////////////////////////////////////////////

	// emulation loop
	loop {
		// emulate one cycle
		my_chip8.emulate_cycle();

		// if the draw flag is set, update the screen
		if my_chip8.draw_flag {
			// draw_graphics();
			/////////////  UPDATE TEXTURE ///////////////
			let mut image = vec![vec![(0.0, 0.0, 0.0); 64]; 32];

			for row in 0..32 {
				for col in 0..64 {
					if my_chip8.gfx[row * 64 + col]  == 1 {
						image[row][col] = (1.0, 1.0, 1.0);
					} else {
						image[row][col] = (0.0, 0.0, 0.0);
					}
				}		
			}

			let texture = glium::texture::Texture2d::new(&display, image);
			// texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest);
			///////////////////////////////////////////////////////

			/////////////////////////// DRAW /////////////////////
			let mut target = display.draw();
			target.clear_color(0.0, 0.0, 1.0, 1.0);

			let uniforms = uniform! {
				tex: texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
			};
			target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default())
				.unwrap();
			target.finish();

			///////////////////////////////////////

			if display.is_closed() {
				break;
			}
		}

		// store key press stage (Press and Release)
		my_chip8.set_keys();
	}
}