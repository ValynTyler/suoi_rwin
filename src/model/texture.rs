use std::{
    ffi::c_void,
    path::{Path, PathBuf},
};

use gl::types::GLuint;
use image::GenericImage;

use crate::GraphicsObject;

#[allow(unused)]
pub struct Texture {
    id: GLuint,
    path: PathBuf,
}

impl GraphicsObject for Texture {
    unsafe fn with<F>(&self, mut f: F)
    where
        F: FnMut(),
    {
        // BIND
        gl::BindTexture(gl::TEXTURE_2D, self.id);
        // call `f`
        f();
        // UNBIND
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }
}

impl Texture {
    pub unsafe fn new(path: &Path) -> Self {
        // ID
        let mut texture: GLuint = 0;
        gl::GenTextures(1, &mut texture);

        let tex = Self {
            id: texture,
            path: path.to_owned(),
        };

        tex.with(|| {
            // Operate on BOUND texture
            // Texture wrapping
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::MIRRORED_REPEAT as i32,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // Texture filtering
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            // Mipmap filtering
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        });

        tex.load();

        tex
    }

    pub unsafe fn load(&self) {
        self.with(|| {
            let img = image::open(&self.path)
                .expect(&format!("Failed to load Texture at {:?}", &self.path))
                .flipv();
            let data = img.raw_pixels();

            let color_type = match img.color() {
                image::ColorType::RGB(_) => gl::RGB,
                image::ColorType::RGBA(_) => gl::RGBA,
                _ => panic!(),
            };

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                color_type as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                color_type,
                gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        });
    }
    
    pub fn id(&self) -> u32 {
        self.id
    }
}
