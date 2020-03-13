pub mod layers;

use std::collections::HashMap;

use crate::resources::*;
use layers::*;
use crate::helpers::{self, data, buffer, system};
use crate::camera::*;
use crate::sprite::{Sprite};

#[derive(VertexAttribPointers)]
#[derive(Debug)]
#[repr(C, packed)]
pub struct BatchVertex {
    #[location=0]
    pos: data::f32_f32_f32,
    #[location=1]
    tex: data::f32_f32,
    #[location=2]
    color: data::f32_f32_f32_f32,
    #[location=3]
    tex_id: data::f32_,
}

pub struct Renderer2D<'s> {
    program: helpers::Program,
    vertices: Vec<Vec<BatchVertex>>,
    indices: Vec<[i32; 6]>,
    sprites: Vec<&'s Sprite>,
    layers: Layers,
    vbo: buffer::DynamicArrayBuffer,
    vao: buffer::VertexArray,
    ibo: buffer::ElementArrayBuffer,
    clear_color: (u8, u8, u8, f32),
    max_textures: gl::types::GLint,
    max_sprites: usize,
    texture_slots: Vec<i32>,
    uniforms: HashMap<String, i32>,
}

impl<'s> Renderer2D<'s> {
    pub fn new(res: &Resources) -> Result<Renderer2D, failure::Error> {
        let default_clear_color = (255, 255, 255, 1.0);
        let max_buffer_size = ((::std::mem::size_of::<BatchVertex>()) * 4000) as gl::types::GLsizeiptr;
        let max_sprites = 1000;
        let max_index_size = ((::std::mem::size_of::<[u32; 6]>()) * 4000) as gl::types::GLsizeiptr;
        let max_textures = system::SystemInfo::get_max_textures();
        let program = helpers::Program::from_resource(res, "shaders/batch")?;
        let uniform_textures = program.get_uniform_location("Textures")?;
        let uniform_mvp = program.get_uniform_location("MVP")?;
        let texture_slots = Vec::with_capacity(max_textures as usize);

        let vbo = buffer::DynamicArrayBuffer::new(max_buffer_size);
        let vao = buffer::VertexArray::new();
        let ibo = buffer::ElementArrayBuffer::new();

        let indices = generate_batch_indices(max_sprites);

        vbo.bind();
        vbo.set_buffer_data();
        vbo.unbind();

        vao.bind();
        vbo.bind();

        BatchVertex::vertex_attrib_pointers();

        ibo.bind();
        ibo.static_draw_data(&indices);
        ibo.unbind();

        vbo.unbind();
        vao.unbind();

        Ok(Renderer2D {
            program,
            vertices: Vec::new(),
            indices,
            sprites: Vec::new(),
            layers: Layers::new(),
            vbo,
            vao,
            ibo,
            clear_color: default_clear_color,
            max_textures,
            max_sprites,
            texture_slots,
            uniforms: vec![
                ("textures".to_owned(), uniform_textures),
                ("mvp".to_owned(), uniform_mvp),
            ].into_iter().collect(),
        })
    }

    pub fn begin_scene(camera: &Camera) {
    }

    pub fn end_scene() {

    }

    pub fn begin_batch(&mut self) {
        // TODO: reset pointer into the vertex data buffer
        // keep track of where we are in the buffer to put data
    }

    pub fn end_batch(&mut self) {
        self.vbo.bind();
        self.vbo.reset_buffer_offset();

        for sprite in &self.sprites {
            let sprite_vertices = sprite.get_vertices();
            let sprite_texture_handle = sprite.texture.get_texture_handle() as i32;
            let sprite_tex_id = self.texture_slots.iter().position(|&id| id == sprite_texture_handle).unwrap_or(0); // should use a single reserved slot for blank white texture or a debug texture

            let mut batch_vertices: Vec<BatchVertex> = Vec::new();
            for vertex in sprite_vertices {
                batch_vertices.push(
                    BatchVertex {
                        pos: vertex.get_pos(),
                        tex: vertex.get_tex(),
                        color: vertex.get_color(),
                        tex_id: (sprite_tex_id as u32).into(),
                    }
                );
            };

            //TODO: might need to have some conversion from SpriteVertex to BatchVertex
            self.vbo.upload_draw_data(&batch_vertices);
            self.vbo.set_buffer_offset(self.vbo.buffer_offset + ((::std::mem::size_of::<BatchVertex>()) * 4) as isize);

            self.vertices.push(batch_vertices);
        }

        self.vbo.reset_buffer_offset();
        self.vbo.unbind();
    }

