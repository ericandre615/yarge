use image::{ImageResult, DynamicImage, GenericImageView};
use std::ffi::{CString, c_void};

use crate::helpers::{self, data, buffer};
use crate::resources::*;
use crate::texture::{Texture};
use crate::camera::{Camera};

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
    attrib_texcoord_location: i32,
    uniform_mvp: i32,
    uniform_color: i32,
    image: ImageProps,
    indicies: Vec<u32>,
    texture: Texture,
    color: (f32, f32, f32, f32),
    model: glm::TMat4<f32>,
}

impl Image {
    pub fn new(res: &Resources, image: ImageProps) -> Result<Image, failure::Error> {
        let program = helpers::Program::from_resource(res, "shaders/image")?;
        let attrib_texcoord_location = program.get_attrib_location("TexCoord")?;
        let uniform_mvp = program.get_uniform_location("MVP")?;
        let uniform_color = program.get_uniform_location("TexColor")?;
        let texture = Texture::new(res, image.img_path.to_string())?;
        let color = (1.0, 1.0, 1.0, 1.0);
        let (x, y) = (0.0, 0.0);
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

        let (x, y) = image.pos;
        let pos = glm::vec3(x, y, 0.0);
        let model = glm::translate(&glm::identity(), &pos);

        Ok(Image {
            program,
            _vbo: vbo,
            vao,
            image,
            attrib_texcoord_location,
            uniform_mvp,
            uniform_color,
            indicies,
            texture,
            color,
            model,
        })
    }

    pub fn get_position(&self) -> (f32, f32) {
        self.image.pos
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        let pos = glm::vec3(x, y, 0.0);
        self.image.pos = (x, y);
        self.model = glm::translate(&glm::identity(), &pos);
    }

    pub fn set_posX(&mut self, x: f32) {
        let (_x, y) = self.image.pos;
        let pos = glm::vec3(x, y, 0.0);
        self.image.pos = (x, y);
        self.model = glm::translate(&glm::identity(), &pos);
    }

    pub fn set_posY(&mut self, y: f32) {
        let (x, _y) = self.image.pos;
        let pos = glm::vec3(x, y, 0.0);
        self.image.pos = (x, y);
        self.model = glm::translate(&glm::identity(), &pos);
    }

    pub fn set_color(&mut self, color: (f32, f32, f32, f32)) {
        self.color = color;
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        let (r, g, b, _a) = self.color;
        self.color = (r, g, b, alpha);
    }

    pub fn flip_h(&mut self) {
        self.model = glm::scale(&self.model, &glm::vec3(-1.0, 1.0, 1.0));
    }

    pub fn flip_v(&mut self) {
        self.model = glm::scale(&self.model, &glm::vec3(1.0, -1.0, 1.0));
    }

    pub fn get_model(&self) -> glm::TMat4<f32> {
        self.model
    }

    pub fn render(&self, camera: &Camera) {
        let mvp = camera.get_projection() * camera.get_view() * self.model;
        // call BindTexture again for render to draw the right image for each image/object
        self.texture.bind();

        self.program.set_used();
        self.program.set_uniform_4f(self.uniform_color, self.color);
        self.program.set_uniform_mat4f(self.uniform_mvp, &mvp);
        self.vao.bind();

        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                self.indicies.as_ptr() as *const gl::types::GLvoid
            );
        }
    }
}

