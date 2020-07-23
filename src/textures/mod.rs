pub mod texture;
pub mod transform;

use std::collections::HashMap;

use texture::{Texture};
use crate::resources::{Resources};

#[derive(Debug)]
pub struct TextureManager<'a> {
    resource: &'a Resources,
    textures: HashMap<&'a str, Texture>,
}

impl<'a> TextureManager<'a> {
    pub fn new(resource: &'a Resources) -> TextureManager {
        TextureManager {
            resource,
            textures: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &'a str, texture: Texture) -> &str {
        self.textures.insert(name, texture);

        name
    }

    pub fn create(&mut self, name: &'a str, image_path: &str) -> Result<&str, failure::Error> {
        let texture = Texture::new(self.resource, image_path.to_string())?;
        self.textures.insert(name, texture);

        Ok(name)
    }

    pub fn remove(&mut self, name: &str) -> Result<Texture, failure::Error> {
        let texture = self.textures.remove(name).unwrap();

        Ok(texture)
    }

    pub fn clear(&mut self) {
        self.textures.clear();
    }

    pub fn get(&self, key: &str) -> &Texture {
        self.textures.get(key).unwrap()
    }

    //pub fn collection(self) -> &'a HashMap<String, Texture> {
    //    &self.textures
    //}
}

//#[cfg(test)]
//mod test {
//    use super::*;
//
//    #[test]
//    fn can_add_texture() {
//       use image::{DynamicImage, ImageBuffer, RgbaImage};
//       let test_resources = Resources::from_relative_path(Path::new("assets")).unwrap();
//       let mut texture_manager = TextureManager::new(&test_resources);
//       let empty_rgba: RgbaImage = ImageBuffer::new(10, 10);
//       let empty_image_data = DynamicImage::ImageRgba8(empty_rgba);
//       let added_key = texture_manager.add(
//           "test",
//           Texture {
//               texture_handle: 1,
//               texture_offset: 0,
//               image_data: empty_image_data,
//               image_path: "images/test.png".to_string(),
//           }
//       );
//
//       assert_eq!("test", added_key);
//       assert_eq!(HashMap::new(), texture_manager.textures);
//   }
//}
