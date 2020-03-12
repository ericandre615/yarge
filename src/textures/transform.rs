pub struct TextureTransform {
    pub ortho: glm::Mat4,
    pub translate: glm::Mat4,
    pub scale: glm::Mat4,
    identity: glm::Mat4,
    width: u32,
    height: u32,
}

impl Default for TextureTransform {
    fn default() -> Self {
        let identity = glm::Mat4::identity();
        Self {
            ortho: glm::ortho(-1.0, 1.0, -1.0, 1.0, -2.0, 2.0),
            translate: glm::translate(&identity, &glm::vec3(0.0, 0.0, 0.0)),
            scale: glm::scale(&identity, &glm::vec3(1.0, 1.0, 0.0)),
            identity,
            width: 0,
            height: 0,
        }
    }
}

impl TextureTransform {
    pub fn new(width: u32, height: u32) -> TextureTransform {
        TextureTransform {
            //ortho: glm::ortho(0.0, width as f32, 0.0, height as f32, -1.0, 10.0),
            //ortho: glm::ortho(0.0, 1.0, 0.0, 1.0, -2.0, 2.0),
            //ortho: glm::ortho(0.0, 2.0, 0.0, 2.0, -1.0, 10.0),
            width,
            height,
            ..Default::default()
        }
    }

    pub fn get_transform(&self) -> glm::Mat4 {
        // self.ortho * self.scale * self.translate
        self.scale * self.translate
    }

    pub fn set_scale(&mut self, x: f32, y: f32) {
        self.scale = glm::scale(&self.identity, &glm::vec3(x, y, 0.0));
    }

    pub fn set_frame(&mut self, x: f32, y: f32) {
        let tx = x / self.width as f32;
        let ty = y / self.height as f32;
        self.translate = glm::translate(&self.identity, &glm::vec3(tx, ty, 0.0));
    }
}
