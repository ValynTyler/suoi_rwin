use std::ffi::CString;

use gl::types::GLint;
use suoi_types::{Matrix, Matrix4};
use crate::Shader;

pub trait ShaderUniform {
    unsafe fn set(&self, shader: &Shader, var_name: &str);
}

impl ShaderUniform for i32 {
    unsafe fn set(&self, shader: &Shader, var_name: &str) {
        let name = CString::new(var_name).unwrap();
        let address = gl::GetUniformLocation(shader.id, name.as_ptr());
        gl::Uniform1i(address, *self);
    }
}

impl ShaderUniform for bool {
    unsafe fn set(&self, shader: &Shader, var_name: &str) {
        let name = CString::new(var_name).unwrap();
        let address = gl::GetUniformLocation(shader.id, name.as_ptr());
        gl::Uniform1i(address, *self as GLint);
    }
}

impl ShaderUniform for f32 {
    unsafe fn set(&self, shader: &Shader, var_name: &str) {
        let name = CString::new(var_name).unwrap();
        let address = gl::GetUniformLocation(shader.id, name.as_ptr());
        gl::Uniform1f(address, *self);
    }
}

// impl ShaderUniform for Vector4 {
//     unsafe fn set(&self, shader: &Shader, var_name: &str) {
//         let name = CString::new(var_name).unwrap();
//         let address = gl::GetUniformLocation(shader.id, name.as_ptr());
//         gl::Uniform4f(address, self.x, self.y, self.z, self.w);
//     }
// }

impl ShaderUniform for &Matrix4 {
    unsafe fn set(&self, shader: &Shader, var_name: &str) {
        let name = CString::new(var_name).unwrap();
        let address = gl::GetUniformLocation(shader.id, name.as_ptr());
        gl::UniformMatrix4fv(address, 1, gl::FALSE, self.ptr());
    }
}
