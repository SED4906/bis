#[cfg(test)]
mod test;

pub mod vector;
pub mod matrix;
pub mod quaternion;

pub use vector::Vector;
pub use matrix::Matrix;
pub use quaternion::Quaternion;

pub type Vector2 = Vector<2, f32>;
pub type Vector3 = Vector<3, f32>;