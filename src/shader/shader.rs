use std::str::from_utf8;

use gl::types::{GLchar, GLint, GLuint};

use crate::{ShaderError, ShaderStage};

#[allow(unused)]
pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub unsafe fn compile(vertex: ShaderStage, fragment: ShaderStage) -> Result<Shader, ShaderError> {
        // Shader Program
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex.id());
        gl::AttachShader(shader_program, fragment.id());
        gl::LinkProgram(shader_program);

        // clean up shaders
        gl::DeleteShader(vertex.id());
        gl::DeleteShader(fragment.id());

        let shader = Shader { id: shader_program };

        // check for linking errors
        match shader.linking_status() {
            Ok(_) => Ok(shader),
            Err(e) => Err(e),
        }
    }

    pub unsafe fn linking_status(&self) -> Result<(), ShaderError> {
        let mut success = gl::FALSE as GLint;
        let mut info_log: Vec<u8> = Vec::with_capacity(512);

        gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success);

        match success == gl::TRUE as GLint {
            true => Ok(()),
            false => {
                gl::GetProgramInfoLog(
                    self.id,
                    512,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );

                Err(ShaderError::LinkingError(
                    from_utf8(&info_log).unwrap().to_owned(),
                ))
            }
        }
    }
}
