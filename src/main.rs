extern crate sdl2;
extern crate gl;

use std::ffi::CString;
use std::path::Path;

pub mod helpers;
pub mod resources;

use resources::Resources;

const WIDTH: u32 = 720;
const HEIGHT: u32 = 480;

// http://nercury.github.io/rust/opengl/tutorial/2018/02/12/opengl-in-rust-from-scratch-06-gl-generator.html

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("Demo", WIDTH, HEIGHT)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    let mut event_pump = sdl.event_pump().unwrap();

    unsafe { gl::ClearColor(0.3, 0.3, 0.5, 1.0); }

    let res = Resources::from_relative_path(Path::new("assets")).unwrap();

    //let vert_shader = helpers::Shader::from_vertex_source(
    //    &CString::new(include_str!("triangle.vertex")).unwrap()
    //).unwrap();

    //let frag_shader = helpers::Shader::from_fragment_source(
    //    &CString::new(include_str!("triangle.fragment")).unwrap()
    //).unwrap();

    //let shader_program = helpers::Program::from_shaders(
    //    &[vert_shader, frag_shader]
    //).unwrap();

    let shader_program = helpers::Program::from_resource(
        &res, "shaders/triangle"
    ).unwrap();

    shader_program.set_used();

    let vertices: Vec<f32> = vec![
        // positions        // colors
        -0.5, -0.5, 0.0,    1.0, 0.0, 0.0,
        0.5, -0.5, 0.0,     0.0, 1.0, 0.0,
        0.0, 0.5, 0.0,      0.0, 0.0, 1.0
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, //usage
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0); //unbind the buffer
    }

    let mut vao: gl::types::GLuint = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null()
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind buffer
        gl::BindVertexArray(0);
    }

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        // render window contents here
        unsafe {
            gl::Viewport(0, 0, WIDTH as i32, HEIGHT as i32);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                3,
            );
        }

        window.gl_swap_window();
    }
}
