extern crate sdl2;
extern crate gl;
extern crate nalgebra_glm as glm;

use std::path::Path;

use yarge::helpers::*;
use yarge::helpers::timer::{Timer};
use yarge::resources::Resources;
use yarge::camera::*;
use yarge::sprite::*;
use yarge::renderer;
use yarge::textures;
use yarge::{debug};
use yarge::sprite::animation::{Animation, Animations};

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

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("Yarge | Animation", WIDTH, HEIGHT)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    let mut event_pump = sdl.event_pump().unwrap();
    let mut viewport = Viewport::for_window(WIDTH as f32, HEIGHT as f32);

    let res = Resources::from_relative_path(Path::new("assets")).unwrap();
    let mut texture_manager = textures::TextureManager::new(&res);
    let mut renderer = renderer::Renderer2D::new()?;

    let ninja_spritesheet_texture = textures::texture::Texture::new(&res, "images/ninja-gaiden-spritesheet.png".to_string())?;
    texture_manager.add("ninja_spritesheet", ninja_spritesheet_texture);

    renderer.set_clear_color(10, 10, 10, 1.0);

    let mut camera = Camera::new(viewport.w, viewport.h, Projection::Ortho)?;

    let mut spritesheet_as_sprite = Sprite::from_texture(
        texture_manager.get("ninja_spritesheet"),
        SpriteProps {
            pos: (220.0, 20.0, 0.0),
            dim: (256, 256),
            ..Default::default()
        }
    )?;

    let mut ninja_sprite = Sprite::from_texture(
        texture_manager.get("ninja_spritesheet"),
        SpriteProps {
            pos: (220.0, 300.0, 0.0),
            dim: (256, 256),
            ..Default::default()
        }
    )?;

    spritesheet_as_sprite.set_frame((246.0, 0.0));

    let sprite_frames = vec![
        (0.0, 0.0),
        (256.0, 0.0),
        (512.0, 0.0),
        (256.0, 0.0),
        (0.0, 0.0),
    ];

    let walk_animation = Animation::new("walk".to_string(), sprite_frames.clone());

    let walk_anim_from_json = Animation::from_json(&res, "animations/single.json".to_string())?;
    let anims_from_json = Animations::from_json(&res, "animations/multi.json".to_string())?;

    let mut animations_manager = Animations::new(Vec::new());
    let mut animations_manager_single_json = Animations:: new(vec![walk_anim_from_json]);
    animations_manager.add(walk_animation);

    animations_manager.play("walk");
    animations_manager.set_framerate(80.0);


    animations_manager_single_json.play("walk_single");
    animations_manager_single_json.set_framerate(60.0);

    let mut timer = Timer::new();

    viewport.set_used();

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
                    let (sprite_x, sprite_y, sprite_z) = spritesheet_as_sprite.props.pos;

                    match keycode {
                        Some(sdl2::keyboard::Keycode::Right) => {
                            let x = sprite_x + 2.0 * dt;
                            spritesheet_as_sprite.set_position((x, sprite_y, sprite_z));
                        },
                        Some(sdl2::keyboard::Keycode::Left) => {
                            let x = sprite_x - 2.0 * dt;
                            spritesheet_as_sprite.set_position((x, sprite_y, sprite_z));
                        },
                        Some(sdl2::keyboard::Keycode::P) => {
                            if animations_manager.is_playing() {
                                animations_manager.pause();
                            } else {
                                animations_manager.resume();
                            }
                        },
                        _ => break,
                    }
                },
                _ => {},
            }
        }

        let dt = timer.delta_time();

        animations_manager.update(dt);
        animations_manager_single_json.update(dt);

        let sprite_frame = animations_manager.get_frame();
        let single_sprite_frame = animations_manager_single_json.get_frame();

        renderer.clear();

        spritesheet_as_sprite.set_frame(sprite_frame);
        ninja_sprite.set_frame(single_sprite_frame);

        renderer.begin_scene(&camera);
        renderer.begin_batch();

        renderer.submit(&spritesheet_as_sprite);
        renderer.submit(&ninja_sprite);

        renderer.end_batch();
        renderer.render(&camera);
        renderer.end_scene();

        window.gl_swap_window();
    }

    Ok(())
}

