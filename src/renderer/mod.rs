pub mod layers;

use std::collections::HashMap;

use crate::resources::*;
use layers::*;
use crate::helpers::{self, data, buffer, system};
use crate::camera::*;
use crate::sprite::{Sprite};

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
    program: helpers::Program,
    vertices: Vec<BatchVertex>,
    indices: Vec<[i32; 6]>,
    layers: Layers,
    vbo: buffer::DynamicArrayBuffer,
    vao: buffer::VertexArray,
    ibo: buffer::DynamicElementArrayBuffer,
    clear_color: (u8, u8, u8, f32),
    max_textures: gl::types::GLint,
    uniforms: HashMap<String, i32>,
}

impl Renderer2D {
    pub fn new(res: &Resources) -> Result<Renderer2D, failure::Error> {
        let default_clear_color = (255, 255, 255, 1.0);
        let max_buffer_size = ((::std::mem::size_of::<BatchVertex>()) * 1000) as gl::types::GLsizeiptr;
        let max_index_size = ((::std::mem::size_of::<[u32; 6]>()) * 2000) as gl::types::GLsizeiptr;
        let max_textures = system::SystemInfo::get_max_textures();
        let program = helpers::Program::from_resource(res, "shaders/batch")?;
        let uniform_textures = program.get_uniform_location("Textures")?;

        Ok(Renderer2D {
            program,
            vertices: Vec::new(),
            indices: Vec::new(),
            layers: Layers::new(),
            vbo: buffer::DynamicArrayBuffer::new(max_buffer_size),
            vao: buffer::VertexArray::new(),
            ibo: buffer::DynamicElementArrayBuffer::new(max_index_size),
            clear_color: default_clear_color,
            max_textures,
            uniforms: vec![
                ("texture".to_owned(), uniform_textures),
            ].into_iter().collect()
        })
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

    pub fn submit(sprites: Vec<Sprite>) {
        // TODO: submit sprites to draw
        // query from sprite the texture, transform, Vertex data? wait, that's all in the vertex data?

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

fn generate_batch_indices(vertices_len: usize) -> Vec<[i32; 6]> {
    let mut offset: i32 = 0;
    let mut indices: Vec<[i32; 6]> = Vec::new();

    for i in (0..vertices_len) {
        let mut group: [i32; 6] = [
            offset + 0,
            offset + 1,
            offset + 2,
            offset + 2,
            offset + 3,
            offset + 0,
        ];

        indices.push(group);

        offset += 4;
    }

    indices
}

fn generate_texture_slots(max: i32) -> Vec<u32> {
    let mut texture_slots = Vec::new();

    for i in 0..max {
        texture_slots.push(gl::TEXTURE0 + i as u32);
    }

    texture_slots
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_generate_indices() {
        let expected_len = 4;
        let test_vec = vec![0; expected_len];
        let expected_indices = vec![
            [0, 1, 2, 2, 3, 0],
            [4, 5, 6, 6, 7, 4],
            [8, 9, 10, 10, 11, 8],
            [12, 13, 14, 14, 15, 12]
        ];

        let indices = generate_batch_indices(test_vec.len());

        assert_eq!(expected_len, indices.len());
        assert_eq!(expected_indices, indices);
    }

    #[test]
    fn can_generate_texture_slots() {
        let max = 4;
        let actual_texture_slots = generate_texture_slots(max);
        let expected_texture_slots = vec![33984, 33985, 33986, 33987];

        assert_eq!(max as usize, actual_texture_slots.len());
        assert_eq!(expected_texture_slots, actual_texture_slots);
    }
}
