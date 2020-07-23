pub mod text;
mod font_texture;
mod font_shaders;
mod layout;

use rusttype::gpu_cache::Cache;
use rusttype::{point, vector, Font, Rect, Scale};
use std::fmt;
use std::collections::HashMap;

use crate::helpers::{self, data, buffer};
use crate::resources::{Resources};
use crate::camera::{Camera};

use font_shaders::{VERTEX_SOURCE, FRAGMENT_SOURCE};
use layout::{basic_layout};
use font_texture::{FontTexture, GlyphTexture};
pub use text::{Text, TextSettings};

#[derive(VertexAttribPointers)]
#[derive(Debug)]
#[repr(C, packed)]
pub struct GlyphVertex {
    #[location=0]
    pos: data::f32_f32_f32,
    #[location=1]
    tex: data::f32_f32,
    #[location=2]
    color: data::f32_f32_f32_f32,
}

pub type GlyphVertices = Vec<GlyphVertex>;

pub struct FontRenderer<'a> {
    res: &'a Resources,
    pub fonts: HashMap<String, Font<'a>>,
    pub cache: Cache<'a>,
    scale_factor: f32,
    program: helpers::Program,
    vbo: buffer::DynamicArrayBuffer,
    vao: buffer::VertexArray,
    ibo: buffer::ElementArrayBuffer,
    texture: FontTexture,
    vertices: Vec<GlyphVertices>,
    indices: Vec<[i32; 6]>,
    uniforms: HashMap<String, i32>,
}

impl fmt::Debug for FontRenderer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TextRenderer")
            .field("res", &self.res)
            .field("fonts", &self.fonts)
            .field("cache", &"FontCache".to_string())
            .field("scale_factor", &self.scale_factor)
            .finish()
    }
}

impl<'a> FontRenderer<'a> {
    pub fn new(res: &'a Resources, display_dpi: f32) -> Result<FontRenderer, failure::Error> {
        let scale_factor = display_dpi;
        // TODO: research Signed Distance Field
        let (cache_width, cache_height) = (
            (512.0 * scale_factor) as u32,
            (512.0 * scale_factor) as u32,
        );
        let cache = Cache::builder()
            .dimensions(cache_width, cache_height)
            .build();
        let shaders = vec![
            helpers::Shader::from_raw(&VERTEX_SOURCE, gl::VERTEX_SHADER)?,
            helpers::Shader::from_raw(&FRAGMENT_SOURCE, gl::FRAGMENT_SHADER)?,
        ];
        let program = helpers::Program::from_shaders(&shaders[..], "internal/shaders/font")
            .expect("Failed to create Font Shader Program");
        let uniform_texture = program.get_uniform_location("GlyphTexture")?;
        let uniform_mvp = program.get_uniform_location("MVP")?;
        let max_buffer_size = ((::std::mem::size_of::<GlyphVertex>()) * 4000) as gl::types::GLsizeiptr;
        let max_glyphs = 1000;

        let vbo = buffer::DynamicArrayBuffer::new(max_buffer_size);
        let vao = buffer::VertexArray::new();
        let ibo = buffer::ElementArrayBuffer::new();

        let indices = generate_batch_indices(max_glyphs);

        vbo.bind();
        vbo.set_buffer_data();

        vao.bind();

        GlyphVertex::vertex_attrib_pointers();

        ibo.bind();
        ibo.static_draw_data(&indices);
        ibo.unbind();

        vbo.unbind();
        vao.unbind();

        Ok(FontRenderer {
            res,
            scale_factor: scale_factor as f32,
            cache,
            program,
            fonts: HashMap::new(),
            vbo,
            vao,
            ibo,
            vertices: Vec::new(),
            indices,
            texture: FontTexture::new(cache_width, cache_height),
            uniforms: vec![
                ("texture".to_owned(), uniform_texture),
                ("mvp".to_owned(), uniform_mvp),
            ].into_iter().collect(),
        })
    }

    pub fn add_font(&mut self, font_name: String, font_path: &str) {
        let font = self.res.load_font(font_path).unwrap();

        self.fonts.insert(font_name, font);
    }

    pub fn get_font(&self, font_name: &str) -> &Font {
        self.fonts.get(font_name).as_ref().unwrap()
    }

    pub fn update_cache_size(&mut self, screen_width: u32, screen_height: u32) {
        let (cache_width, cache_height) = (
            (screen_width * self.scale_factor as u32),
            (screen_height * self.scale_factor as u32)
        );

        self.cache = Cache::builder()
            .dimensions(cache_width, cache_height)
            .build();
    }

    pub fn set_scale_factor(&mut self, display_dpi: f32) {
        self.scale_factor = display_dpi;
    }

