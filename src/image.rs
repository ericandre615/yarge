use crate::helpers::{self, data, buffer};
use crate::resources::*;

const IMAGE_BASE_COLOR: (f32, f32, f32, f32) = (1.0, 0.0, 0.5, 1.0);

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    clr: data::u2_u10_u10_u10_rev_float,
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
    image: ImageProps,
}

impl Image {
    pub fn new(res: &Resources, image: ImageProps) -> Result<Image, failure::Error> {
        let program = helpers::Program::from_resource(res, "shaders/image")?;
        let uniform_viewport_resolution_location = program.get_uniform_location("ViewportResolution")?;
        let (x, y) = image.pos;
        let (width, height) = image.dim;
        let x2 = x + (width as f32);
        let y2 = y + (height as f32);
        let vertices: Vec<Vertex> = vec![
            // positions        // colors
           Vertex { pos: (x, y, 0.0).into(), clr: IMAGE_BASE_COLOR.into() }, // bottom right
           Vertex { pos: (x2, y, 0.0).into(), clr: IMAGE_BASE_COLOR.into() }, // bottom left
           Vertex { pos: (x, y2, 0.0).into(), clr: IMAGE_BASE_COLOR.into() }, // top
           // second triangle
           Vertex { pos: (x, y2, 0.0).into(), clr: IMAGE_BASE_COLOR.into() },
           Vertex { pos: (x2, y, 0.0).into(), clr: IMAGE_BASE_COLOR.into() },
           Vertex { pos: (x2, y2, 0.0).into(), clr: IMAGE_BASE_COLOR.into() }
        ]; // 2 triangles makes a rectangle
        //let vertices: Vec<Vertex> = vec![
        //    // positions        // colors
        //   Vertex { pos: (10.0, 20.0, 0.0).into(), clr: IMAGE_BASE_COLOR.into() }, // bottom right
        //   Vertex { pos: (80.0, 20.0, 0.0).into(), clr: IMAGE_BASE_COLOR.into() }, // bottom left
        //   Vertex { pos: (10.0, 30.0, 0.0).into(), clr: IMAGE_BASE_COLOR.into() }, // top
        //   // second triangle
        //   Vertex { pos: (10.0, 30.0, 0.0).into(), clr: IMAGE_BASE_COLOR.into() },
        //   Vertex { pos: (80.0, 20.0, 0.0).into(), clr: IMAGE_BASE_COLOR.into() },
        //   Vertex { pos: (80.0, 30.0, 0.0).into(), clr: IMAGE_BASE_COLOR.into() }
        //]; // 2 triangles makes a rectangle

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

        Ok(Image {
            program,
            _vbo: vbo,
            vao,
            image,
            uniform_viewport_resolution_location,
        })
    }

    pub fn render(&self, viewport: &helpers::Viewport) {
        let viewport_dimensions = nalgebra::Vector2::new(viewport.w as f32, viewport.h as f32);
        self.program.set_used();
        self.program.set_uniform_2f(self.uniform_viewport_resolution_location, &viewport_dimensions);
        self.vao.bind();

        unsafe {
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                6 // 3 per triangle
            );
        }
    }
}

