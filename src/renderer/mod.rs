pub mod layers;

use layers::*;
use crate::helpers::{data, buffer, system};
use crate::camera::*;

#[derive(VertexAttribPointers)]
#[repr(C, packed)]
pub struct BatchVertex {
    #[location=0]
    pos: data::f32_f32_f32,
    #[location=1]
    color: data::f32_f32_f32_f32,
    #[location=2]
    tex: data::f32_f32,
    #[location=3]
    tex_id: data::u32_,
}

pub struct Renderer2D {
    vertices: Vec<BatchVertex>,
    indices: Vec<[i32; 6]>,
    layers: Layers,
    vbo: buffer::DynamicArrayBuffer,
    vao: buffer::VertexArray,
    ibo: buffer::DynamicElementArrayBuffer,
    clear_color: (u8, u8, u8, f32),
    max_textures: gl::types::GLint,
}

impl Renderer2D {
    pub fn new() -> Renderer2D {
        let default_clear_color = (255, 255, 255, 1.0);
        let max_buffer_size = ((::std::mem::size_of::<BatchVertex>()) * 1000) as gl::types::GLsizeiptr;
        let max_index_size = ((::std::mem::size_of::<[u32; 6]>()) * 2000) as gl::types::GLsizeiptr;
        let max_textures = system::SystemInfo::get_max_textures();

        Renderer2D {
            vertices: Vec::new(),
            indices: Vec::new(),
            layers: Layers::new(),
            vbo: buffer::DynamicArrayBuffer::new(max_buffer_size),
            vao: buffer::VertexArray::new(),
            ibo: buffer::DynamicElementArrayBuffer::new(max_index_size),
            clear_color: default_clear_color,
            max_textures,
        }
    }

    pub fn begin_scene(camera: &Camera) {
    }

    pub fn end_scene() {

    }

    pub fn begin_batch() {
        // TODO: reset pointer into the vertex data buffer
        // keep track of where we are in the buffer to put data
    }

    pub fn end_batch() {

    }

    pub fn submit(vertex_data: BatchVertex) {

    }

    pub fn set_clear_color(&mut self, r: u8, g: u8, b: u8, a: f32) {
        let rf = r as f32 / 255.0;
        let gf = g as f32 / 255.0;
        let bf = b as f32 / 255.0;

        unsafe {
            gl::ClearColor(rf, gf, bf, a);
        }
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn render(&self) {
        self.clear();
        println!("RENDERING 2D");
        println!("MAX_TEXTURES {:?}", self.max_textures);

        //let scale_x = match self.orientation.horizontal {
        //    Direction::Normal => 1.0,
        //    Direction::Flipped => -1.0,
        //};
        //let scale_y = match self.orientation.vertical {
        //    Direction::Normal => 1.0,
        //    Direction::Flipped => -1.0,
        //};
        //let model = glm::scale(&self.model, &glm::vec3(scale_x, scale_y, 1.0));
        //let mvp = camera.get_projection() * camera.get_view() * model;//self.model;
        //let texcoord_transform = self.texture_transform.get_transform();

        //// call BindTexture again for render to draw the right image for each image/object
        //self.texture.bind();
        //self.program.set_used();
        //self.program.set_uniform_4f(self.uniform_color, self.color);
        //self.program.set_uniform_mat4f(self.uniform_texcoord_transform, &texcoord_transform);
        //self.program.set_uniform_mat4f(self.uniform_mvp, &mvp);
        //self.vao.bind();

        //unsafe {
        //    gl::DrawElements(
        //        gl::TRIANGLES,
        //        6,
        //        gl::UNSIGNED_INT,
        //        self.indicies.as_ptr() as *const gl::types::GLvoid
        //    );
        //}
    }
}

