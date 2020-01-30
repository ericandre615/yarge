#[macro_use] extern crate failure;
#[macro_use] extern crate gl_vertex_derive;

extern crate sdl2;
extern crate gl;
extern crate nalgebra_glm as glm;
extern crate vec_2_10_10_10;

use std::path::Path;

pub mod helpers;
pub mod resources;
pub mod texture;
mod triangle;
mod rectangle;
mod image;
mod camera;
mod debug;

use helpers::{data};
use helpers::timer::{Timer};
use resources::Resources;
use camera::*;

const WIDTH: u32 = 720;
const HEIGHT: u32 = 480;

fn main() {
    if let Err(e) = run() {
        println!("{}", debug::failure_to_string(e));
    }
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
    let mut viewport = helpers::Viewport::for_window(WIDTH as f32, HEIGHT as f32);

    unsafe { gl::ClearColor(0.3, 0.3, 0.5, 1.0); }

    let res = Resources::from_relative_path(Path::new("assets")).unwrap();
    let triangle = triangle::Triangle::new(&res)?;
    let rect1 = rectangle::Rectangle::new(&res, &rectangle::RectangleProps {
        width: 256.0,
        height: 256.0,
        pos: (20.0, 20.0),
        color: (1.0, 0.0, 0.0, 1.0),
    })?;
    let rect2 = rectangle::Rectangle::new(&res, &rectangle::RectangleProps {
        width: 256.0,
        height: 256.0,
        pos: (40.0, 40.0),
        color: (0.0, 1.0, 0.0, 0.8),
    })?;
    let rect3 = rectangle::Rectangle::new(&res, &rectangle::RectangleProps {
        width: 210.0,
        height: 210.0,
        pos: (180.0, 80.0),
        color: (0.0, 0.0, 1.0, 0.8),
    })?;

    let mut camera = Camera::new(viewport.w, viewport.h, Projection::Ortho)?;

    let mut image2 = image::Image::new(
        &res,
        image::ImageProps {
            pos: (20.0, 20.0),
            dim: (100, 100),
            img_path: "images/penguin.png".to_string(),
        }
    )?;
    let mut image3 = image::Image::new(
        &res,
        image::ImageProps {
            pos: (40.0, 40.0),
            dim: (256, 256),
            img_path: "images/ninja-gaiden.gif".to_string(),
        }
    )?;
    let mut image = image::Image::new(
        &res,
        image::ImageProps {
            pos: (180.0, 80.0),
            dim: (210, 210),//(420, 420),
            img_path: "images/mario-sprite.png".to_string(),
        }
    )?;
    let mut spritesheet = image::Image::new(
        &res,
        image::ImageProps {
            pos: (20.0, 20.0),
            dim: (256, 256),
            img_path: "images/ninja-gaiden-spritesheet.png".to_string(),
        }
    )?;

    image2.flip_v();

    image3.set_color((255, 0, 0, 1.0));

    image.set_alpha(0.5);
    let scale_ix = 210.0 / 420.0 * 4.0;
    let scale_iy = 210.0 / 420.0 * 4.0;
    image.set_texture_scale(scale_ix, scale_iy);
    image3.set_texture_scale(0.75, 0.75);
    image3.set_frame((-40, 40));
    spritesheet.set_frame((256, 0));

    let sprite_frames = [
        (0, 0),(0, 0),(0, 0),(0, 0),
        (0, 0),(0, 0),(0, 0),(0, 0),
        (256, 0),(256, 0),(256, 0),(256, 0),
        (256, 0),(256, 0),(256, 0),(256, 0),
        (512, 0),(512, 0),(512, 0),(512, 0),
        (512, 0),(512, 0),(512, 0),(512, 0),
        (256, 0),(256, 0),(256, 0),(256, 0),
        (256, 0),(256, 0),(256, 0),(256, 0),
    ];

    let mut timer = Timer::new();

    viewport.set_used();

    let mut i = 0;

    let mut is_look_at: bool = true;

    'main: loop {
        timer.tick();
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
                },
                sdl2::event::Event::KeyDown { keycode, .. } => {
                    let dt = timer.delta_time();
                    match keycode {
                        Some(sdl2::keyboard::Keycode::L) => {
//                            camera.cancel_look_at();
                            is_look_at = false;
                        },
                        Some(sdl2::keyboard::Keycode::Right) => {
                            let (x, _y) = image3.get_position();
                            image3.set_orientation(image::Direction::Normal, image::Direction::Normal);
                            image3.set_pos_x(x + 1.0 * dt);

                        },
                        Some(sdl2::keyboard::Keycode::Left) => {
                            let (x, _y) = image3.get_position();
                            image3.set_orientation(image::Direction::Flipped, image::Direction::Normal);
                            image3.set_pos_x(x - 1.0 * dt);

                        },
                        _ => break,
                    }
                },
                _ => {},
            }
        }
        //if is_look_at {
        //    let (tx, ty) = image3.get_position();
        //    camera.look_at((tx, ty, 0.0));
        //}
        //let (tx, ty) = image3.get_position();
        //camera.set_position(tx, ty, 0.0);
        // render window contents here
        unsafe {
            viewport.set_used();
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let delta_time = timer.delta_time();

        if delta_time as u32 % 2 > 0 {
            image3.set_color((255, 0, 0, 1.0));
        } else {
            image3.set_color((0, 0, 255, 1.0));
        }

        triangle.render();
        rect1.render(&camera);
        rect2.render(&camera);
        rect3.render(&camera);
        image.render(&camera, delta_time);
        image2.render(&camera, delta_time);
        spritesheet.render(&camera, delta_time);
        image3.render(&camera, delta_time);

        spritesheet.set_frame(sprite_frames[i]);

        i += 1;

        if i >= sprite_frames.len() - 1 { i = 0; }

        window.gl_swap_window();
    }

    Ok(())
}

