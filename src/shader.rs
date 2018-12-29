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

pub trait Shader
{
	fn id(&self) -> gl::types::GLuint;
	fn compile(source: &CStr) -> Result<gl::types::GLuint, String>;
}

pub struct VertexShader {
	id: gl::types::GLuint
}

impl Shader for VertexShader {
	fn id(&self) -> gl::types::GLuint
	{
		self.id
	}

	fn compile(source: &CStr) -> Result<gl::types::GLuint, String>
	{
		let id = compile_shader(source, gl::VERTEX_SHADER)?;
		Ok(VertexShader{ id });
	}
}

impl Drop for VertexShader {
	fn drop(&mut self)
	{
		unsafe {
			gl::DeleteShader(self.id);
		}
	}
}

pub struct FragmentShader {
	id: gl::types::GLuint
}

impl Shader for FragmentShader
{
	fn id(&self) -> gl::types::GLuint
	{
		self.id
	}

	fn compile(source: &CStr) -> Result<gl::types::GLuint, String>
	{
		let id = compile_shader(source, gl::FRAGMENT_SHADER)?;
		Ok(FragmentShader{ id });
	}
}

impl Drop for FragmentShader {
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
