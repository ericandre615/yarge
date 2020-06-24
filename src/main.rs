#[macro_use] extern crate failure;
#[macro_use] extern crate gl_vertex_derive;

extern crate sdl2;
extern crate gl;
extern crate nalgebra_glm as glm;
extern crate vec_2_10_10_10;

use std::path::Path;

pub mod helpers;
pub mod resources;
pub mod textures;
pub mod triangle;
pub mod rectangle;
pub mod image;
pub mod camera;
pub mod renderer;
pub mod debug;
pub mod sprite;
pub mod tilemaps;
pub mod font;

use helpers::{data};
use helpers::timer::{Timer};
use resources::Resources;
use camera::*;
use sprite::*;
use tilemaps::*;
use font::{FontRenderer};

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
        .window("Yarge", WIDTH, HEIGHT)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    let mut event_pump = sdl.event_pump().unwrap();
    let mut viewport = helpers::Viewport::for_window(WIDTH as f32, HEIGHT as f32);

    let res = Resources::from_relative_path(Path::new("assets")).unwrap();
    let mut texture_manager = textures::TextureManager::new(&res);
    let mut renderer = renderer::Renderer2D::new(&res)?;

    let mario_texture = textures::texture::Texture::new(&res, "images/mario-sprite.png".to_string())?;
    let test_texture = textures::texture::Texture::new(&res, "images/test.png".to_string())?;
    let spritesheet_texture = textures::texture::Texture::new(&res, "images/ninja-gaiden-spritesheet.png".to_string())?;
    texture_manager.create("ninja", "images/ninja-gaiden.gif");
    //texture_manager.create("test", "images/test.png");
    texture_manager.create("test_b", "images/test_b.png");
    texture_manager.add("mario", mario_texture);
    texture_manager.add("test", test_texture);
    texture_manager.add("ninja_spritesheet", spritesheet_texture);
    //let ninja_texture = texture_manager.get(ninja_t);

    renderer.set_clear_color(30, 30, 30, 1.0);

    let mut font_renderer = FontRenderer::new(&res, WIDTH, HEIGHT, initial_dpi.0 / 100.0)?;
    font_renderer.add_font("dejavu".to_string(), "fonts/dejavu/DejaVuSansMono.ttf");
    font_renderer.add_font("cjk".to_string(), "fonts/wqy-microhei/WenQuanYiMicroHei.ttf");

    println!("TEXT_RENDER: {:#?}", font_renderer);
    println!("TEXT_FONT: {:#?}", font_renderer.fonts.get("dejavu").unwrap());
    println!("TEXT_FONT_COUNT: {:#?}", font_renderer.fonts.get("dejavu").unwrap().glyph_count());

    // TODO: remove set_ppe_program to get normal, this is a basic post-process example effect with
    // a very primitively implemented light
    //let lighting_program = helpers::Program::from_resource(&res, "shaders/basic-light")?;
    //renderer.set_ppe_program(&lighting_program);

    //lighting_program.set_used();
    //let uniform_intensity = lighting_program.get_uniform_location("Intensity")?;
    //let uniform_lightpos = lighting_program.get_uniform_location("LightPosition")?;
    //lighting_program.set_uniform_1f(uniform_intensity, 0.45);
    //lighting_program.set_uniform_2f(uniform_lightpos, &glm::vec2(0.1, 0.1));

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
    let mut ui_camera = Camera::new(viewport.w, viewport.h, Projection::Ortho)?;

    let mut image2 = image::Image::new(
        &res,
        image::ImageProps {
            pos: (20.0, 20.0),
            dim: (100, 100),
            img_path: "images/penguin.png".to_string(),
            ..Default::default()
        }
    )?;
    let mut image3 = image::Image::new(
        &res,
        image::ImageProps {
            pos: (40.0, 40.0),
            dim: (256, 256),
            img_path: "images/ninja-gaiden.gif".to_string(),
            texture_slot: 2,
        }
    )?;
    let mut image = image::Image::new(
        &res,
        image::ImageProps {
            pos: (180.0, 80.0),
            dim: (210, 210),//(420, 420),
            img_path: "images/mario-sprite.png".to_string(),
            ..Default::default()
        }
    )?;
    let mut spritesheet = image::Image::new(
        &res,
        image::ImageProps {
            pos: (20.0, 20.0),
            dim: (256, 256),
            img_path: "images/ninja-gaiden-spritesheet.png".to_string(),
            texture_slot: 30,
        }
    )?;

    let mut spritesheet_as_sprite = Sprite::from_texture(
        texture_manager.get("ninja_spritesheet"),
        SpriteProps {
            pos: (220.0, 200.0, 0.0),
            dim: (256, 256),
            ..Default::default()
        }
    )?;

    let mut some_sprite = Sprite::from_texture(
        texture_manager.get("test"),
        SpriteProps {
            pos: (100.0, 20.0, 0.0),
            dim: (240, 240),
            color: (255, 255, 255, 1.0),//(20, 30, 80, 0.5),
            texture_slot: 12,
        },
    )?;

    let mut some_other_sprite = Sprite::from_texture(
        texture_manager.get("test_b"),
        SpriteProps {
            pos: (20.0, 280.0, 0.0),
            dim: (240, 240),
            color: (255, 255, 255, 1.0),//(20, 30, 80, 0.5),
            texture_slot: 14,
        },
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
    spritesheet_as_sprite.set_frame((246.0, 0.0));

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

    let mut vbs = Vec::new();
    let start_pos = (300.0, 300.0);
    for i in 0..4 {
        let pos = (
            start_pos.0 + 200.0 + (i as f32 * 40.0),//+ 40.0 + (i * 20) as f32 + 200.0,
            start_pos.1 - 100.0 - (i as f32 * 40.0),//- 20.0 - (i * 10) as f32 - 100.0,
            0.0
        );
        let alpha = match i {
            3 => 1.0,
            _ => (90.0 - (i as f32 * 2.0)) / 255.0,
        };
        let mut batch_sprite = Sprite::from_texture(
            texture_manager.get("ninja"),
            SpriteProps {
                pos: pos,
                dim: (240, 240),
                color: (255, 255, 255, alpha),//(20, 30, 80, 0.5),
                texture_slot: 4,
            },
        )?;
        vbs.push(batch_sprite);
    }
    let mut mario_as_sprite = Sprite::from_texture(
        texture_manager.get("mario"),
        SpriteProps {
            pos: (10.0, 10.0, 0.0),
            dim: (210, 210),
            color: (255, 255, 255, 0.75),
            texture_slot: 8,
        },
    )?;

    mario_as_sprite.set_texture_scale((scale_ix, scale_iy));
    //mario_as_sprite.set_frame((0.0, 210.0));

    let mut ninja_as_sprite = Sprite::from_texture(
        texture_manager.get("ninja"),
        SpriteProps {
            pos: (400.0, 40., 0.0),
            dim: (256, 256),
            color: (255, 255, 255, 1.0),
            texture_slot: 7
        },
    )?;

    let tilemap = Tilemap::from_json(&res, "tilemaps/tilemap_test.json".to_string())?;

    let my_text = font::Text::new(
        "Hello OpenGL".to_string(),
        font::TextSettings {
            font: "dejavu".to_string(),
            width: 200.0,
            size: 62.0.into(),
            pos: (0.0, 0.0),
            color: (255, 255, 0, 0.58),
        }
    );
    let my_text_b = font::Text::new(
        "Retro Style Games".to_string(),
        font::TextSettings {
            font: "dejavu".to_string(),
            width: 600.0,
            size: 24.0.into(),
            pos: (40.0, 40.0),
            color: (100, 100, 100, 1.0),
        }
    );


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
                    ui_camera.update_viewport(viewport.w, viewport.h);
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
                        Some(sdl2::keyboard::Keycode::A) => {
                            let pos = camera.get_position();
                            camera.set_pos_x(pos.x - 5.0 * dt);
                        },
                        Some(sdl2::keyboard::Keycode::D) => {
                            let pos = camera.get_position();
                            camera.set_pos_x(pos.x + 5.0 * dt);
                        },
                        Some(sdl2::keyboard::Keycode::W) => {
                            let pos = camera.get_position();
                            camera.set_pos_y(pos.y - 5.0 * dt);
                        },
                        Some(sdl2::keyboard::Keycode::S) => {
                            let pos = camera.get_position();
                            camera.set_pos_y(pos.y + 5.0 * dt);
                        },
                        Some(sdl2::keyboard::Keycode::R) => {
                            camera.set_position(0.0, 0.0, 0.0);
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
        // TODO: can not get look_at to work????
        let (ipx, ipy) = image3.get_position();
        //camera.look_at((ipx, ipy, 0.0));
        viewport.set_used();

        triangle.render(); // not rendered because renderer.render calls clear... is that clear though?
        renderer.clear();

        let delta_time = timer.delta_time();

        if delta_time as u32 % 2 > 0 {
            image3.set_color((255, 0, 0, 1.0));
        } else {
            image3.set_color((0, 0, 255, 1.0));
        }

        rect1.render(&camera);
        rect2.render(&camera);
        rect3.render(&camera);
        image.render(&camera, delta_time);
        image2.render(&camera, delta_time);
        spritesheet.render(&camera, delta_time);
        image3.render(&camera, delta_time);

        spritesheet.set_frame(sprite_frames[i]);

        spritesheet_as_sprite.set_frame((sprite_frames[i].0 as f32, sprite_frames[i].1 as f32));

        i += 1;

        if i >= sprite_frames.len() - 1 { i = 0; }

        renderer.begin_scene(&camera);
        renderer.begin_batch();

        for s in &vbs {
            renderer.submit(s);
        }

        renderer.submit(&some_sprite);
        renderer.submit(&some_other_sprite);
        renderer.submit(&mario_as_sprite);
        renderer.submit(&ninja_as_sprite);
        renderer.submit(&spritesheet_as_sprite);

        for ts in tilemap.get_vertices() {
            renderer.submit(&ts);
        }

        renderer.end_batch();
        renderer.render(&camera);
        renderer.end_scene();

       let jp_text = font::Text::new(
           "こんにちは　世界".to_string(),
           font::TextSettings {
               font: "cjk".to_string(),
               width: 1000.0,
               size: 72.0.into(),
               pos: (200.0, 80.0),
               color: (0, 150, 50, 1.0),
           }
       );

        font_renderer.render(&my_text, &ui_camera);
        font_renderer.render(&my_text_b, &ui_camera);
        font_renderer.render(&jp_text, &ui_camera);

        window.gl_swap_window();
    }

    Ok(())
}

