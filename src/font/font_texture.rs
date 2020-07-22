#[derive(Debug)]
pub struct FontTexture {
    texture_handle: gl::types::GLuint,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct GlyphTexture<'a> {
    pub left: u32,
    pub bottom: u32,
    pub width: u32,
    pub height: u32,
    pub data: &'a [u8],
}

impl FontTexture {
    pub fn new(cache_width: u32, cache_height: u32) -> FontTexture {
        let texture_handle = create_font_texture(cache_width, cache_height);

        FontTexture {
            texture_handle,
            width: cache_width,
            height: cache_height,
        }
    }

    //pub fn get_texture_handle(&self) -> gl::types::GLuint {
    //    self.texture_handle
    //}

    pub fn update(&self, glyph: &GlyphTexture) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_handle);

            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::TexSubImage2D(
                gl::TEXTURE_2D,// GLenum target,
                0,// GLint level,
                glyph.left as i32,// GLint xoffset,
                glyph.bottom as i32,// GLint yoffset,
                glyph.width as i32,// GLsizei width,
                glyph.height as i32,// GLsizei height,
                gl::RED,// GLenum format, // gl::ALPHA may also work depending on shader
                gl::UNSIGNED_BYTE,//?// GLenum type,
                glyph.data.as_ptr() as *const gl::types::GLvoid,
            );

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    //pub fn bind(&self) {
    //    unsafe {
    //        gl::BindTexture(gl::TEXTURE_2D, self.texture_handle);
    //    }
    //}

    pub fn bind_to_unit(&self, slot: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot as gl::types::GLuint);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_handle);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

fn create_font_texture(cache_width: u32, cache_height: u32) -> gl::types::GLuint {
    let mut texture_handle: gl::types::GLuint = 0;

    unsafe {
        gl::ActiveTexture(gl::TEXTURE0);

        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        gl::GenTextures(1, &mut texture_handle);
        gl::BindTexture(gl::TEXTURE_2D, texture_handle);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);//gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);//gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 0);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            cache_width as i32,
            cache_height as i32,
            0,
            gl::RGBA as u32,
            gl::UNSIGNED_BYTE,
            std::ptr::null(),
        );

        gl::BindTexture(gl::TEXTURE_2D, 0);
    }

    texture_handle
}

impl Drop for FontTexture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.texture_handle);
        }
    }
}

