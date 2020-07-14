#[macro_use] extern crate failure;

extern crate sdl2;
extern crate gl;
extern crate nalgebra_glm as glm;

use std::path::Path;

use yarge::helpers::{Viewport, mesh};
use yarge::resources::Resources;
use yarge::camera::*;
use yarge::renderer;
use yarge::{font, debug};
use yarge::font::FontRenderer;

const WIDTH: u32 = 1024;//720;
const HEIGHT: u32 = 780;//480;

fn main() {
    if let Err(e) = run() {
        println!("{}", debug::failure_to_string(e));
    }
}

fn run() -> Result<(), failure::Error> {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    let initial_dpi = video_subsystem.display_dpi(0).unwrap(); // 0 = window/display number?

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("Yarge | Load OBJ&MTL", WIDTH, HEIGHT)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    let mut event_pump = sdl.event_pump().unwrap();
    let mut viewport = Viewport::for_window(WIDTH as f32, HEIGHT as f32);

    let res = Resources::from_relative_path(Path::new("assets")).unwrap();
    let mut renderer = renderer::Renderer2D::new(&res)?;

    renderer.set_clear_color(30, 30, 30, 1.0);

    let mut font_renderer = FontRenderer::new(&res, WIDTH, HEIGHT, initial_dpi.0 / 100.0)?;
    font_renderer.add_font("dejavu".to_string(), "fonts/dejavu/DejaVuSansMono.ttf");

    let cube_obj_contents = res.load_obj("meshes/cube.obj")?;
    let simple_cube_mesh = mesh::Mesh::from_file(&res, "meshes/simple-cube.obj")?;
    let cube_mesh = mesh::Mesh::from_file(&res, "meshes/cube.obj")?;

    println!("MESH {:#?}", cube_mesh);

    let mut ui_camera = Camera::new(viewport.w, viewport.h, Projection::Ortho)?;

    viewport.set_used();

    let instruction_text = font::Text::new(
        r#"
Load OBJ and MTL files contents
        "#.to_string(),
        font::TextSettings {
            font: "dejavu".to_string(),
            width: WIDTH as f32 - 40.0,
            size: 32.0.into(),
            pos: (20.0, 20.0),
            color: (255, 255, 0, 0.58),
        }
    );

    let cube_debug = font::Text::new(
        cube_obj_contents,
        font::TextSettings {
            font: "dejavu".to_string(),
            width: WIDTH as f32 - 40.0,
            size: 24.0.into(),
            pos: (20.0, 100.0),
            color: (255, 255, 0, 1.0),
        }
    );

    let simple_cube_verts = simple_cube_mesh.get_vertices();

    println!("SIMPLECUBEVERTS {:#?}", simple_cube_verts);

    let cube_verts = cube_mesh.get_vertices();

    println!("CUBEVERTS {:#?}", cube_verts);

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    viewport.update_size(w as f32, h as f32);
                    viewport.set_used();

                    ui_camera.update_viewport(viewport.w, viewport.h);
                },
               _ => {},
            }
        }

        renderer.clear();

        font_renderer.render(&instruction_text, &ui_camera);
        font_renderer.render(&cube_debug, &ui_camera);

        window.gl_swap_window();
    }

    Ok(())
}

