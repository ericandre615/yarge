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

impl std::fmt::Debug for dyn RenderVertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "RenderVertex {{ position: {:?}, uv: {:?}, color: {:?}, texture_translate: {:?}, texture_scale: {:?} }}",
            self.position(),
            self.uv(),
            self.color(),
            self.texture_translate(),
            self.texture_scale(),
        )
    }
}

impl std::fmt::Debug for dyn RenderVertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "RenderVertex {{ position: {:?}, uv: {:?}, color: {:?}, texture_translate: {:?}, texture_scale: {:?} }}",
            self.position(),
            self.uv(),
            self.color(),
            self.texture_translate(),
            self.texture_scale(),
        )
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
