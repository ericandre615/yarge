use image::{ImageResult, DynamicImage, GenericImageView};

use crate::resources::*;

pub struct Texture {
    texture_handle: gl::types::GLuint,
    image_data: DynamicImage,
    image_path: String,
}

impl Texture {
    pub fn new(res: &Resources, image_path: String) -> Result<Texture, failure::Error> {
        let image_data = res.load_image_from_path(&image_path)?;
        let image_rgba = image_data.to_rgba();
        let (iw, ih) = image_data.dimensions();

        let texture_handle = create_texture(iw, ih, &image_rgba.to_vec());

        Ok(Texture {
            texture_handle,
            image_data,
            image_path,
        })
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        self.image_data.dimensions()
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_handle);
        }
    }
}

fn create_texture(width: u32, height: u32, image_raw: &Vec<u8>) -> gl::types::GLuint {
    let image_ptr = image_raw.as_ptr() as *const gl::types::GLvoid;
    let mut texture_handle: gl::types::GLuint = 0;

    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        gl::GenTextures(1, &mut texture_handle);
        gl::BindTexture(gl::TEXTURE_2D, texture_handle);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

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

        texture_handle
    }
}
