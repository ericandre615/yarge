use crate::helpers::data;

pub trait RenderVertex {
    fn position(&self) -> data::f32_f32_f32 {
        (0.0, 0.0, 0.0).into()
    }
    fn uv(&self) -> data::f32_f32 {
        (0.0, 0.0).into()
    }
    fn color(&self) -> data::f32_f32_f32_f32 {
        (0.0, 0.0, 0.0, 0.0).into()
    }
    fn texture_translate(&self) -> data::f32_f32_f32 {
        (0.0, 0.0, 0.0).into()
    }
    fn texture_scale(&self) -> data::f32_f32_f32 {
        (0.0, 0.0, 0.0).into()
    }
}

pub trait Renderable2D {
    fn texture(&self) -> u32 {
        // default should be 0? or whatever determines
        // no texture
        0
    }

    fn vertices(&self) -> Vec<Box<dyn RenderVertex>> {
        Vec::new()
    }
}
