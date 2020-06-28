#[macro_use] extern crate failure;

extern crate sdl2;
extern crate gl;
extern crate nalgebra_glm as glm;

use std::path::Path;

use yarge::helpers::*;
use yarge::helpers::{data};
use yarge::helpers::timer::{Timer};
use yarge::resources::Resources;
use yarge::camera::*;
use yarge::sprite::*;
use yarge::tilemaps::*;
use yarge::renderer;
use yarge::textures;
use yarge::{font, image, debug};
use yarge::font::FontRenderer;
use yarge::{Triangle};
use yarge::{Rectangle, RectangleProps};

use rusttype::{point};

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
        .window("Yarge | Font Rendering", WIDTH, HEIGHT)
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
    font_renderer.add_font("cjk".to_string(), "fonts/wqy-microhei/WenQuanYiMicroHei.ttf");

    let mut camera = Camera::new(viewport.w, viewport.h, Projection::Ortho)?;
    let mut ui_camera = Camera::new(viewport.w, viewport.h, Projection::Ortho)?;

    viewport.set_used();

    let instruction_text = font::Text::new(
        r#"
Font Rendering Demo
Use the keyboard to input new text dynamically
        "#.to_string(),
        font::TextSettings {
            font: "dejavu".to_string(),
            width: WIDTH as f32 - 80.0,
            size: 32.0.into(),
            pos: (20.0, 20.0),
            color: (255, 255, 0, 0.58),
        }
    );

    let mut d_text: String = String::from("<yarge>$: ");

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

                    camera.update_viewport(viewport.w, viewport.h);
                    ui_camera.update_viewport(viewport.w, viewport.h);
                },
                sdl2::event::Event::KeyDown { keycode, .. } => {
                    match keycode {
                        Some(sdl2::keyboard::Keycode::Backspace) => {
                            d_text.pop();
                        },
                        _ => {},
                    }
                },
                sdl2::event::Event::TextInput { text, .. } => {
                    d_text.push_str(&text);
                },
               _ => {},
            }
        }

        viewport.set_used();

        let dynamic_text = font::Text::new(
            d_text.to_string(),
            font::TextSettings {
                font: "dejavu".to_string(),
                width: WIDTH as f32 / 2.0,
                size: 18.0.into(),
                pos: (20.0, 140.0),
                color: (255, 255, 255, 1.0),
            }
        );

        let jp_text = font::Text::new(
           "フォント・レンダリング".to_string(),
           font::TextSettings {
               font: "cjk".to_string(),
               width: 1000.0,
               size: 28.0.into(),
               pos: (20.0, 100.0),
               color: (0, 150, 50, 1.0),
           }
       );

        renderer.clear();

        font_renderer.render(&instruction_text, &ui_camera);
        font_renderer.render(&jp_text, &ui_camera);
        font_renderer.render(&dynamic_text, &ui_camera);

        window.gl_swap_window();
    }

    Ok(())
}

