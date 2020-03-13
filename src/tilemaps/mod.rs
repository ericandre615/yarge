use serde::{Serialize, Deserialize};
use serde_json;

use std::collections::HashMap;
use std::path::Path;

use crate::resources::*;
use crate::textures::texture::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tile {
    pos: (u32, u32),
    id: u32,
    tile: String,
    tile_type: String,
    width: u32,
    height: u32,
}

#[derive(Debug)]
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
    pub fn new(res: &Resources, image_path: String, tiles: HashMap<String, Tile>) -> Result<Tileset, failure::Error> {
        let texture = Texture::new(res, image_path)?;
        let (width, height) = texture.get_dimensions();

        Ok(Tileset {
            texture,
            width,
            height,
            tile_width: 32,
            tile_height: 32,
            tiles,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileLayer {
    height: u32,
    width:u32,
    name: String,
    opacity: f64,
    visible: bool,
    x: u32,
    y: u32,
    data: Vec<u32>,
    layer_type: String,
}

#[derive(Debug)]
pub struct Tilemap {
    tile_width: u32,
    tile_height: u32,
    layers: Vec<TileLayer>,
    tileset: Tileset,
}

impl Tilemap {
    pub fn from_json(res: &Resources, file_path: String) -> Result<Tilemap, failure::Error> {
        let json = res.load_from_json(&file_path)?;

        let tiles: HashMap<String, Tile> = serde_json::from_value(
            json["tilesets"]["tile_data"].clone()
        )?;
        let tileset_str = serde_json::from_value(
            json["tilesets"]["filepath"].clone()
        )?;
        //let tileset_path = Path::new(&tileset_str);
        let tileset = Tileset::new(res, tileset_str, tiles)?;

        let tilemap = Tilemap {
            tile_width: serde_json::from_value(json["tilewidth"].clone())?,
            tile_height: serde_json::from_value(json["tileheight"].clone())?,
            layers: serde_json::from_value(json["layers"].clone())?,
            tileset,
        };

        Ok(tilemap)
    }
}
