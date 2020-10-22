use crate::camera::Projection;

pub struct Camera3D {
    projection: glm::TMat4<f32>,
    view: glm::TMat4<f32>,
    width: f32,
    height: f32,
    aspect: f32,
    fov: f32,
    kind: Projection,
    pos: glm::Vec3,
    is_looking_at: bool,
    lookat_target: (f32, f32, f32),
}

impl Camera3D {
    pub fn new(width: f32, height: f32, fov: f32, kind: Projection) -> Result<Camera3D, failure::Error> {
        let aspect = width / height;
        // use glm::radians? is there type for that?
        let fovy = 50.0; // field of view in radians, good default? 50 used by three.js?
        let projection = glm::perspective(aspect, fov, 0.5, 100.0);
        let pos = glm::vec3(0.0, 0.0, 0.0);
        let view = glm::translate(&glm::identity(), &pos); //glm::translate(&projection, &glm::vec3(0.0, 0.0, 0.0));

        Ok(Camera3D {
            projection,
            view,
            width,
            height,
            aspect,
            fov,
            kind,
            pos,
            is_looking_at: false,
            lookat_target: (0.0, 0.0, 0.0),
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

    pub fn get_fov(&self) -> f32 {
        self.fov
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.pos = glm::vec3(x, y, z);
        if self.is_looking_at {
            self.look_at(self.lookat_target);
        } else {
            self.view = glm::translate(&glm::identity(), &self.pos);
        }
    }

    pub fn set_pos_x(&mut self, x: f32) {
        self.set_position(x, self.pos.y, self.pos.z);
    }

    pub fn set_pos_y(&mut self, y: f32) {
        self.set_position(self.pos.x, y, self.pos.z);
    }

    pub fn set_pos_z(&mut self, z: f32) {
        self.set_position(self.pos.x, self.pos.y, z);
    }

    pub fn update_width(&mut self, width: f32) {
        self.width = width;
        self.aspect = width / self.height;

        self.update_perspective();
    }

    pub fn update_height(&mut self, height: f32) {
        self.height = height;
        self.aspect = self.width / height;

        self.update_perspective();
    }

    pub fn update_viewport(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.aspect = width / height;

        self.update_perspective();
    }

    pub fn update_fov(&mut self, fov: f32) {
        self.fov = fov;

        self.update_perspective();
    }

    pub fn look_at(&mut self, target: (f32, f32, f32)) {
        let up = glm::vec3(0.0, 1.0, 0.0);
        let (x, y, z) = target;
        let look_at_target = glm::vec3(x, y, z);

        self.is_looking_at = true;
        self.view = glm::look_at(&self.pos, &look_at_target, &up);
    }

    pub fn cancel_look_at(&mut self) {
        self.is_looking_at = false;
        self.view = glm::translate(&glm::identity(), &self.pos);
    }

    fn update_perspective(&mut self) {
        self.projection = glm::perspective(self.aspect, self.fov, -10.0, 100.0);
    }
}

