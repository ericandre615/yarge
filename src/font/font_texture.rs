#[derive(Debug)]
pub struct FontTexture {
    texture_handle: gl::types::GLuint,
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
    pub fn new(screen_width: u32, screen_height: u32) -> FontTexture {
        let texture_handle = create_font_texture(screen_width, screen_height);

        FontTexture {
            texture_handle,
        }
    }

    pub fn get_texture_handle(&self) -> gl::types::GLuint {
        self.texture_handle
    }

    pub fn update(&self, glyph: &GlyphTexture) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_handle);

            gl::TexSubImage2D(
                gl::TEXTURE_2D,// GLenum target,
                0,// GLint level,
                glyph.left as i32,// GLint xoffset,
                glyph.bottom as i32,// GLint yoffset,
                glyph.width as i32,// GLsizei width,
                glyph.height as i32,// GLsizei height,
                gl::RGBA,//?// GLenum format,
                gl::UNSIGNED_BYTE,//?// GLenum type,
                glyph.data.as_ptr() as *const gl::types::GLvoid,// const GLvoid * datav
            );

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn set_size(&self, width: u32, height: u32) {
        self.bind();
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                std::ptr::null() //NULL
            );
        }
        self.unbind();
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_handle);
        }
    }

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

fn create_font_texture(screen_width: u32, screen_height: u32) -> gl::types::GLuint {
    let mut texture_handle: gl::types::GLuint = 0;

    unsafe {
        gl::ActiveTexture(gl::TEXTURE0);
        gl::GenTextures(1, &mut texture_handle);
        gl::BindTexture(gl::TEXTURE_2D, texture_handle);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            screen_width as i32,
            screen_height as i32,
            0,
            gl::RGBA as u32,
            gl::UNSIGNED_BYTE,
            std::ptr::null() // NULL // 0? null ptr? this is never clear around here
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

