pub mod transform;

use image::{ImageResult, DynamicImage, GenericImageView};

use crate::resources::*;

pub struct TextureBuilder<'a> {
    resource: &'a Resources,
    image_path: String,
    texture_slot: u32,
}

impl<'a> TextureBuilder<'a> {
    pub fn new(res: &'a Resources, image_path: String) -> Self {
        Self {
            resource: res,
            image_path: image_path,
            texture_slot: 0,
        }
    }

    pub fn with_texture_slot(mut self, texture_slot: u32) -> Self {
        self.texture_slot = texture_slot;
        self
    }

    pub fn build(self) -> Result<Texture, failure::Error> {
        Texture::new(self.resource, self.image_path, self.texture_slot)
    }
}

pub struct Texture {
    texture_handle: gl::types::GLuint,
    texture_offset: gl::types::GLuint,
    image_data: DynamicImage,
    image_path: String,
}

impl Texture {
    pub fn new(res: &Resources, image_path: String, texture_slot: u32) -> Result<Texture, failure::Error> {
        let image_data = res.load_image_from_path(&image_path)?;
        let image_rgba = image_data.to_rgba();
        let (iw, ih) = image_data.dimensions();
        let texture_offset = texture_slot;// as gl::types::GLuint;

        let texture_handle = create_texture(iw, ih, &image_rgba.to_vec(), texture_offset);

        Ok(Texture {
            texture_handle,
            texture_offset,
            image_data,
            image_path,
        })
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        self.image_data.dimensions()
    }

    pub fn get_texture_offset(&self) -> u32 {
        self.texture_offset
    }

    pub fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + self.texture_offset as gl::types::GLuint);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_handle);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

fn create_texture(width: u32, height: u32, image_raw: &Vec<u8>, texture_offset: u32) -> gl::types::GLuint {
    let image_ptr = image_raw.as_ptr() as *const gl::types::GLvoid;
    let mut texture_handle: gl::types::GLuint = 1;

    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        gl::ActiveTexture(gl::TEXTURE0 + texture_offset as gl::types::GLuint);

        gl::GenTextures(1, &mut texture_handle);
        gl::BindTexture(gl::TEXTURE_2D, texture_handle);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        // TODO: debug
        let colour = vec![0.0, 0.0, 1.0, 1.0];
        gl::TexParameterfv(gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, colour.as_ptr() as *const f32);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            image_ptr
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);

        gl::BindTexture(gl::TEXTURE_2D, 0);

        texture_handle
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.texture_handle);
        }
    }
}

