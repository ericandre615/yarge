use crate::helpers::buffer::{FrameBuffer, ArrayBuffer, VertexArray};
use crate::resources::{Resources};
use crate::helpers::{self, data};
use crate::camera::{Camera};

#[derive(VertexAttribPointers)]
#[derive(Debug)]
#[repr(C, packed)]
pub struct RenderVertex {
    #[location=0]
    pos: data::f32_f32,
    #[location=1]
    uv: data::f32_f32,
}

#[derive(Debug)]
pub struct RenderTarget {
    vbo: ArrayBuffer,
    vao: VertexArray,
    pub program: helpers::Program,
    pub frame_buffer: FrameBuffer,
    vertices: Vec<RenderVertex>,
}

impl RenderTarget {
    pub fn new(screen_width: u32, screen_height: u32) -> Result<RenderTarget, failure::Error> {
        let default_program = default_program()?;
        let frame_buffer = FrameBuffer::new(screen_width, screen_height)?;
        let vbo = ArrayBuffer::new();
        let vao = VertexArray::new();

        let vertices = vec![
            RenderVertex { pos: (-1.0, 1.0).into(), uv: (0.0, 1.0).into() },
            RenderVertex { pos: (-1.0, -1.0).into(), uv: (0.0, 0.0).into() },
            RenderVertex { pos: (1.0, -1.0).into(), uv: (1.0, 0.0).into() },

            RenderVertex { pos: (-1.0, 1.0).into(), uv: (0.0, 1.0).into() },
            RenderVertex { pos: (1.0, -1.0).into(), uv: (1.0, 0.0).into() },
            RenderVertex { pos: (1.0, 1.0).into(), uv: (1.0, 1.0).into() },
        ];

        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        vao.bind();
        vbo.bind();

        RenderVertex::vertex_attrib_pointers();

        vbo.unbind();
        vao.unbind();

        Ok(RenderTarget {
            frame_buffer,
            vbo,
            vao,
            vertices,
            program: default_program,
        })
    }

    pub fn bind(&self) {
        self.frame_buffer.bind();
    }

    pub fn unbind(&self) {
        self.frame_buffer.unbind();
    }

    pub fn set_program(&mut self, program: helpers::Program) {
        self.program = program;
    }

    pub fn update_fbo_size(&mut self, width: u32, height: u32) {
        self.frame_buffer.texture.set_size(width, height);
    }

    pub fn render(&mut self) {
        self.program.set_used();
        self.frame_buffer.texture.bind_to_unit(0);
        let uniform_texture = self.program.get_uniform_location("RenderTexture")
            .expect("RenderTexture Uniform Not Found");
        self.program.set_uniform_1i(uniform_texture, 0);

        self.vao.bind();

        unsafe {
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                6
            );
        }

        self.vao.unbind();
    }
}

fn default_program() -> Result<helpers::Program, failure::Error> {
    let vert_src = r#"
        #version 330 core

        layout (location = 0) in vec2 Position;
        layout (location = 1) in vec2 UV;

        out VS_OUTPUT {
            vec2 UV;
        } OUT;

        void main() {
            gl_Position = vec4(Position.x, Position.y, 0.0, 1.0);
            OUT.UV = UV;
        }
    "#;
    let frag_src = r#"
        #version 330 core

        precision mediump float;

        uniform sampler2D RenderTexture;

        in VS_OUTPUT {
            vec2 UV;
        } IN;

        out vec4 Color;

        void main() {
            Color = texture(RenderTexture, IN.UV);
        }
    "#;

    let shaders = vec![
        helpers::Shader::from_raw(&vert_src, gl::VERTEX_SHADER)?,
        helpers::Shader::from_raw(&frag_src, gl::FRAGMENT_SHADER)?,
    ];
    let program = helpers::Program::from_shaders(&shaders[..], "internal/shaders/render_target")
        .expect("Failed to load FB RenderTarget Shader Program");

    Ok(program)
}
