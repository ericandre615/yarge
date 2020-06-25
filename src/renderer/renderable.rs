trait Renderable2D {
    fn get_texture_handle(&self) -> u32;
    fn get_vertices(&self) -> Vec<SpriteVertex>;
}
