use nalgebra_glm as glm;

pub struct ColorBuffer {
    pub color: glm::Vec4,
}

impl ColorBuffer {
    pub fn from_color(color: glm::Vec3) -> ColorBuffer {
        ColorBuffer {
            color: color.fixed_resize::<glm::U4, glm::U1>(1.0),
        }
    }

    pub fn update_color(&mut self, color: glm::Vec3) {
        self.color = color.fixed_resize::<glm::U4, glm::U1>(1.0);
    }

    pub fn set_used(&self) {
        unsafe {
            gl::ClearColor(self.color.x, self.color.y, self.color.z, 1.0);
        }
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
