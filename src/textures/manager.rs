use crate::texture::{Texture};
use crate::resources::{Resources};

pub struct TextureManager {
    resource: &Resources,
    textures: HashMap<Texture>,
}

impl TextureManager {
    pub fn new(resource: &Resources) -> TextureManager {
        TextureManager {
            resource,
            textures: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, texture: Texture) -> &str {
        self.textures.insert(name, texture);
        name
    }

    pub fn create(&mut self, name: &str, image_path: &str) -> &str {
        self.textures.insert(
            name,
            Texture::new(self.resource, image_path.to_string(), 0),
        );
        name
    }

    pub fn remove(&mut self, name: &str) -> Texture {
        let texture = self.textures.remove(name);
        texture
    }

    pub fn clear(&mut self) {
        self.textures.clear();
    }

    pub fn collection(self) -> &HashMap<Texture> {
        &self.textures
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_add_texture() {
        let text_resources = Resources::from_relative_path(Path::new("assets")).unwrap();
        let textures = TextureManager::new(&test_resources);

        assert_eq!("Textures", textures);
    }
}
