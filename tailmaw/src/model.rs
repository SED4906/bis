use crate::math::Vector;

pub struct Model {
    mesh: Mesh,
}

pub struct Mesh {
    vertices: Vec<Vertex>,
    triangles: Vec<Vector<3,u32>>,
}

pub struct Vertex {
    position: Vector<3, f32>,
    texture_coord: Vector<2, f32>,
    texture_index: u32,
}
