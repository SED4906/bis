use crate::{
    formats::riff::Riff,
    geometry::space::{Vector2, Vector3},
    model::{Mesh, MeshGeometry, Vertex},
};

pub fn vit2(input: &[u8]) -> Option<Mesh> {
    let mut vertices = vec![];
    let mut indices = vec![];
    let mut texture = String::new();
    let riff = Riff::new(input)?;
    if !riff.has_id("VIT ") {
        return None;
    }
    for vchunk in riff.get_chunks("V   ")? {
        for vertex in vchunk.as_chunks::<20>().0 {
            let chunks = vertex.as_chunks::<4>().0;
            let x = f32::from_le_bytes(chunks[0]);
            let y = f32::from_le_bytes(chunks[1]);
            let z = f32::from_le_bytes(chunks[2]);
            let u = f32::from_le_bytes(chunks[3]);
            let v = f32::from_le_bytes(chunks[4]);
            let data = Vertex {
                texture_coord: Vector2::new([u, v]),
                position: Vector3::new([x, y, z]),
            };
            vertices.push(data);
        }
    }
    for ichunk in riff.get_chunks("I   ")? {
        for index in ichunk.as_chunks::<4>().0 {
            indices.push(u32::from_le_bytes(*index));
        }
    }
    for tchunk in riff.get_chunks("T   ")? {
        texture.push_str(&String::from_utf8_lossy(tchunk).to_string());
    }
    Some(Mesh {
        mesh: MeshGeometry { vertices, indices }.into(),
        texture: texture.into(),
    })
}
