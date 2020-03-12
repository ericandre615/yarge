use crate::helpers::{data};
use crate::resources::*;
use crate::textures::texture::*;

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
}

impl SpriteVertex {
    pub fn get_pos(&self) -> data::f32_f32_f32 {
        self.pos
    }

    pub fn get_tex(&self) -> data::f32_f32 {
        self.tex
    }

    pub fn get_color(&self) -> data::f32_f32_f32_f32 {
        self.color
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
            color: (0, 0, 0, 0.0),
            texture_slot: 0,
        }
    }
}
//let pos = glm::vec3(x, y, 0.0);
//let model = glm::translate(&glm::identity(), &pos);
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
    image_path: String,
    props: SpriteProps,
}

impl Sprite {
    pub fn new(res: &Resources, image_path: String, props: SpriteProps) -> Result<Sprite, failure::Error> {
        let texture = TextureBuilder::new(res, image_path.to_string())
            .with_texture_slot(props.texture_slot)
            .build()?;
        let mut transform = SpriteTransform::default();
        let (px, py, pz) = props.pos;

        transform.set_translation((px, py, pz));

        Ok(Sprite {
            vertices: update_vertices(&texture, &props),
            texture,
            transform,
            image_path,
            props,
        })
    }

    pub fn with_texture(texture: &Texture, props: SpriteProps) -> Result<Sprite, failure::Error> {
        let mut transform = SpriteTransform::default();

        transform.set_translation(props.pos);

        Ok(Sprite {
            vertices: update_vertices(&texture, &props),
            texture: texture.clone(),//TODO: how to handle only wantin ga ref if using an existing texture vs creating a sprite that creates it's own texture???
            transform,
            image_path: texture.image_path.to_string(), // TODO: need to probably use &strs
            props,
        })
    }

    pub fn transform(&mut self, transform: SpriteTransform) {
        self.transform = transform;
    }

    pub fn get_vertices(&self) -> &Vec<SpriteVertex> {
        &self.vertices
    }
}

fn update_vertices(texture: &Texture, props: &SpriteProps) -> Vec<SpriteVertex> {
    let (tw, th) = texture.get_dimensions();
    let (x, y, _) = props.pos; // TODO: exclude z for now
    let (width, height) = props.dim;
    let x2 = x + (width as f32);
    let y2 = y + (height as f32);
    let tx = width as f32 / tw as f32;
    let ty = height as f32 / th as f32;
    let color = normalize_color(props.color);
    let vertices: Vec<SpriteVertex> = vec![
       SpriteVertex { pos: (x, y, 0.0).into(), tex: (0.0, 0.0).into(), color: color.into() },
       SpriteVertex { pos: (x2, y, 0.0).into(), tex: (tx, 0.0).into(), color: color.into() },
       SpriteVertex { pos: (x, y2, 0.0).into(), tex: (0.0, ty).into(), color: color.into() },
       // second triangle
       SpriteVertex { pos: (x2, y2, 0.0).into(), tex: (tx, ty).into(), color: color.into() }
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