    pub fn submit(&mut self, sprite: &'s Sprite) {  // TODO: testing, should be able to submit many sprites at once
        let has_sprite = self.sprites.contains(&sprite);

        if has_sprite {
            println!("Sprite already submitted");
            return ();
        }

        if self.sprites.len() >= self.max_sprites {
            // need to reset/end/flush/render and start a new batch
        }

        self.sprites.push(sprite); // TODO: sprite should be reference?

        let sprite_texture_handle = sprite.texture.get_texture_handle() as i32;
        let tex_id: i32 = match self.texture_slots.binary_search(&sprite_texture_handle) {
            Ok(tid) => tid as i32,
            Err(next_id) if self.texture_slots.len() >= self.max_textures as usize => {
                // we dont have this texture, but we have no space
                // we need to flush/end/render, and start again

                -1
            },
            Err(next_id) => {
                // we don't have this, but we have space to add it
                self.texture_slots.push(sprite_texture_handle);

                next_id as i32
            },
        };
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

    pub fn render(&mut self, camera: &Camera) {
        //self.clear();
        let mvp = camera.get_projection() * camera.get_view();

        self.vao.bind();

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        for (i, handle) in self.texture_slots.iter().enumerate() {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);
                gl::BindTexture(gl::TEXTURE_2D, *handle as u32);
            }
        }

        self.program.set_used();
        self.program.set_uniform_mat4f(*self.uniforms.get("mvp").unwrap(), &mvp);
        self.program.set_uniform_1iv(
            *self.uniforms.get("textures").unwrap(),
            &generate_texture_slots(self.max_textures)
        );

        self.vao.bind();

        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32 * 6,
                gl::UNSIGNED_INT,
                self.indices.as_ptr() as *const gl::types::GLvoid
            );
        }

        self.vao.unbind();

        self.sprites = Vec::new();
        self.texture_slots = Vec::new();
    }
}

fn generate_batch_indices(vertices_len: usize) -> Vec<[i32; 6]> {
    let mut offset: i32 = 0;
    let mut indices: Vec<[i32; 6]> = Vec::new();

    // TODO: maybe take in a format or base it off given vertices?
    // as this needs to match the order of a sprites vertices
    // this order is more of a top left to bottom right
    for i in (0..vertices_len) {
        let group: [i32; 6] = [
            offset + 0,
            offset + 1,
            offset + 2,
            offset + 2,
            offset + 1,
            offset + 3,
        ];

        indices.push(group);

        offset += 4;
    }

    indices
}

fn generate_texture_slots(max: i32) -> Vec<i32> {
    let mut texture_slots = Vec::new();

    for i in 0..max {
        texture_slots.push(i as i32);
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
            [0, 1, 2, 2, 1, 3],
            [4, 5, 6, 6, 5, 7],
            [8, 9, 10, 10, 9, 11],
            [12, 13, 14, 14, 13, 15]
        ];

        let indices = generate_batch_indices(test_vec.len());

        assert_eq!(expected_len, indices.len());
        assert_eq!(expected_indices, indices);
    }

    #[test]
    fn can_generate_texture_slots() {
        let max = 4;
        let actual_texture_slots = generate_texture_slots(max);
        let expected_texture_slots = vec![0, 1, 2, 3];

        assert_eq!(max as usize, actual_texture_slots.len());
        assert_eq!(expected_texture_slots, actual_texture_slots);
    }
}
