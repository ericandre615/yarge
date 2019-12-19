#[macro_use] extern crate failure;
#[macro_use] extern crate gl_vertex_derive;

extern crate sdl2;
extern crate gl;
extern crate vec_2_10_10_10;

use std::path::Path;

pub mod helpers;
pub mod resources;
mod triangle;
mod debug;

use helpers::data;
use resources::Resources;

const WIDTH: u32 = 720;
const HEIGHT: u32 = 480;

// http://nercury.github.io/rust/opengl/tutorial/2018/02/15/opengl-in-rust-from-scratch-08-failure.html

fn main() {
    if let Err(e) = run() {
        println!("{}", debug::failure_to_string(e));
    }
}

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    clr: data::u2_u10_u10_u10_rev_float,
}

fn run() -> Result<(), failure::Error> {
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
    let triangle = triangle::Triangle::new(&res)?;

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

        triangle.render();

        window.gl_swap_window();
    }

    Ok(())
}

