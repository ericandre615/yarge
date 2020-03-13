use std::collections::HashMap;

pub stuct Tile {
    pos: (u32, u32),
    id: u32,
    tile: String,
    kind: String,
    width: u32,
    height: u32,
}

pub struct Tileset {
    texture: Texture,
    width: u32,
    height: u32,
    tile_width: u32,
    tile_height: u32,
    tiles: HashMap<String, Tile>,
    //rect: [f64; 4],
    //src_rect: SourceRectangle,
}

impl Tileset {
    pub fn new(res: &Resources, image_path: String) -> Tileset {
        let texture = Texture::new(res, image_path)?;
        Tileset {
            texture,
            width:,
            height:,
            tile_width:,
            tile_height:,
            tiles:,
        }
    }
}

pub struct TileLayer {
    height: u32,
    width:u32,
    name: String,
    opacity: f64,
    visible: bool,
    x: u32,
    y: u32,
    data: Vec<u32>,
    kind: String,
}

pub struct Tilemap {
    tilewidth: u32,
    tileheight: u32,
    layers: Vec<TileLayer>,
    tileset: Tileset,
}
