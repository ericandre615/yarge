use crate::helpers::{data};
use crate::resources::*;
use crate::textures::texture::{Texture};
use crate::textures::transform::{TextureTransform};
use crate::renderer::renderable::{Renderable2D, RenderVertex};

pub mod animation;

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C, packed)]
pub struct SpriteVertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    tex: data::f32_f32,
    #[location = 2]
    color: data::f32_f32_f32_f32,
    #[location = 3]
    tex_translate: data::f32_f32_f32,
    #[location = 4]
    tex_scale: data::f32_f32_f32,
}

impl RenderVertex for SpriteVertex {
    fn position(&self) -> data::f32_f32_f32 {
        self.pos
    }

    fn uv(&self) -> data::f32_f32 {
        self.tex
    }

    fn color(&self) -> data::f32_f32_f32_f32 {
        self.color
    }

    fn texture_translate(&self) -> data::f32_f32_f32 {
        self.tex_translate
    }

    fn texture_scale(&self) -> data::f32_f32_f32 {
        self.tex_scale
    }
}

#[derive(PartialEq, Debug)]
pub struct SpriteProps {
    pub pos: (f32, f32, f32),
    pub dim: (u32, u32),
    pub color: (u8, u8, u8, f32),
    pub texture_slot: u32,
}

impl Default for SpriteProps {
    fn default() -> SpriteProps {
        SpriteProps {
            pos: (0.0, 0.0, 0.0),
            dim: (0, 0),
            color: (255, 255, 255, 1.0),
            texture_slot: 0,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct SpriteTransform {
    translation: glm::Mat4,
    scale: glm::Mat4,
    rotation: glm::Mat4,
    model: glm::Mat4
}

impl Default for SpriteTransform {
    fn default() -> SpriteTransform {
        let translation = glm::translate(&glm::identity(), &glm::vec3(0.0, 0.0, 0.0));
        let scale = glm::scale(&translation, &glm::vec3(1.0, 1.0, 1.0));
        let rotation = glm::rotate(&scale, 0.0, &glm::vec3(0.0, 0.0, 0.0)); // what's normalized?
        SpriteTransform {
            translation,
            scale,
            rotation,
            model: translation,
        }
    }
}

impl SpriteTransform {
    pub fn get_translation(&self) -> glm::Mat4 {
        self.translation
    }

    pub fn set_translation(&mut self, pos: (f32, f32, f32)) {
        let (x, y, z) = pos;
        self.translation = glm::translate(&glm::identity(), &glm::vec3(x, y, z));
        self.set_model();
    }

    pub fn get_scale(&self) -> glm::Mat4 {
        self.scale
    }

    pub fn set_scale(&mut self, scale: (f32, f32)) {
        let (sx, sy) = scale;
        self.scale = glm::scale(&glm::identity(), &glm::vec3(sx, sy, 1.0));
        self.set_model();
    }

    pub fn get_rotation(&self) -> glm::Mat4 {
        self.rotation
    }

    pub fn set_rotation(&mut self, rads: f32) {
        self.rotation = glm::rotate(&glm::identity(), rads, &glm::vec3(0.0, 1.0, 0.0)); // what's normalized rotation?
        self.set_model();
    }

    pub fn get_model(&self) -> glm::Mat4 {
        self.model
    }

    fn set_model(&mut self) {
        self.model = self.translation * self.rotation * self.scale;
    }
}

#[derive(PartialEq, Debug)]
pub struct Sprite {
    transform: SpriteTransform,
    vertices: Vec<SpriteVertex>,
    pub texture: Texture,
    texture_transform: TextureTransform,
    image_path: String,
    pub props: SpriteProps,
}

impl Renderable2D for Sprite {
    fn texture(&self) -> u32 {
        self.texture.texture_handle
    }

    fn vertices(&self) -> Vec<Box<dyn RenderVertex>> {
        let mut v = Vec::new();
        for sv in &self.vertices {
            v.push(Box::new(*sv) as Box<dyn RenderVertex>);
        }

        v
    }
}

impl Sprite {
    pub fn new(res: &Resources, image_path: String, props: SpriteProps) -> Result<Sprite, failure::Error> {
        let texture = Texture::new(res, image_path.to_string())?;
        let (tw, th) = texture.get_dimensions();
        let mut transform = SpriteTransform::default();
        let texture_transform = TextureTransform::new(tw, th);
        let (px, py, pz) = props.pos;

        transform.set_translation((px, py, pz));

        Ok(Sprite {
            vertices: update_vertices(&texture, &props, &texture_transform),
            texture,
            transform,
            texture_transform,
            image_path,
            props,
        })
    }

    pub fn from_texture(texture: &Texture, props: SpriteProps) -> Result<Sprite, failure::Error> {
        let mut transform = SpriteTransform::default();
        let (tw, th) = texture.get_dimensions();
        let texture_transform = TextureTransform::new(tw, th);

        transform.set_translation(props.pos);

        Ok(Sprite {
            vertices: update_vertices(&texture, &props, &texture_transform),
            texture: texture.clone(),//TODO: how to handle only wantin ga ref if using an existing texture vs creating a sprite that creates it's own texture???
            transform,
            texture_transform,
            image_path: texture.image_path.to_string(), // TODO: need to probably use &strs
            props,
        })
    }

    pub fn transform(&mut self, transform: SpriteTransform) {
        self.transform = transform;
    }

    pub fn set_texture_scale(&mut self, scale: (f32, f32)) {
        let (sx, sy) = scale;
        self.texture_transform.set_scale(sx, sy);
        self.vertices = update_vertices(&self.texture, &self.props, &self.texture_transform);
    }

    pub fn set_frame(&mut self, pos: (f32, f32)) {
        let (x, y) = pos;
        self.texture_transform.set_frame(x, y);
        self.vertices = update_vertices(&self.texture, &self.props, &self.texture_transform);
    }

    pub fn set_position(&mut self, pos: (f32, f32, f32)) {
        let (x, y, z) = pos;
        self.transform.set_translation((x, y, z));
        self.props.pos = pos;
        self.vertices = update_vertices(&self.texture, &self.props, &self.texture_transform);
    }

    pub fn get_vertices(&self) -> &Vec<SpriteVertex> {
        &self.vertices
    }
}

fn update_vertices(texture: &Texture, props: &SpriteProps, texture_transform: &TextureTransform) -> Vec<SpriteVertex> {
    let (tw, th) = texture.get_dimensions();
    let (x, y, _) = props.pos; // TODO: exclude z for now
    let (width, height) = props.dim;
    let x2 = x + (width as f32);
    let y2 = y + (height as f32);
    let tx = width as f32 / tw as f32;
    let ty = height as f32 / th as f32;
    let tex_translate = texture_transform.get_raw_translate();
    let tex_scale = texture_transform.get_raw_scale();
    let color = normalize_color(props.color);
    let vertices: Vec<SpriteVertex> = vec![
       SpriteVertex { pos: (x, y, 0.0).into(), tex: (0.0, 0.0).into(), color: color.into(), tex_translate: tex_translate.into(), tex_scale: tex_scale.into() },
       SpriteVertex { pos: (x2, y, 0.0).into(), tex: (tx, 0.0).into(), color: color.into(), tex_translate: tex_translate.into(), tex_scale: tex_scale.into() },
       SpriteVertex { pos: (x, y2, 0.0).into(), tex: (0.0, ty).into(), color: color.into(), tex_translate: tex_translate.into(), tex_scale: tex_scale.into() },
       // second triangle
       SpriteVertex { pos: (x2, y2, 0.0).into(), tex: (tx, ty).into(), color: color.into(), tex_translate: tex_translate.into(), tex_scale: tex_scale.into() }
    ];

    vertices
}
// TODO: this is clearly used a lot, need to find better single place for this type of thing
fn normalize_color(color: (u8, u8, u8, f32)) -> (f32, f32, f32, f32) {
    let (r, g, b, a) = color;
    (
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_set_translate() {
        let mut transform = SpriteTransform::default();
        let expected_transform = [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            2.5, 2.5, 1.5, 1.0
        ];
        transform.set_translation((2.5, 2.5, 1.5));

        assert_eq!(expected_transform, transform.get_translation().data.as_slice());
    }

    #[test]
    fn can_set_scale() {
        let mut transform = SpriteTransform::default();
        let expected_transform = [
            2.0, 0.0, 0.0, 0.0,
            0.0, 2.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        ];
        transform.set_scale((2.0, 2.0));

        assert_eq!(expected_transform, transform.get_scale().data.as_slice());
    }

    #[test]
    fn can_set_rotation() {
        // TODO: not sure about rotation
        let mut transform = SpriteTransform::default();
        let expected_transform = [
            0.921061, 0.0, -0.38941833, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.38941833, 0.0, 0.921061, 0.0,
            0.0, 0.0, 0.0, 1.0
        ];
        transform.set_rotation(0.40);

        assert_eq!(expected_transform, transform.get_rotation().data.as_slice());
    }

    #[test]
    fn model_is_updated_on_change() {
        let mut transform = SpriteTransform::default();
        let expected_transform = [
            1.842122, 0.0, -0.77883667, 0.0,
            0.0, 2.0, 0.0, 0.0,
            0.38941833, 0.0, 0.921061, 0.0,
            2.5, 2.5, 1.5, 1.0
        ];
        transform.set_scale((2.0, 2.0));
        transform.set_translation((2.5, 2.5, 1.5));
        transform.set_rotation(0.40);

        assert_eq!(expected_transform, transform.get_model().data.as_slice());
    }
}

