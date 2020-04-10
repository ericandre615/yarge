pub mod text;
mod font_texture;
mod font_shaders;

use rusttype::gpu_cache::Cache;
use rusttype::{point, vector, Font, PositionedGlyph, Rect, Scale};
use std::borrow::Cow;
use std::fmt;
use std::collections::HashMap;

use crate::helpers::{self, data, buffer};
use crate::resources::{Resources};
use crate::camera::{Camera};

use font_shaders::{get_font_shaders};
use font_texture::{FontTexture, GlyphTexture};
pub use text::{Text, TextSettings};

use crate::rectangle::{Rectangle, RectangleProps};

#[derive(VertexAttribPointers)]
#[derive(Debug)]
#[repr(C, packed)]
pub struct TextVertex {
    #[location=0]
    pos: data::f32_f32_f32,
    #[location=1]
    tex: data::f32_f32,
    #[location=2]
    color: data::f32_f32_f32_f32,
}

pub type TextVertices = Vec<TextVertex>;

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
    vertices: Vec<TextVertices>,
    indices: Vec<[i32; 6]>,
    uniforms: HashMap<String, i32>,
    test_rects: Vec<Rectangle>,
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
    pub fn new(res: &'a Resources, screen_width: u32, screen_height: u32) -> Result<FontRenderer, failure::Error> {
        let scale_factor = 24;
        // maybe 512x512 is enough?
        // TODO: research Signed Distance Field
        let (cache_width, cache_height) = (
            1024,//(screen_width * scale_factor) as u32, // 512 needs to be screen width
            780,//(screen_height * scale_factor) as u32 // screen height
            // with_inner_size(glium::glutin::dpi::PhysicalSize::new(512, 512))
        );
        let mut cache = Cache::builder()
            .dimensions(cache_width, cache_height)
            .build();
        let (vert_src, frag_src) = get_font_shaders();
        let shaders = vec![
            helpers::Shader::from_raw(&vert_src, gl::VERTEX_SHADER)?,
            helpers::Shader::from_raw(&frag_src, gl::FRAGMENT_SHADER)?,
        ];
        let program = helpers::Program::from_shaders(&shaders[..], "internal/shaders/text")
            .expect("Failed to create Text Shader Program");
        let uniform_texture = program.get_uniform_location("GlyphTexture")?;
        //let uniform_mvp = program.get_uniform_location("MVP")?;
        let max_buffer_size = ((::std::mem::size_of::<TextVertex>()) * 4000) as gl::types::GLsizeiptr;
        let max_glyphs = 1000;

        let vbo = buffer::DynamicArrayBuffer::new(max_buffer_size);
        let vao = buffer::VertexArray::new();
        let ibo = buffer::ElementArrayBuffer::new();

        let indices = generate_batch_indices(max_glyphs);

        vbo.bind();
        vbo.set_buffer_data();
        vbo.unbind();

        vao.bind();
        vbo.bind();

        TextVertex::vertex_attrib_pointers();

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
            texture: FontTexture::new(1024, 1024),
            uniforms: vec![
                ("texture".to_owned(), uniform_texture),
                //("mvp".to_owned(), uniform_mvp),
            ].into_iter().collect(),
            test_rects: Vec::new(),
        })
    }

    pub fn add_font(&mut self, font_name: String, font_path: &str) {
        let font = self.res.load_font(font_path).unwrap();

        self.fonts.insert(font_name, font);
    }

    pub fn get_font(&self, font_name: &String) -> &Font {
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

    pub fn render(&mut self, text: Text, camera: &Camera) {
        //let font = self.get_font(&text.font);
        let (screen_width, screen_height) = camera.get_dimensions();
        let font = self.fonts.get(&text.settings.font)
            .expect(&format!("No Font {:#?} Found", text.settings.font));
        let res = self.res.clone();
        let text_color = (
            text.settings.color.0 as f32 / 255.0,
            text.settings.color.1 as f32 / 255.0,
            text.settings.color.2 as f32 / 255.0,
            text.settings.color.3
        );

        let glyphs = layout_text(
            font, //font,
            text.settings.size.scale,//scale,
            // probably need width field on Text
            1024, //width as u32, //width? of? full text? window?,
            &text.text//text_str
        );

        for glyph in &glyphs {
            self.cache.queue_glyph(0, glyph.clone());
        }
        // TODO: remove testing rects and implement
        // building the vertex data and writing to the texture
        let mut rects = Vec::new();
        let font_texture = &self.texture;
        let mut glyph_textures: Vec<GlyphTexture> = Vec::new();
        self.cache.cache_queued(|rect, data| {
            //println!("CACHE_QUEUED: {:#?}", rect);
            //println!("CACHE_DATA {:#?}", data);

            let glyph_rect = Rectangle::new(&res, &RectangleProps {
                width: rect.width() as f32,//rect.max.x as f32,
                height: rect.height() as f32,//rect.max.y as f32,
                pos: (rect.min.x as f32, rect.min.y as f32),//(rect.width() as f32, rect.height() as f32),
                color: (1.0, 0.0, 0.5, 1.0),//text_color,
            }).unwrap();

            rects.push(glyph_rect);
            // TODO: basically these need to happen
            let glyph_texture = GlyphTexture {
                left: rect.min.x,
                bottom: rect.min.y,
                width: rect.width(),
                height: rect.height(),
                data,
            };

            //glyph_textures.push(glyph_texture);
            font_texture.update(&glyph_texture);
        }).unwrap();

        //for gt in &glyph_textures {
        //    self.texture.update(gt);
        //}

        let origin = point(0.0, 0.0);
        let vertices: Vec<Vec<TextVertex>> = glyphs
            .iter()
            .filter_map(|g| self.cache.rect_for(0, g).ok().unwrap())//.flatten())
            .map(|(uv_rect, screen_rect)| {
                let gl_rect = Rect {
                    min: origin
                        + (vector(
                            screen_rect.min.x as f32 / screen_width - 0.5,
                            1.0 - screen_rect.min.y as f32 / screen_height - 0.5,
                        )) * 2.0,
                    max: origin
                        + (vector(
                            screen_rect.max.x as f32 / screen_width -0.5,
                            1.0 - screen_rect.max.y as f32 / screen_height - 0.5,
                        )) * 2.0,
                };
                //let gl_rect = Rect {
                //    min: origin + vector(screen_rect.min.x as f32, screen_rect.min.y as f32),
                //    max: origin + vector(screen_rect.max.x as f32, screen_rect.max.y as f32),
                //};

                //println!("DEBUG FONT Screen: sw {:#?}, sh {:#?}", screen_width, screen_height);
                //println!("DEBUG FONT Screen: {:#?}", screen_rect);
                //println!("DEBUG FONT GlRect: {:#?}", gl_rect);
                //println!("DEBUG FONT UV: {:#?}", uv_rect);

                // 0 top left
                // 1 top right
                // 2 bottom left
                // 3 bottom right
                vec![
                    TextVertex {
                        pos: (gl_rect.min.x, gl_rect.min.y, 0.0).into(),
                        tex: (uv_rect.min.x, uv_rect.min.y). into(),
                        color: text_color.into(),
                    },
                    TextVertex {
                        pos: (gl_rect.max.x, gl_rect.min.y, 0.0).into(),
                        tex: (uv_rect.max.x, uv_rect.min.y). into(),
                        color: text_color.into(),
                    },
                    TextVertex {
                        pos: (gl_rect.min.x, gl_rect.max.y, 0.0).into(),
                        tex: (uv_rect.min.x, uv_rect.max.y). into(),
                        color: text_color.into(),
                    },
                    TextVertex {
                        pos: (gl_rect.max.x, gl_rect.max.y, 0.0).into(),
                        tex: (uv_rect.max.x, uv_rect.max.y). into(),
                        color: text_color.into(),
                    },
                ]
            })
            .collect();

        // TODO: don't really like this, would like to just push to self.vertices (but borrow issue inside that
        // cache closure... need to figure that out.


        if !rects.is_empty() {
            self.test_rects = rects;
        }

        self.vbo.bind();
        self.vbo.reset_buffer_offset();

        if !vertices.is_empty() {
            for v in &vertices {
                self.vbo.upload_draw_data(v);
                self.vbo.set_buffer_offset(self.vbo.buffer_offset + ((::std::mem::size_of::<TextVertex>()) * 4) as isize);
            }
        }

        if !vertices.is_empty() {
            self.vertices = vertices;
        }

        println!("FONT VERTICES {:#?}", self.vertices);

        self.texture.bind_to_unit(0);

        let mvp = camera.get_projection() * camera.get_view();

        self.program.set_used();
        self.program.set_uniform_1i(*self.uniforms.get("texture").unwrap(), 0);
        //self.program.set_uniform_mat4f(*self.uniforms.get("mvp").unwrap(), &mvp);

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

        self.vbo.reset_buffer_offset();
        self.vbo.unbind();
    }

    pub fn get_test_rects(&mut self, camera: &Camera) {
        //self.test_rects//.clone()
        for rect in &self.test_rects {
            rect.render(camera);
        }
    }
}

