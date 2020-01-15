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
}

impl Camera {
    pub fn new(width: f32, height: f32, kind: Projection) -> Result<Camera, failure::Error> {
        let projection = glm::ortho(0.0, width, height, 0.0, -10.0, 100.0);
        let view = glm::translate(&projection, &glm::vec3(0.0, 0.0, 0.0));

        Ok(Camera {
            projection,
            view,
            width,
            height,
            kind,
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

