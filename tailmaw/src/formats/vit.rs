use crate::{formats::{parse_to_f32, parse_to_u32}, geometry::space::{Vector2, Vector3}, model::{Mesh, MeshGeometry, Vertex}};

pub enum VitParseState {
    None,
    Vertex,
    Index,
    Texture,
}

pub fn vit(input: &str) -> Option<Mesh> {
    let mut vertices = vec![];
    let mut indices = vec![];
    let mut texture = String::new();
    let mut part = String::new();
    let mut vertex_parts = vec![];
    let mut state = VitParseState::None;
    for c in input.chars() {
        match (&state, c) {
            (_, 'V') => {
                state = VitParseState::Vertex;
            }
            (_, 'I') => {
                state = VitParseState::Index;
            }
            (_, 'T') => {
                state = VitParseState::Texture;
            }
            (VitParseState::Vertex, ' ' | '\t' | '\r' | '\n') => {
                if !part.is_empty() {
                    vertex_parts.push(parse_to_f32(part.clone())?);
                }
                if vertex_parts.len() >= 5 {
                    vertices.push(Vertex {
                        position: Vector3::new(vertex_parts[0..3].try_into().unwrap()),
                        texture_coord: Vector2::new(vertex_parts[3..5].try_into().unwrap()),
                    });
                    vertex_parts.clear();
                }
                part.clear();
            }
            (VitParseState::Index, ' ' | '\t' | '\r' | '\n') => {
                if !part.is_empty() {
                    indices.push(parse_to_u32(part.clone())?);
                }
                part.clear();
            }
            (VitParseState::Texture, ' ' | '\t' | '\r' | '\n') => {
                texture = part.clone();
                part.clear();
            }
            (_, c) => {
                part.push(c);
            }
        }
    }
    Some(Mesh {
        mesh: MeshGeometry { vertices, indices }.into(),
        texture: texture.into(),
    })
}
