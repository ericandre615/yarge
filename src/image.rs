use image::{ImageResult, DynamicImage, GenericImageView};
use std::ffi::{CString, c_void};

use crate::helpers::{self, data, buffer};
use crate::resources::*;

const IMAGE_BASE_COLOR: (f32, f32, f32, f32) = (1.0, 0.0, 0.5, 0.0);

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    tex: data::f32_f32,
}

pub struct ImageProps {
    pub pos: (f32, f32),
    pub dim: (u32, u32),
    pub img_path: String,
}

pub struct Image {
    program: helpers::Program,
    _vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
    uniform_viewport_resolution_location: i32,
    attrib_texcoord_location: i32,
    image: ImageProps,
    image_data: DynamicImage,
    id: u32,
    texture_handle: gl::types::GLuint,
}

impl Image {
    pub fn new(res: &Resources, image: ImageProps, id: u32) -> Result<Image, failure::Error> {
        let program = helpers::Program::from_resource(res, "shaders/image")?;
        let uniform_viewport_resolution_location = program.get_uniform_location("ViewportResolution")?;
        let attrib_texcoord_location = program.get_attrib_location("TexCoord")?;
        let image_data = res.load_image_from_path(&image.img_path)?;
        let image_rgba = image_data.to_rgba();
        let (iw, ih) = image_data.dimensions();
        let (x, y) = image.pos;
        let (width, height) = image.dim;
        let x2 = x + (width as f32);
        let y2 = y + (height as f32);
        let vertices: Vec<Vertex> = vec![
           Vertex { pos: (x, y, 0.0).into(), /*clr: IMAGE_BASE_COLOR.into(),*/ tex: (0.0, 0.0).into() },
           Vertex { pos: (x2, y, 0.0).into(),/* clr: IMAGE_BASE_COLOR.into(),*/ tex: (1.0, 0.0).into() },
           Vertex { pos: (x, y2, 0.0).into(),/* clr: IMAGE_BASE_COLOR.into(),*/ tex: (0.0, 1.0).into() },
           // second triangle
           Vertex { pos: (x, y2, 0.0).into(), /*clr: IMAGE_BASE_COLOR.into(),*/ tex: (0.0, 1.0).into() },
           Vertex { pos: (x2, y, 0.0).into(), /*clr: IMAGE_BASE_COLOR.into(),*/ tex: (1.0, 0.0).into() },
           Vertex { pos: (x2, y2, 0.0).into(), /*clr: IMAGE_BASE_COLOR.into(),*/ tex: (1.0, 1.0).into() }
        ]; // 2 triangles makes a rectangle
        let mut texture_handle: gl::types::GLuint = id;

        let vbo = buffer::ArrayBuffer::new();

        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        let vao = buffer::VertexArray::new();

        vao.bind();
        vbo.bind();

       // let mut ebo: gl::types::GLuint = 0;
       // let elements = vec![
       //     0, 1, 2,
       //     2, 3, 0
       // ];
       // unsafe {
       //     gl::GenBuffers(1, &mut ebo);
       //     gl::BindBuffer(
       //         gl::ELEMENT_ARRAY_BUFFER,
       //         ebo
       //     );
       //     gl::BufferData(
       //         gl::ELEMENT_ARRAY_BUFFER,
       //         (elements.len() * ::std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
       //         elements.as_ptr() as *const gl::types::GLvoid,
       //         gl::STATIC_DRAW
       //     );
       // }

        Vertex::vertex_attrib_pointers();

        let _tex = create_texture(&image, iw, ih, &image_rgba.to_vec(), texture_handle);//.raw_pixels());

        vbo.unbind();
        vao.unbind();

        Ok(Image {
            program,
            _vbo: vbo,
            vao,
            image,
            uniform_viewport_resolution_location,
            attrib_texcoord_location,
            image_data,
            id,
            texture_handle: _tex,
        })
    }

    pub fn render(&self, viewport: &helpers::Viewport) {
        let viewport_dimensions = nalgebra::Vector2::new(viewport.w as f32, viewport.h as f32);

        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.texture_handle); }

        self.program.set_used();
        self.program.set_uniform_2f(self.uniform_viewport_resolution_location, &viewport_dimensions);
        self.vao.bind();

        unsafe {
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                6 // 3 per triangle
            );
        }
    }
}

fn create_texture(image: &ImageProps, width: u32, height: u32, image_raw: &Vec<u8>, mut texture_handle: gl::types::GLuint) -> gl::types::GLuint {
    //let mut texture_handle: gl::types::GLuint = id;
    let image_ptr = image_raw.as_ptr() as *const gl::types::GLvoid;

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
