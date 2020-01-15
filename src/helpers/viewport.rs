pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Viewport {
    pub fn for_window(w: f32, h: f32) -> Viewport {
        Viewport { x: 0.0, y: 0.0, w, h }
    }

    pub fn update_size(&mut self, w: f32, h: f32) {
        self.w = w;
        self.h = h;
    }

    pub fn set_used(&self) {
        unsafe {
            gl::Viewport(self.x as i32, self.y as i32, self.w as i32, self.h as i32);
        }
    }
}
