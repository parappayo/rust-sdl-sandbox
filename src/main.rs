extern crate gl;
extern crate sdl2;

pub mod shader;

fn main()
{
	const WINDOW_WIDTH: i32 = 800;
	const WINDOW_HEIGHT: i32 = 600;
	const CLEAR_COLOR_R: f32 = 0.3;
	const CLEAR_COLOR_G: f32 = 0.3;
	const CLEAR_COLOR_B: f32 = 0.3;
	const CLEAR_COLOR_A: f32 = 1.0;
	const GL_CONTEXT_VERSION_MAJOR: u8 = 3;
	const GL_CONTEXT_VERSION_MINOR: u8 = 3;

	let sdl = sdl2::init().unwrap();
	let video_subsystem = sdl.video().unwrap();

	let gl_attr = video_subsystem.gl_attr();
	gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
	gl_attr.set_context_version(GL_CONTEXT_VERSION_MAJOR, GL_CONTEXT_VERSION_MINOR);

	let window = video_subsystem
		.window("Sandbox", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
		.opengl()
		.resizable()
		.build()
		.unwrap();


	let _gl_context = window.gl_create_context().unwrap();
	let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

	unsafe {
		gl::Viewport(0, 0, WINDOW_HEIGHT, WINDOW_HEIGHT);
		gl::ClearColor(CLEAR_COLOR_R, CLEAR_COLOR_G, CLEAR_COLOR_B, CLEAR_COLOR_A);
	}

	use std::ffi::CString;
	let vert_shader =
		shader::Shader::compile_vertex_shader(
			&CString::new(include_str!("triangle.vert")).unwrap()
		).unwrap();
	let frag_shader =
		shader::Shader::compile_fragment_shader(
			&CString::new(include_str!("triangle.frag")).unwrap()
		).unwrap();
	let shader_program = shader::ShaderProgram::create(&[vert_shader, frag_shader]).unwrap();
	shader_program.set_used();

	let mut event_pump = sdl.event_pump().unwrap();

	'main: loop {
		for event in event_pump.poll_iter() {
			match event {
				sdl2::event::Event::Quit {..} => break 'main,
				_ => {},
			}
		}

		unsafe {
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}

		window.gl_swap_window();
	}
}
