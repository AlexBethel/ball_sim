use std::{ffi::CStr, os::raw::c_char};

pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        unsafe {
            let program_id = gl::CreateProgram();

            for shader in shaders {
                gl::AttachShader(program_id, shader.id());
            }

            gl::LinkProgram(program_id);

            let success =
                from_mutator(|success| gl::GetProgramiv(program_id, gl::LINK_STATUS, success));
            if success == 0 {
                let len =
                    from_mutator(|len| gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, len));
                let error = from_foreign_writer(len as _, |ptr| {
                    gl::GetProgramInfoLog(program_id, len, std::ptr::null_mut(), ptr)
                });
                return Err(error);
            }

            for shader in shaders {
                gl::DetachShader(program_id, shader.id());
            }

            Ok(Program { id: program_id })
        }
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) }
    }
}

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, kind: gl::types::GLuint) -> Result<Self, String> {
        unsafe {
            let id = gl::CreateShader(kind);
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);

            let mut success: gl::types::GLint = 1;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let len = from_mutator(|len| gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, len));
                let error = from_foreign_writer(len as _, |ptr| {
                    gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), ptr)
                });
                return Err(error);
            }

            Ok(Self { id })
        }
    }

    pub fn from_vert_source(source: &CStr) -> Result<Self, String> {
        Self::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Self, String> {
        Self::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id) }
    }
}

fn from_mutator<T: Default>(func: impl FnOnce(&mut T)) -> T {
    let mut rv = T::default();
    func(&mut rv);
    rv
}

unsafe fn from_foreign_writer(len: usize, func: impl FnOnce(*mut c_char)) -> String {
    let mut buf = Vec::<c_char>::with_capacity(len + 1);
    buf.extend([0].iter().cycle().take(len + 1));
    func(buf.as_mut_ptr());

    CStr::from_ptr(buf.as_ptr()).to_str().unwrap().to_owned()
}
