use crate::helpers::{self, data, buffer};
use crate::resources::*;

use crate::camera::{Camera};

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
}

pub struct RectangleProps {
    pub width: f32,
    pub height: f32,
    pub pos: (f32, f32),
    pub color: (f32, f32, f32, f32),
}

impl Default for RectangleProps {
    fn default() -> Self {
        Self {
            width: 20.0,
            height: 20.0,
            pos: (0.0, 0.0),
            color: (0.0, 0.0, 0.0, 1.0),
        }
    }
}

pub struct Rectangle {
    program: helpers::Program,
    _vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
    uniform_mvp: i32,
    uniform_color: i32,
    indicies: Vec<u32>,
    props: RectangleProps,
    model: glm::TMat4<f32>,
}

impl Rectangle {
    pub fn new(res: &Resources, props: &RectangleProps) -> Result<Rectangle, failure::Error> {
        let program = helpers::Program::from_resource(res, "shaders/rectangle")?;
        let uniform_mvp = program.get_uniform_location("MVP")?;
        let uniform_color = program.get_uniform_location("Color")?;
        let pos = props.pos;
        let (x, y) = pos;
        let width = props.width;
        let height = props.height;
        let color = props.color;
        let model = glm::translate(&glm::identity(), &glm::vec3(x, y, 0.0));
        let x2 = x + (width as f32);
        let y2 = y + (height as f32);
        let vertices: Vec<Vertex> = vec![
            // positions        // colors
           Vertex { pos: (x, y, 0.0).into() }, // bottom right
           Vertex { pos: (x2, y, 0.0).into() }, // bottom left
           Vertex { pos: (x, y2, 0.0).into() }, // top
           Vertex { pos: (x2, y2, 0.0).into() },
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

        vbo.unbind();


        vao.unbind();

        ibo.bind();
        ibo.static_draw_data(&indicies);
        ibo.unbind();

        Ok(Rectangle {
            program,
            _vbo: vbo,
            vao,
            uniform_mvp,
            uniform_color,
            indicies,
            model,
            props: RectangleProps {
                width,
                height,
                pos,
                color,
                ..Default::default()
            },
        })
    }

    pub fn get_position(&self) -> (f32, f32) {
        self.props.pos
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        let pos = glm::vec3(x, y, 0.0);
        self.props.pos = (x, y);
        self.model = glm::translate(&glm::identity(), &pos);
    }

    pub fn set_color(&mut self, color: (f32, f32, f32, f32)) {
        self.props.color = color;
    }

    pub fn render(&self, camera: &Camera) {
        let mvp = camera.get_projection() * camera.get_view() * self.model;

        self.program.set_used();
        self.program.set_uniform_4f(self.uniform_color, self.props.color);
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