// TODO: probably temporary, just trying to get some text on screen
fn generate_batch_indices(vertices_len: usize) -> Vec<[i32; 6]> {
    let mut offset: i32 = 0;
    let mut indices: Vec<[i32; 6]> = Vec::new();

    // TODO: maybe take in a format or base it off given vertices?
    // as this needs to match the order of a sprites vertices
    // this order is more of a top left to bottom right
    for i in (0..vertices_len) {
        let group: [i32; 6] = [
            offset + 0,
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

fn convert_points_to_pixels(size: f32) -> u32 {
    ((size as f32) * 1.333).round() as u32
}

// taken directly from the rusttype example
// https://github.com/redox-os/rusttype/blob/master/dev/examples/gpu_cache.rs
fn layout_text<'f>(
    font: &Font<'f>,
    scale: Scale,
    width: u32,
    text: &str,
) -> Vec<PositionedGlyph<'f>> {
    let mut result = Vec::new();
    let v_metrics = font.v_metrics(scale);
    let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
    let mut caret = point(0.0, v_metrics.ascent);
    let mut last_glyph_id = None;

    for c in text.chars() {
        if c.is_control() {
            match c {
                '\r' => {
                    caret = point(0.0, caret.y + advance_height);
                },
                '\n' => {},
                _ => {}
            }

            continue;
        }

        let base_glyph = font.glyph(c);
        if let Some(id) = last_glyph_id.take() {
            caret.x += font.pair_kerning(scale, id, base_glyph.id());
        }
        last_glyph_id = Some(base_glyph.id());
        let mut glyph = base_glyph.scaled(scale).positioned(caret);
        if let Some(bb) = glyph.pixel_bounding_box() {
            if bb.max.x > width as i32 {
                caret = point(0.0, caret.y + advance_height);
                glyph.set_position(caret);
                last_glyph_id = None;
            }
        }

        caret.x += glyph.unpositioned().h_metrics().advance_width;

        result.push(glyph);
    }

    result
}
