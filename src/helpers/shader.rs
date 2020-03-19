extern crate gl;

use crate::resources::{self, Resources};

use std::ffi::{CString, CStr};

// http://nercury.github.io/rust/opengl/tutorial/2018/02/10/opengl-in-rust-from-scratch-03-compiling-shaders.html

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    // fill buffer with len spaces
    buffer.extend([b' '].iter().cycle().take(len as usize));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to load resource {}", name)]
    ResourceLoad { name: String, #[cause] inner: resources::Error },
    #[fail(display = "Can not  determine shader type from resource {}", name)]
    CanNotDetermineShaderTypeForResource { name: String },
    #[fail(display = "Failed to compile shader {}: {}", name, message)]
    CompileError { name: String, message: String },
    #[fail(display = "Failed to link program {}: {}", name, message)]
    LinkError { name: String, message: String },
    #[fail(
        display="Failed to find uniform {} in [{}]:{}",
        uniform_name,
        program_id,
        program_name,
    )]
    UniformLocationNotFound {
        uniform_name: String,
        program_id: gl::types::GLuint,
        program_name: String,
    },
    #[fail(
        display="Failed to find attribute {} in [{}]: {}",
        attrib_name,
        program_id,
        program_name,
    )]
    AttribLocationNotFound {
        attrib_name: String,
        program_id: gl::types::GLuint,
        program_name: String,
    },
}

#[derive(Debug)]
pub struct Program {
    id: gl::types::GLuint,
    name: String,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader], name: &str) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };
        let mut success: gl::types::GLint = 1;

        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id()); }
        }

        unsafe {
            gl::LinkProgram(program_id);
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;

            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id());
            }
        }

        Ok(Program { id: program_id, name: name.into() })
    }

    pub fn from_resource(res: &Resources, name: &str) -> Result<Program, Error> {
        const POSSIBLE_EXT: [&str; 2] = [
            ".vertex",
            ".fragment",
        ];

        let shaders = POSSIBLE_EXT.iter()
            .map(|file_extension| {
                Shader::from_res(res, &format!("{}{}", name, file_extension))
            })
            .collect::<Result<Vec<Shader>, Error>>()?;

        Program::from_shaders(&shaders[..], name).map_err(|message| Error::LinkError {
            name: name.into(),
            message
        })
    }

    pub fn get_uniform_location(&self, name: &str) -> Result<i32, Error> {
        let cname = CString::new(name).expect("expected uniform name to have no nul bytes");

        let location = unsafe {
            gl::GetUniformLocation(self.id, cname.as_bytes_with_nul().as_ptr() as *const i8)
        };

        if location == -1 {
            return Err(Error::UniformLocationNotFound {
                program_id: self.id.clone(),
                program_name: self.name.clone(),
                uniform_name: name.into(),
            });
        }

        Ok(location)
    }

    pub fn get_attrib_location(&self, name: &str) -> Result<i32, Error> {
        let cname = CString::new(name).expect("expected attribute name to have nul bytes");
        let location = unsafe {
            gl::GetAttribLocation(self.id, cname.as_bytes_with_nul().as_ptr() as *const i8)
        };

        if location == -1 {
            return Err(Error::AttribLocationNotFound {
                program_id: self.id.clone(),
                program_name: self.name.clone(),
                attrib_name: name.into(),
            });
        }

        Ok(location)
    }

    pub fn set_uniform_1i(&self, location: i32, value: i32) {
        unsafe {
            gl::Uniform1i(location, value);
        }
    }

    pub fn set_uniform_1f(&self, location: i32, value: f32) {
        unsafe {
            gl::Uniform1f(location, value);
        }
    }

    pub fn set_uniform_1iv(&self, location: i32, value: &Vec<i32>) {
        unsafe {
            gl::Uniform1iv(
                location,
                value.len() as i32,
                value.as_ptr() as *const i32
            );
        }
    }

    pub fn set_uniform_1fv(&self, location: i32, value: &Vec<f32>) {
        unsafe {
            gl::Uniform1fv(
                location,
                1,//value.len() as i32,
                value.as_ptr() as *const f32
            );
        }
    }

    pub fn set_uniform_1uiv(&self, location: i32, value: &Vec<u32>) {
        unsafe {
            gl::Uniform1uiv(
                location,
                1,//value.len() as i32,
                value.as_ptr() as *const u32
            );
        }
    }

    pub fn set_uniform_2f(&self, location: i32, value: &glm::Vec2) {
        unsafe {
            gl::Uniform2f(location, value.x, value.y);
        }
    }

    pub fn set_uniform_4f(&self, location: i32, value: (f32, f32, f32, f32)) {
        let (x, y, z, w) = value;
        unsafe {
            gl::Uniform4f(location, x, y, z, w);
        }
    }

    pub fn set_uniform_mat3f(&self, location: i32, value: &glm::TMat3<f32>) {
        unsafe {
            gl::UniformMatrix3fv(
                location,
                1,
                gl::FALSE,//gl::TRUE,
                value.as_ptr() as *const f32
            );
        }
    }

    pub fn set_uniform_mat4f(&self, location: i32, value: &glm::TMat4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(
                location,
                1, // # of matrices
                gl::FALSE, // transpose? column major (set true if matrix is row major)
                value.as_ptr() as *const f32
            );
        }
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

pub struct Shader {
    id: gl::types::GLuint,
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl Shader {
    fn from_source(
        source: &CStr,
        kind: gl::types::GLenum
    ) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    pub fn from_vertex_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_fragment_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn from_res(res: &Resources, name: &str) -> Result<Shader, Error> {
        const POSSIBLE_EXT: [(&str, gl::types::GLenum); 2] = [
            (".vertex", gl::VERTEX_SHADER),
            (".fragment", gl::FRAGMENT_SHADER),
        ];
        let shader_kind = POSSIBLE_EXT.iter()
            .find(|&&(file_extension, _)| {
                name.ends_with(file_extension)
            })
            .map(|&(_, kind)| kind)
            .ok_or_else(|| Error::CanNotDetermineShaderTypeForResource { name: name.into() })?;
        let source = res.load_cstring(name)
            .map_err(|e| Error::ResourceLoad {
                name: name.into(),
                inner: e,
            })?;

        Shader::from_source(&source, shader_kind).map_err(|message| Error::CompileError {
            name: name.into(),
            message,
        })
    }

    pub fn from_raw(source: &str, shader_kind: gl::types::GLenum) -> Result<Shader, Error> {
        let c_source = CString::new(source).unwrap();

        Shader::from_source(&c_source, shader_kind).map_err(|message| Error::CompileError {
            name: format!("Raw Shader Source: {}", shader_kind),
            message,
        })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub fn shader_from_source(
    source: &CStr,
    kind: gl::types::GLuint
) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };
    let mut success: gl::types::GLint = 1;

    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);

        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;

        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

