#[macro_use] extern crate failure;
#[macro_use] extern crate gl_vertex_derive;

extern crate gl;
extern crate nalgebra_glm as glm;
extern crate vec_2_10_10_10;

pub mod helpers;
pub mod resources;
pub mod textures;
pub mod triangle;
pub mod rectangle;
pub mod image;
pub mod camera;
pub mod renderer;
pub mod debug;
pub mod sprite;
pub mod tilemaps;
pub mod font;

pub use triangle::{Triangle};
pub use rectangle::{Rectangle, RectangleProps};
pub use helpers::viewport::Viewport;
