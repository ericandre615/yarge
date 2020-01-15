use image::{ImageResult, DynamicImage, GenericImageView};
use std::ffi::{CString, c_void};

use crate::helpers::{self, data, buffer};
use crate::resources::*;
use crate::texture::{Texture};

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
    indicies: Vec<u32>,
    texture: Texture,
}

impl Image {
    pub fn new(res: &Resources, image: ImageProps) -> Result<Image, failure::Error> {
        let program = helpers::Program::from_resource(res, "shaders/image")?;
        let uniform_viewport_resolution_location = program.get_uniform_location("ViewportResolution")?;
        let attrib_texcoord_location = program.get_attrib_location("TexCoord")?;
        let texture = Texture::new(res, image.img_path.to_string())?;
        let (x, y) = image.pos;
        let (width, height) = image.dim;
        let x2 = x + (width as f32);
        let y2 = y + (height as f32);
        let vertices: Vec<Vertex> = vec![
           Vertex { pos: (x, y, 0.0).into(), tex: (0.0, 0.0).into() },
           Vertex { pos: (x2, y, 0.0).into(), tex: (1.0, 0.0).into() },
           Vertex { pos: (x, y2, 0.0).into(), tex: (0.0, 1.0).into() },
           // second triangle
           Vertex { pos: (x2, y2, 0.0).into(), tex: (1.0, 1.0).into() }
        ];
        let indicies = vec![
            0, 1, 2,
            2, 1, 3,
        ];

        let vbo = buffer::ArrayBuffer::new();
        let ibo = buffer::ElementArrayBuffer::new();

        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        let vao = buffer::VertexArray::new();

        vao.bind();
        vbo.bind();

        Vertex::vertex_attrib_pointers();

        ibo.bind();
        ibo.static_draw_data(&indicies);
        ibo.unbind();

        vbo.unbind();
        ibo.unbind();
        vao.unbind();

        Ok(Image {
            program,
            _vbo: vbo,
            vao,
            image,
            uniform_viewport_resolution_location,
            attrib_texcoord_location,
            indicies,
            texture,
        })
    }

    pub fn render(&self, viewport: &helpers::Viewport) {
        let viewport_dimensions = nalgebra::Vector2::new(viewport.w as f32, viewport.h as f32);

        // call BindTexture again for render to draw the right image for each image/object
        self.texture.bind();

        self.program.set_used();
        self.program.set_uniform_2f(self.uniform_viewport_resolution_location, &viewport_dimensions);
        self.vao.bind();

        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                6,// # of vertices to draw
                gl::UNSIGNED_INT,
                self.indicies.as_ptr() as *const gl::types::GLvoid
            );
        }
    }
}

