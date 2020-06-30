mod triangle_shaders;

use crate::helpers::{self, data, buffer};
use crate::resources::*;

use triangle_shaders::{VERTEX_SOURCE, FRAGMENT_SOURCE};

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    clr: data::u2_u10_u10_u10_rev_float,
}

pub struct Triangle {
    program: helpers::Program,
    _vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
}

impl Triangle {
    pub fn new(res: &Resources) -> Result<Triangle, failure::Error> {
        let shaders = vec![
            helpers::Shader::from_raw(&VERTEX_SOURCE, gl::VERTEX_SHADER)?,
            helpers::Shader::from_raw(&FRAGMENT_SOURCE, gl::FRAGMENT_SHADER)?,
        ];
        let program = helpers::Program::from_shaders(&shaders[..], "internal/shaders/triangle")
            .expect("Failed to create Triangle Shader Program");
        let vertices: Vec<Vertex> = vec![
            // positions        // colors
           Vertex { pos: (-0.5, -0.5, 0.0).into(), clr: (1.0, 0.0, 0.0, 1.0).into() }, // bottom right
           Vertex { pos: (0.5, -0.5, 0.0).into(), clr: (0.0, 1.0, 0.0, 1.0).into() }, // bottom left
           Vertex { pos: (0.0, 0.5, 0.0).into(), clr: (0.0, 0.0, 1.0, 1.0).into() } // top
        ];

        let vbo = buffer::ArrayBuffer::new();

        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        let vao = buffer::VertexArray::new();

        vao.bind();
        vbo.bind();

        Vertex::vertex_attrib_pointers();

        vbo.unbind();
        vao.unbind();

        Ok(Triangle {
            program,
            _vbo: vbo,
            vao,
        })
    }

    pub fn render(&self) {
        self.program.set_used();
        self.vao.bind();

        unsafe {
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                3
            );
        }
    }
}

