use std::{ffi::CString, ptr, str::from_utf8};

use gl::types::*;

use crate::ShaderError;

pub enum ShaderStageType {
    Vertex,
    Fragment,
}

#[allow(unused)]
pub struct ShaderStage {
    id: GLuint,
    stage: ShaderStageType,
}

impl ShaderStageType {
    pub fn gl_enum(&self) -> GLuint {
        match self {
            ShaderStageType::Vertex => gl::VERTEX_SHADER,
            ShaderStageType::Fragment => gl::FRAGMENT_SHADER,
        }
    }
}

impl ShaderStage {
    pub fn id(&self) -> GLuint {
        self.id
    }

    pub unsafe fn compile(source: &str, stage: ShaderStageType) -> Result<ShaderStage, ShaderError> {
        let id: GLuint = gl::CreateShader(stage.gl_enum());
        let c_str_vert = CString::new(source.as_bytes()).unwrap();
        gl::ShaderSource(id, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(id);

        let shader = Self { id, stage };

        match shader.linking_status() {
            Ok(_) => Ok(shader),
            Err(e) => Err(e),
        }
    }

    unsafe fn linking_status(&self) -> Result<(), ShaderError> {
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character

        gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut success);

        match success == gl::TRUE as GLint {
            true => Ok(()),
            false => {
                gl::GetShaderInfoLog(
                    self.id,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                Err(ShaderError::LinkingError(
                    from_utf8(&info_log).unwrap().to_owned(),
                ))
            }
        }
    }
}
