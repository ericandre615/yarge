extern crate sdl2;
extern crate gl;

use std::path::Path;

use yarge::helpers::*;
use yarge::resources::Resources;
use yarge::camera::{Camera, Camera3D, Projection};
use yarge::renderer;
use yarge::{font, debug};
use yarge::font::FontRenderer;
use yarge::{Rectangle, RectangleProps};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 780;

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
        .window("Yarge | Basic UI", WIDTH, HEIGHT)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    let mut event_pump = sdl.event_pump().unwrap();
    let mut viewport = Viewport::for_window(WIDTH as f32, HEIGHT as f32);

    let res = Resources::from_relative_path(Path::new("assets")).unwrap();
    let mut renderer = renderer::Renderer2D::new()?;
    let mut renderer3d = renderer::Renderer3D::new()?;


    renderer.set_clear_color(30, 30, 30, 1.0);

    let mut font_renderer = FontRenderer::new(&res, initial_dpi.0 / 100.0)?;
    font_renderer.add_font("dejavu".to_string(), "fonts/dejavu/DejaVuSansMono.ttf");
    font_renderer.add_font("cjk".to_string(), "fonts/wqy-microhei/WenQuanYiMicroHei.ttf");

    let mut ui_camera = Camera::new(viewport.w, viewport.h, Projection::Ortho)?;
    let camera_3d = Camera3D::new(viewport.w, viewport.h, 50.0, Projection::Perspective)?;

    viewport.set_used();


    let top_bar_text = font::Text::new(
        "YARGE UI".to_string(),
        font::TextSettings {
            font: "dejavu".to_string(),
            width: viewport.w as f32 / 2.0,
            size: 18.0.into(),
            pos: (20.0, 20.0),
            color: (255, 255, 255, 1.0),
        }
    );

    let left_sidebar = Rectangle::new(&RectangleProps {
        width: 280.0,
        height: 400.0,
        pos: (20.0, 100.0),
        color: (0.75, 0.85, 0.45, 0.70),
    })?;

    let left_sidebar_sub = Rectangle::new(&RectangleProps {
        width: 280.0,
        height: 200.0,
        pos: (20.0, 520.0),
        color: (0.85, 0.55, 0.25, 0.60),
    })?;

    let left_sidebar_text = font::Text::new(
        "
    Bacon ipsum dolor amet kielbasa turkey venison buffalo
    filet mignon prosciutto boudin shoulder. Shoulder ground
    round alcatra jerky, chicken pork chop fatback burgdoggen.
    Pork brisket shoulder andouille kevin hamburger.
    Beef ribs pork belly pork turducken venison short ribs.
        ".to_string(),
        font::TextSettings {
            font: "dejavu".to_string(),
            width: 240.0,
            size: 16.0.into(),
            pos: (40.0, 120.0),
            color: (100, 100, 50, 1.0),
        },
    );

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

        // TODO: several issues, have to recreate this Rect in the loop every time
        // just to get the width to change
        // Rectangle should be able to have properties updated at any point, easily
        // Also, renderer for batching can still only support Sprite, but should
        // be able to support Rect or any other Renderable item
        let top_bar = Rectangle::new(&RectangleProps {
            width: viewport.w,
            height: 60.0,
            pos: (0.0, 0.0),
            color: (0.65, 0.65, 0.65, 1.0),
        })?;
        let main_content = Rectangle::new(&RectangleProps {
            width: viewport.w - 340.0,
            height: 600.0,
            pos: (320.0, 100.0),
            color: (0.2, 0.2, 0.2, 0.80),
        })?;

        renderer.clear();
        top_bar.render(&ui_camera);
        left_sidebar.render(&ui_camera);
        left_sidebar_sub.render(&ui_camera);
        main_content.render(&ui_camera);

        font_renderer.render(&left_sidebar_text, &ui_camera);
        font_renderer.render(&top_bar_text, &ui_camera);

        window.gl_swap_window();
    }

    Ok(())
}

