extern crate sdl2;
extern crate gl;

use std::path::Path;

use yarge::helpers::{Viewport, timer::Timer};
use yarge::helpers::mesh;
use yarge::resources::Resources;
use yarge::camera::{Camera, Camera3D, Projection};
use yarge::renderer;
use yarge::{font, debug};
use yarge::font::FontRenderer;
use yarge::{Rectangle, RectangleProps};
use yarge::textures;

const WIDTH: u32 = 1024 * 2; // high DPI bro X 2 that shitzz
const HEIGHT: u32 = 780 * 2;

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
        .window("Yarge | ThreeDee", WIDTH, HEIGHT)
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
    let mut timer = Timer::new();
    let mut texture_manager = textures::TextureManager::new(&res);

    texture_manager.create("mario", "images/mario-sprite.png")?;

    renderer.set_clear_color(30, 30, 30, 1.0);
    renderer3d.set_clear_color(30, 30, 30, 1.0);

    let mut font_renderer = FontRenderer::new(&res, initial_dpi.0 / 100.0)?;
    font_renderer.add_font("dejavu".to_string(), "fonts/dejavu/DejaVuSansMono.ttf");
    font_renderer.add_font("cjk".to_string(), "fonts/wqy-microhei/WenQuanYiMicroHei.ttf");

    let mut ui_camera = Camera::new(viewport.w, viewport.h, Projection::Ortho)?;
    let mut camera3d = Camera3D::new(viewport.w, viewport.h, 1.5708, Projection::Perspective)?;

    let katana_obj_data = res.load_obj("meshes/katana/my_first_katana_toon.obj")?;
    let colored_cube_data = res.load_obj("meshes/colored_cube/colored_cube.obj")?;
    let default_cube_data = res.load_obj("meshes/default_cube/default_cube.obj")?;

    //let katana_mesh = mesh::Mesh::from_file(&res, "meshes/katana/my_first_katana_toon.obj")?;
    //let katana_mesh = mesh::Mesh::from_file(&res, "meshes/katana_joined/katana_applied.obj")?;
    let katana_mesh = mesh::Mesh::from_file(&res, "meshes/katana_broken_simple/katana_blade_only.obj")?;

    let default_cube = mesh::Mesh::from_file(&res, "meshes/default_cube/default_cube.obj")?;
    let colored_cube = mesh::Mesh::from_file(&res, "meshes/colored_cube/colored_cube.obj")?;
    let default_sphere = mesh::Mesh::from_file(&res, "meshes/default_uv_sphere/default_sphere.obj")?;
    let multi_cube = mesh::Mesh::from_file(&res, "meshes/multi_cube/three_cube.obj")?;
    //println!("KATANA MESH {:#?}", katana_mesh);

    println!("Cube Data {:#?}", colored_cube_data);
    println!("Cube Mesh {:#?}", colored_cube);

    viewport.set_used();

    let katana_text = font::Text::new(
        "刀・カタナ".to_string(),
        font::TextSettings {
            font: "cjk".to_string(),
            width: viewport.w as f32,
            size: 18.0.into(),
            pos: (0.0, 0.0),
            color: (255, 0, 140, 1.0),
        },
    );

    let title = font::Text::new(
        "3D Katana Toon Shade Demo".to_string(),
        font::TextSettings {
            font: "dejavu".to_string(),
            width: viewport.w as f32,
            size: 18.0.into(),
            pos: (0.0, 36.0),
            color: (255, 0, 140, 1.0),
        },
    );

    let katana_debug_text = font::Text::new(
        katana_obj_data,
        font::TextSettings {
            font: "dejavu".to_string(),
            width: viewport.w as f32,
            size: 18.0.into(),
            pos: (20.0, 20.0),
            color: (255, 255, 255, 1.0),
        }
    );

    let top_bar = Rectangle::new(&RectangleProps {
        width: viewport.w,
        height: 140.0,
        pos: (0.0, 0.0),
        color: (0.65, 0.65, 0.65, 1.0),
    })?;

    let mut mouse_move = false;

    camera3d.set_pos_z(-90.0);

    'main: loop {
        timer.tick();
        let mouse_state = event_pump.mouse_state();
        let (mouse_x, mouse_y) = (mouse_state.x(), mouse_state.y());

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
                    camera3d.update_viewport(viewport.w, viewport.h);
                },
                sdl2::event::Event::KeyDown { keycode, .. } => {
                    let dt = timer.delta_time();
                    let cam_pos = camera3d.get_position();
                    let cx = cam_pos.x;
                    let cy = cam_pos.y;
                    let cz = cam_pos.z;
                    let cvel = 5.0;
                    let fov = camera3d.get_fov();
                    match keycode {
                        Some(sdl2::keyboard::Keycode::Z) => {
                            camera3d.update_fov(fov + 0.25);
                        },
                        Some(sdl2::keyboard::Keycode::X) => {
                            camera3d.update_fov(fov - 0.25);
                        },
                        Some(sdl2::keyboard::Keycode::C) => {
                            camera3d.update_fov(43.0); // why does 43 strangly sort of work?
                        },
                        Some(sdl2::keyboard::Keycode::V) => {
                            camera3d.update_fov(1.5708);
                        },
                        Some(sdl2::keyboard::Keycode::R) => {
                            camera3d.set_position(0.0, 0.0, 0.0);
                        },
                        Some(sdl2::keyboard::Keycode::L) => {
                            camera3d.look_at((0.0, 0.0, 0.0));
                        },
                        Some(sdl2::keyboard::Keycode::K) => {
                            camera3d.cancel_look_at();
                        },
                        // Camera Pan
                        Some(sdl2::keyboard::Keycode::W) => {
                            camera3d.set_pos_y(cy - cvel * dt);
                        },
                        Some(sdl2::keyboard::Keycode::S) => {
                            camera3d.set_pos_y(cy + cvel * dt);
                        },
                        Some(sdl2::keyboard::Keycode::A) => {
                            camera3d.set_pos_x(cx - cvel * dt);
                        },
                        Some(sdl2::keyboard::Keycode::D) => {
                            camera3d.set_pos_x(cx + cvel * dt);
                        },
                        // Camera Zoom
                        Some(sdl2::keyboard::Keycode::Q) => {
                            camera3d.set_pos_z(cz + cvel * dt);
                        },
                        Some(sdl2::keyboard::Keycode::E) => {
                            camera3d.set_pos_z(cz - cvel * dt);
                        },
                        Some(sdl2::keyboard::Keycode::M) => {
                            mouse_move = true;
                        },
                        _ => break,
                    }
                },
                sdl2::event::Event::KeyUp { keycode, .. } => {
                    match keycode {
                        Some(sdl2::keyboard::Keycode::M) => {
                            mouse_move = false;
                        },
                        _ => break,
                    }
                },
               _ => {},
            }
        }

        if mouse_move {
            let cam_pos = camera3d.get_position();
            let x = (cam_pos.x / mouse_x as f32) + mouse_x as f32;
            let y = (cam_pos.y / mouse_y as f32) + mouse_y as f32;
            camera3d.set_position(x, y, cam_pos.z);
        }


        let fov_debug_text = font::Text::new(
            format!("FOV: {:?}", camera3d.get_fov()),
            font::TextSettings {
                font: "dejavu".to_string(),
                width: 400.0,
                size: 18.0.into(),
                pos: (viewport.h as f32 - 100.0, 0.0),
                color: (255, 255, 255, 1.0),
            }
        );

        let cam_debug_text = font::Text::new(
            format!("Cam {:?}", camera3d.get_position()),
            font::TextSettings {
                font: "dejavu".to_string(),
                width: 400.0,
                size: 16.0.into(),
                pos: (viewport.h as f32 - (100.0 + 400.0), 0.0),
                color: (255, 0, 255, 1.0),
            },
        );

        let mouse_debug_text = font::Text::new(
            format!("Mouse x: {:?}, y: {:?}", mouse_x, mouse_y),
            font::TextSettings {
                font: "dejavu".to_string(),
                width: 200.0,
                size: 14.0.into(),
                pos: (viewport.h as f32 - (100.0 + 400.0 + 200.0), 0.0),
                color: (0, 255, 255, 1.0),
            }
        );

        renderer3d.clear();

        renderer3d.begin_scene(&camera3d);
        renderer3d.begin_batch();

        renderer3d.submit(&katana_mesh);
        //renderer3d.submit(&default_sphere);
        //renderer3d.submit(&default_cube);
        //renderer3d.submit(&multi_cube);

        //renderer3d.submit(&colored_cube);

        renderer3d.end_batch();
        renderer3d.render(&camera3d);
        renderer3d.end_scene();

        top_bar.render(&ui_camera);

        font_renderer.render(&katana_text, &ui_camera);
        font_renderer.render(&title, &ui_camera);
        font_renderer.render(&fov_debug_text, &ui_camera);
        font_renderer.render(&cam_debug_text, &ui_camera);
        font_renderer.render(&mouse_debug_text, &ui_camera);
        //font_renderer.render(&katana_debug_text, &ui_camera);

        window.gl_swap_window();
    }

    Ok(())
}

