use crate::textures::transform;

use image::{ImageResult, DynamicImage, GenericImageView};

use std::fmt;

use crate::resources::*;

// TODO: dont really want to let cloning/copying, temporary for WIP
#[derive(Clone)]
pub struct Texture {
    pub texture_handle: gl::types::GLuint,
    pub image_data: DynamicImage,
    pub image_path: String,
}

impl fmt::Debug for Texture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture Debug not implemented")
        //match self {
        //    &
        //}
    }
}

impl PartialEq for Texture {
    fn eq(&self, other: &Self) -> bool {
        self.texture_handle == other.texture_handle
    }
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

    pub fn get_texture_handle(&self) -> gl::types::GLuint {
        self.texture_handle
    }

    pub fn bind_to_unit(&self, slot: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot as gl::types::GLuint);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_handle);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_handle);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

fn create_texture(width: u32, height: u32, image_raw: &Vec<u8>) -> gl::types::GLuint {
    let image_ptr = image_raw.as_ptr() as *const gl::types::GLvoid;
    let mut texture_handle: gl::types::GLuint = 1;

    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        gl::GenTextures(1, &mut texture_handle);
        gl::BindTexture(gl::TEXTURE_2D, texture_handle);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        // TODO: debug
        //let colour = vec![0.0, 0.0, 1.0, 1.0];
        //gl::TexParameterfv(gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, colour.as_ptr() as *const f32);

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

