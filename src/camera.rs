pub enum Projection {
    Ortho,
    Perspective,
}

pub struct Camera {
    projection: glm::TMat4<f32>,
    view: glm::TMat4<f32>,
    width: f32,
    height: f32,
    kind: Projection,
    pos: glm::Vec3,
}

impl Camera {
    pub fn new(width: f32, height: f32, kind: Projection) -> Result<Camera, failure::Error> {
        let projection = glm::ortho(0.0, width, height, 0.0, -10.0, 100.0);
        let pos = glm::vec3(0.0, 0.0, 0.0);
        let view = glm::translate(&glm::identity(), &pos); //glm::translate(&projection, &glm::vec3(0.0, 0.0, 0.0));

        Ok(Camera {
            projection,
            view,
            width,
            height,
            kind,
            pos,
        })
    }

    pub fn get_projection(&self) -> glm::TMat4<f32> {
        self.projection
    }

    pub fn get_kind(&self) -> &Projection {
        &self.kind
    }

    pub fn get_view(&self) -> glm::TMat4<f32> {
        self.view
    }

    pub fn get_dimensions(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    pub fn get_position(&self) -> glm::Vec3 {
        self.pos
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.pos = glm::vec3(x, y, z);
        self.view = glm::translate(&glm::identity(), &self.pos);
    }

    pub fn set_posX(&mut self, x: f32) {
        self.set_position(x, self.pos.y, self.pos.z);
    }

    pub fn set_posY(&mut self, y: f32) {
        self.set_position(self.pos.x, y, self.pos.z);
    }

    pub fn set_posZ(&mut self, z: f32) {
        self.set_position(self.pos.x, self.pos.y, z);
    }

    pub fn update_width(&mut self, width: f32) {
        self.width = width;
        self.projection = glm::ortho(0.0, width, self.height, 0.0, -10.0, 100.0);

    }

    pub fn update_height(&mut self, height: f32) {
        self.height = height;
        self.projection = glm::ortho(0.0, self.width, height, 0.0, -10.0, 100.0);
    }

    pub fn update_viewport(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.projection = glm::ortho(0.0, width, height, 0.0, -10.0, 100.0);
    }
}

