use std::sync::Arc;

use sdl3::gpu::{Buffer, Texture};

use crate::math::{Vector2, Vector3};

#[derive(Clone)]
pub struct Model {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub texture: Texture<'static>,
}

#[derive(Clone)]
pub struct LoaderModel {
    pub mesh: Arc<Mesh>,
    pub texture: Arc<String>,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

#[derive(Clone, Copy)]
pub struct Vertex {
    pub texture_coord: Vector2,
    pub position: Vector3,
}
