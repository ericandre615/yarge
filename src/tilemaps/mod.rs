use serde::{Serialize, Deserialize};
use serde_json;

use std::collections::HashMap;
use std::path::Path;

use crate::resources::*;
use crate::textures::texture::*;
use crate::sprite::{Sprite, SpriteProps};

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

    pub fn get_texture(&self) -> &Texture {
        &self.texture
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
    // TODO: temporary here because easier to access, might need ot work with layers in some way?
    vertices: Vec<Sprite>,
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
        let tileset = Tileset::new(res, tileset_str, tiles)?;
        let layers: Vec<TileLayer> = serde_json::from_value(json["layers"].clone())?;

        let vertices = generate_vertices_from_layer(&layers[0], &tileset);
        let tilemap = Tilemap {
            tile_width: serde_json::from_value(json["tilewidth"].clone())?,
            tile_height: serde_json::from_value(json["tileheight"].clone())?,
            layers,
            tileset,
            vertices,
        };

        Ok(tilemap)
    }

    pub fn get_vertices(&self) -> &Vec<Sprite> {
        &self.vertices
    }
}

fn generate_vertices_from_layer(layer: &TileLayer, tileset: &Tileset) -> Vec<Sprite> {
    // TODO: need to go through array of data, any value that can mapped to a tile image needs
    // 4 vertex, position of x and y for vertex needs to be determined from total size map in pixels
    // vs size of tiles (assume all tiles have to be the same size
    // should we just make them all sprites?
    // possibly use Sprite::from_texture? how will textures work as we haven't been using texture transforms
    // for Sprites and we need to move the position of the texture for each sprite since the image is a single
    // image of many tiles

    // NOTE: tile.pos [x, y] is the position on our texture, we need to handle this somehow or
    // maybe better is finding a way to easily define this or make it apar tof a "tile" through openGL?
    // like a subimage/subtexture? not sure the best way to handle this. Maybe need a custom Spritesheet? or TextureAtlas?
    // type that can handle all that?

    let mut vertices = Vec::new();
    let cols_len = layer.width / tileset.tile_width -1;
    let mut col = 0;
    let mut row = 0;

    for tile_id in layer.data.iter() {
        let tile = tileset.tiles.get(&String::from(tile_id.to_string()));
        // maybe this is the position on the map/layer/world?
        let tx = (tileset.tile_width * col) as f32;
        let ty = (tileset.tile_height * row) as f32;

        match tile {
            Some(t) => {
                let mut sprite_tile = Sprite::from_texture(
                    tileset.get_texture(),
                    SpriteProps {
                        pos: (tx, ty, 0.0),
                        dim: (tileset.tile_width, tileset.tile_height),
                        ..Default::default()
                    }
                ).unwrap();
                let (tx, ty) = t.pos;
                sprite_tile.set_frame((tx as f32, ty as f32));
                vertices.push(sprite_tile);
            },
            None => (),
        }

        if col == cols_len {
            col = 0;
            row = row + 1;
        } else {
            col += 1;
        }
    }

    vertices
}

