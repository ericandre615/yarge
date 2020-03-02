pub mod layers;

use layers::*;
use crate::helpers::{data, buffer};
use crate::camera::*;

#[derive(VertexAttribPointers)]
#[repr(C, packed)]
struct BatchVertex {
    #[location=0]
    pos: data::f32_f32_f32,
    #[location=1]
    color: data::f32_f32_f32_f32,
    #[location=2]
    tex: data::f32_f32,
    #[location=3]
    tex_id: data::u32_,
}

struct Renderer2D {
    vertices: Vec<BatchVertex>,
    indices: Vec<buffer::ElementArrayBuffer>,
    layers: Layers,
    clear_color: (u8, u8, u8, f32),
}

impl Renderer2D {
    pub fn begin_scene(camera: &Camera) {

    }

    pub fn end_scene() {

    }

    pub fn submit(vao: BatchVertex) {

    }

    pub fn set_clear_color(r: u8, g: u8, b: u8, a: f32) {
        let rf = r as f32 / 255.0;
        let gf = g as f32 / 255.0;
        let bf = b as f32 / 255.0;

        unsafe {
            gl::ClearColor(rf, gf, bf, a);
        }
    }

    pub fn clear() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn render() {

    }
}

