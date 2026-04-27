use crate::{
    geometry::space::{Vector, Vector2, Vector3},
    model::{Mesh, MeshGeometry, Vertex},
};

pub mod vit;
pub mod vit2;
pub mod riff;

fn parse_to_f32(input: String) -> Option<f32> {
    let mut minus = false;
    let mut decimal = false;
    let mut decimal_place = 0;
    let mut value = 0.0;
    for c in input.chars() {
        match c {
            '-' if !minus => {
                minus = true;
            }
            '.' if !decimal => {
                decimal = true;
            }
            c if c.is_digit(10) => {
                let digit = c.to_digit(10)? as f32;
                if decimal {
                    decimal_place -= 1;
                    value += digit * 10.0_f32.powi(decimal_place);
                } else {
                    value *= 10.0;
                    value += digit;
                }
            }
            _ => return None,
        }
    }
    Some(if minus { -value } else { value })
}

fn parse_to_u32(input: String) -> Option<u32> {
    let mut value = 0;
    for c in input.chars() {
        match c {
            c if c.is_digit(10) => {
                let digit = c.to_digit(10)?;
                value *= 10;
                value += digit;
            }
            _ => return None,
        }
    }
    Some(value)
}