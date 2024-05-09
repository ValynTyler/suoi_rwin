use std::path::{Path, PathBuf};

use gl::types::GLuint;

#[allow(unused)]
pub struct Texture {
    id: GLuint,
    path: PathBuf,
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

        tex
    }

    // pub unsafe fn load(&self) {
    //     // BIND
    //     gl::BindTexture(gl::TEXTURE_2D, self.id);

    //     let image = GLImage::load(&self.path);
    //     let data = image.pixel_data();

    //     gl::TexImage2D(
    //         gl::TEXTURE_2D,
    //         0,
    //         gl::RGB as i32,
    //         image.width() as i32,
    //         image.height() as i32,
    //         0,
    //         gl::RGB,
    //         gl::UNSIGNED_BYTE,
    //         &data[0] as *const u8 as *const c_void,
    //     );
    //     gl::GenerateMipmap(gl::TEXTURE_2D);

    //     // UNBIND
    //     gl::BindTexture(gl::TEXTURE_2D, 0);
    // }

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
