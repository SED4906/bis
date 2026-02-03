use std::sync::Arc;

use sdl3::gpu::Texture;

use crate::math::{Vector2, Vector3};

#[derive(Clone)]
pub struct Model<'a> {
    mesh: Arc<Mesh>,
    textures: Arc<Vec<Texture<'a>>>,
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