    pub fn render(&mut self, text: &Text, camera: &Camera) {
        let font = self.fonts.get(&text.settings.font)
            .unwrap_or_else(|| panic!("No Font {:#?} Found", text.settings.font));
        let text_color = (
            text.settings.color.0 as f32 / 255.0,
            text.settings.color.1 as f32 / 255.0,
            text.settings.color.2 as f32 / 255.0,
            text.settings.color.3
        );

        let font_scale = text.settings.size.scale.x * self.scale_factor;

        // TODO: support user layout with function? Probably and default to this basic_layout
        let glyphs = basic_layout(
            font,
            Scale::uniform(font_scale),
            text.settings.width as u32,
            &text.text
        );

        for glyph in &glyphs {
            self.cache.queue_glyph(0/* font_id */, glyph.clone());
        }

        let font_texture = &self.texture;
        self.cache.cache_queued(|rect, data| {
            let glyph_texture = GlyphTexture {
                left: rect.min.x,
                bottom: rect.min.y,
                width: rect.width(),
                height: rect.height(),
                data,
            };

            font_texture.update(&glyph_texture);
        }).unwrap();

        let origin = point(0.0, 0.0);
        let (text_offset_x, text_offset_y) = text.settings.pos;
        let vertices: Vec<Vec<GlyphVertex>> = glyphs
            .iter()
            .filter_map(|g| self.cache.rect_for(0 /* font_id */, g).ok().unwrap())
            .map(|(uv_rect, screen_rect)| {
                let gl_rect = Rect {
                    min: origin + vector(
                        screen_rect.min.x as f32 + text_offset_x as f32,
                        screen_rect.min.y as f32 + text_offset_y as f32
                    ),
                    max: origin + vector(
                        screen_rect.max.x as f32 + text_offset_x as f32,
                        screen_rect.max.y as f32 + text_offset_y as f32
                    ),
                };

                vec![
                    // top left
                    GlyphVertex {
                        pos: (gl_rect.min.x, gl_rect.min.y, 0.0).into(),
                        tex: (uv_rect.min.x, uv_rect.min.y).into(),
                        color: text_color.into(),
                    },
                    // top right
                    GlyphVertex {
                        pos: (gl_rect.max.x, gl_rect.min.y, 0.0).into(),
                        tex: (uv_rect.max.x, uv_rect.min.y).into(),
                        color: text_color.into(),
                    },
                    // bottom left
                    GlyphVertex {
                        pos: (gl_rect.min.x, gl_rect.max.y, 0.0).into(),
                        tex: (uv_rect.min.x, uv_rect.max.y).into(),
                        color: text_color.into(),
                    },
                    // bottom right
                    GlyphVertex {
                        pos: (gl_rect.max.x, gl_rect.max.y, 0.0).into(),
                        tex: (uv_rect.max.x, uv_rect.max.y).into(),
                        color: text_color.into(),
                    },
                ]
            })
            .collect();

        self.vbo.bind();
        self.vbo.reset_buffer_offset();

        if !vertices.is_empty() {
            for v in &vertices {
                self.vbo.upload_draw_data(v);
                self.vbo.set_buffer_offset(self.vbo.buffer_offset + ((::std::mem::size_of::<GlyphVertex>()) * 4) as isize);
            }
        }

        if !vertices.is_empty() {
            self.vertices = vertices;
            self.indices = generate_batch_indices(self.vertices.len());
            self.ibo.bind();
            self.ibo.static_draw_data(&self.indices);
            self.ibo.unbind();
        }

        self.texture.unbind();

        self.texture.bind_to_unit(0);

        let mvp = camera.get_projection() * camera.get_view();

        self.program.set_used();
        self.program.set_uniform_1i(*self.uniforms.get("texture").unwrap(), 0);
        self.program.set_uniform_mat4f(*self.uniforms.get("mvp").unwrap(), &mvp);

        self.ibo.bind();
        self.vao.bind();

        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32 * 6,
                gl::UNSIGNED_INT,
                self.indices.as_ptr() as *const gl::types::GLvoid,
            );
        }

        self.vao.unbind();
        self.ibo.unbind();

        self.vbo.reset_buffer_offset();
        self.vbo.unbind();
    }

    pub fn cache_scale_tolerance(&self) -> f32 {
        self.cache.scale_tolerance()
    }
}

// TODO: probably temporary, just trying to get some text on screen
fn generate_batch_indices(vertices_len: usize) -> Vec<[i32; 6]> {
    let mut offset: i32 = 0;
    let mut indices: Vec<[i32; 6]> = Vec::new();

    // TODO: maybe take in a format or base it off given vertices?
    // as this needs to match the order of a sprites vertices
    // this order is more of a top left to bottom right
    for _i in 0..vertices_len {
        let group: [i32; 6] = [
            offset,
            offset + 1,
            offset + 2,
            offset + 2,
            offset + 1,
            offset + 3,
        ];

        indices.push(group);

        offset += 4;
    }

    indices
}

