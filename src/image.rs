use image::{ImageResult, DynamicImage, GenericImageView};
use std::ffi::{CString, c_void};

use crate::helpers::{self, data, buffer};
use crate::resources::*;
use crate::texture::{Texture};
use crate::texture::transform::{TextureTransform};
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

#[derive(Clone, Debug)]
pub struct ImageProps {
    pub pos: (f32, f32),
    pub dim: (u32, u32),
    pub img_path: String,
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Normal,
    Flipped,
}

pub struct Orientation {
    horizontal: Direction,
    vertical: Direction,
}

pub struct Image {
    program: helpers::Program,
    _vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
    attrib_texcoord_location: i32,
    uniform_mvp: i32,
    uniform_color: i32,
    uniform_texcoord_transform: i32,
    image: ImageProps,
    frame: (i32, i32),
    indicies: Vec<u32>,
    texture: Texture,
    texture_transform: TextureTransform,
    color: (f32, f32, f32, f32),
    model: glm::TMat4<f32>,
    orientation: Orientation,
}

impl Image {
    pub fn new(res: &Resources, image: ImageProps) -> Result<Image, failure::Error> {
        let program = helpers::Program::from_resource(res, "shaders/image")?;
        let attrib_texcoord_location = program.get_attrib_location("TexCoord")?;
        let uniform_mvp = program.get_uniform_location("MVP")?;
        let uniform_color = program.get_uniform_location("TexColor")?;
        let uniform_texcoord_transform = program.get_uniform_location("TexCoordTransform")?;
        let texture = Texture::new(res, image.img_path.to_string())?;
        let (tw, th) = texture.get_dimensions();
        let (x, y) = image.pos;
        let (width, height) = image.dim;
        let x2 = x + (width as f32);
        let y2 = y + (height as f32);
        let tx = width as f32 / tw as f32;
        let ty = height as f32 / th as f32;
        let vertices: Vec<Vertex> = vec![
           Vertex { pos: (x, y, 0.0).into(), tex: (0.0, 0.0).into() },
           Vertex { pos: (x2, y, 0.0).into(), tex: (tx, 0.0).into() },
           Vertex { pos: (x, y2, 0.0).into(), tex: (0.0, ty).into() },
           // second triangle
           Vertex { pos: (x2, y2, 0.0).into(), tex: (tx, ty).into() }
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
            uniform_texcoord_transform,
            indicies,
            texture,
            texture_transform: TextureTransform::new(tw, th),
            color: normalize_rgba(255, 255, 255, 1.0),
            model,
            frame: (0, 0),
            orientation: Orientation {
                horizontal: Direction::Normal,
                vertical: Direction::Normal,
            },
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

    pub fn set_pos_x(&mut self, x: f32) {
        let (_x, y) = self.image.pos;
        let pos = glm::vec3(x, y, 0.0);
        self.image.pos = (x, y);
        self.model = glm::translate(&glm::identity(), &pos);
    }

    pub fn set_pos_y(&mut self, y: f32) {
        let (x, _y) = self.image.pos;
        let pos = glm::vec3(x, y, 0.0);
        self.image.pos = (x, y);
        self.model = glm::translate(&glm::identity(), &pos);
    }

    pub fn set_color(&mut self, color: (u8, u8, u8, f32)) {
        let (r, g, b, a) = color;
        self.color = normalize_rgba(r, g, b, a);
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        let (r, g, b, _a) = self.color;
        self.color = (r, g, b, alpha);
    }

    pub fn flip_h(&mut self) {
        self.orientation = Orientation {
            horizontal: Direction::Flipped,
            vertical: self.orientation.vertical,
        };
    }

    pub fn flip_v(&mut self) {
        self.orientation = Orientation {
            horizontal: self.orientation.horizontal,
            vertical: Direction::Flipped,
        }
    }

    pub fn set_orientation(&mut self, h: Direction, v: Direction) {
        self.orientation = Orientation {
            horizontal: h,
            vertical: v,
        };
    }

    pub fn get_model(&self) -> glm::TMat4<f32> {
        self.model
    }

    pub fn set_frame(&mut self, frame: (i32, i32)) {
        let (fx, fy) = frame;
        self.frame = frame;
        self.texture_transform.set_frame(fx as f32, fy as f32);
    }

    pub fn set_texture_scale(&mut self, x: f32, y: f32) {
        self.texture_transform.set_scale(x, y);
    }

    pub fn render(&self, camera: &Camera, dt: f32) {
        let scale_x = match self.orientation.horizontal {
            Direction::Normal => 1.0,
            Direction::Flipped => -1.0,
        };
        let scale_y = match self.orientation.vertical {
            Direction::Normal => 1.0,
            Direction::Flipped => -1.0,
        };
        let model = glm::scale(&self.model, &glm::vec3(scale_x, scale_y, 1.0));
        let mvp = camera.get_projection() * camera.get_view() * model;//self.model;
        let texcoord_transform = self.texture_transform.get_transform();

        // call BindTexture again for render to draw the right image for each image/object
        self.texture.bind();
        self.program.set_used();
        self.program.set_uniform_4f(self.uniform_color, self.color);
        self.program.set_uniform_mat4f(self.uniform_texcoord_transform, &texcoord_transform);
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

fn normalize_color(color: u8) -> f32 {
    color as f32 / 255.0
}

fn normalize_rgba(r: u8, g: u8, b: u8, a: f32) -> (f32, f32, f32, f32) {
    (normalize_color(r), normalize_color(g), normalize_color(b), a)
}

