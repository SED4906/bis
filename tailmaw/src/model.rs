use sdl3::gpu::Texture;

use crate::math::{Vector2, Vector3};

pub struct Model<'a> {
    mesh: Mesh,
    textures: Vec<Texture<'a>>
}

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

pub struct Vertex {
    position: Vector3,
    texture_coord: Vector2,
    texture_index: u32,
}
