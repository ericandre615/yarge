pub struct SystemInfo {
    max_texture: gl::types::GLint,
}

impl SystemInfo {
    pub fn get_max_textures() -> gl::types::GLint {
        let mut max_textures: gl::types::GLint = 0;

        unsafe {
            gl::GetIntegerv(gl::MAX_TEXTURE_IMAGE_UNITS, &mut max_textures);
        }

        max_textures
    }
}

