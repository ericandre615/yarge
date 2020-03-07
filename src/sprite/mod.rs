use crate::helpers::{data};
use crate::resources::*;
use crate::texture::*;

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct SpriteVertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    tex: data::f32_f32,
    #[location = 2]
    color: data::f32_f32_f32_f32,
}

#[derive(PartialEq, Debug)]
pub struct SpriteProps {
    pos: (f32, f32, f32),
    dim: (u32, u32),
    color: (u8, u8, u8, f32),
}

impl Default for SpriteProps {
    fn default() -> SpriteProps {
        SpriteProps {
            pos: (0.0, 0.0, 0.0),
            dim: (0, 0),
            color: (0, 0, 0, 0.0),
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
        // update model for each set?
    }

    pub fn get_scale(&self) -> glm::Mat4 {
        self.scale
    }

    pub fn set_scale(&mut self, scale: (f32, f32)) {
        let (sx, sy) = scale;
        self.scale = glm::scale(&glm::identity(), &glm::vec3(sx, sy, 1.0));
        self.set_model();
        // should each transform just use Identity?
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
        //self.model = self.scale * self.rotation * self.translation; // order matters, but this may not be it
        self.model = self.translation * self.rotation * self.scale;
    }
}

//pub struct SpriteBuilder {
//    resource: &'static Resources,
//    image_path: String,
//    transform: SpriteTransform,
//    props: SpriteProps,
//}
//
//impl SpriteBuilder {
//    pub fn new(resource: &Resources, image_path: String) -> Self {
//        Self {
//            resource,
//            image_path,
//            transform: SpriteTransform { ..Default::default() },
//            props: SpriteProps { ..Default::default() },
//        }
//    }
//
//    pub fn with_transform(mut self, transform: SpriteTransform) -> Self {
//        self.transform = transform;
//        self
//    }
//
//    pub fn with_props(mut self, props: SpriteProps) -> Self {
//        self.props = props;
//        self
//    }
//
//    pub fn build(self) -> Result<Sprite, failure::Error> {
//        Ok(Sprite::new(
//            self.resource,
//            self.image_path,
//            self.transform,
//            self.props
//        ))
//    }
//}
//
pub struct Sprite {
    transform: SpriteTransform,
    vertices: Vec<SpriteVertex>,
    texture: Texture,
    image_path: String,
    props: SpriteProps,
}

impl Sprite {
    pub fn new(res: &Resources, image_path: String, props: SpriteProps) -> Result<Sprite, failure::Error> {
        let texture = TextureBuilder::new(res, image_path.to_string())
            .build()?;
        let mut transform = SpriteTransform::default();
        let (px, py, pz) = props.pos;
        transform.set_translation((px, py, pz));

        Ok(Sprite {
            vertices: Vec::new(),
            texture,
            transform,
            image_path,
            props,
        })
    }

    pub fn transform(&mut self, transform: SpriteTransform) {
        self.transform = transform;
    }
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

// default rotation
//[
//    1.0, 0.0, 0.0, 0.0,
//    0.0, 1.0, 0.0, 0.0,
//    0.0, 0.0, 1.0, 0.0,
//    0.0, 0.0, 0.0, 1.0
//]

// default translation
//[
//    1.0, 0.0, 0.0, 0.0,
//    0.0, 1.0, 0.0, 0.0,
//    0.0, 0.0, 1.0, 0.0,
//    0.0, 0.0, 0.0, 1.0
//]

// default scale
//[
//    1.0, 0.0, 0.0, 0.0,
//    0.0, 1.0, 0.0, 0.0,
//    0.0, 0.0, 1.0, 0.0,
//    0.0, 0.0, 0.0, 1.0
//]

// with glm functions translate(scale(rotate)))
//[
//    0.3085028997751681, 0.0, 1.9760632481857237, 0.0,
//    0.0, 2.0, 0.0, 0.0,
//    -0.9880316240928618, 0.0, 0.15425144988758405, 0.0,
//    -2.6555919725034176, 4.0, 2.438817597848476, 1.0
//]

// with glm function scale(rotate(translate)))
//[
//    0.3085028997751681, 0.0, 0.9880316240928618, 0.0,
//    0.0, 2.0, 0.0, 0.0,
//    -1.9760632481857237, 0.0, 0.15425144988758405, 0.0,
//    1.0, 2.0, 3.0, 1.0
//]

// with identity
// with just *self.model = self.translation * self.rotation * self.scale;
//[
//    1.9601332, 0.0, -0.39733866, 0.0,
//    0.0, 2.0, 0.0, 0.0,
//    0.19866933, 0.0, 0.9800666, 0.0,
//    1.0, 2.0, 3.0, 1.0
//]

// with just * model = scale * rotate * translate
//[
//    1.9601332, 0.0, -0.19866933, 0.0,
//    0.0, 2.0, 0.0, 0.0,
//    0.39733866, 0.0, 0.9800666, 0.0,
//    3.1521492, 4.0, 2.7415304, 1.0
//]

// without identity

