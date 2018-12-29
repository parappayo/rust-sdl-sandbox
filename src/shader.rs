use gl;
use std;
use std::ffi::{CStr};

pub struct ShaderProgram
{
	id: gl::types::GLuint
}

impl ShaderProgram
{
	pub fn id(&self) -> gl::types::GLuint
	{
		self.id
	}

	pub fn set_used(&self)
	{
		unsafe {
			gl::UseProgram(self.id);
		}
	}

	pub fn create(shaders: &[Shader]) -> Result<ShaderProgram, String>
	{
		let program_id = unsafe { gl::CreateProgram() };

		for shader in shaders {
			unsafe {
				gl::AttachShader(program_id, shader.id());
			}
		}

		unsafe {
			gl::LinkProgram(program_id);
		}

		// TODO: handle shader linker errors here

		for shader in shaders {
			unsafe {
				gl::DetachShader(program_id, shader.id());
			}
		}

		Ok(ShaderProgram { id: program_id })
	}
}

impl Drop for ShaderProgram {
	fn drop(&mut self)
	{
		unsafe {
			gl::DeleteProgram(self.id);
		}
	}
}

pub struct Shader
{
	id: gl::types::GLuint
}

impl Shader {
	pub fn id(&self) -> gl::types::GLuint
	{
		self.id
	}

	pub fn compile_vertex_shader(source: &CStr) -> Result<Shader, String>
	{
		let id = compile_shader(source, gl::VERTEX_SHADER)?;
		Ok(Shader{ id })
	}

	pub fn compile_fragment_shader(source: &CStr) -> Result<Shader, String>
	{
		let id = compile_shader(source, gl::FRAGMENT_SHADER)?;
		Ok(Shader{ id })
	}
}

impl Drop for Shader {
	fn drop(&mut self)
	{
		unsafe {
			gl::DeleteShader(self.id);
		}
	}
}

fn compile_shader(source: &CStr, shader_type: gl::types::GLuint)
	-> Result<gl::types::GLuint, String>
{
	let shader_id = unsafe { gl::CreateShader(shader_type) };

	//let mut compile_result: gl::types::GLint = 1;

	unsafe {
		gl::ShaderSource(shader_id, 1, &source.as_ptr(), std::ptr::null());
		gl::CompileShader(shader_id);
		//gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut compile_result);
	}

	// TODO: handle shader compiler errors
	Ok(shader_id)
}
