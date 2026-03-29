use crate::geometry::space::{
    Matrix4, Quaternion, Vector3,
    vector::{X, Y, Z},
};

pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Matrix4 {
    let num = 1.0 / (fov * 0.5).tan();
    Matrix4::new([
        [num / aspect, 0.0, 0.0, 0.0],
        [0.0, num, 0.0, 0.0],
        [0.0, 0.0, (near + far) / (near - far), -1.0],
        [0.0, 0.0, (2.0 * near * far) / (near - far), 0.0],
    ])
}

pub fn look_at(cam: Vector3, target: Vector3, up: Vector3) -> Matrix4 {
    let a = (cam - target).normalized();
    let b = up.cross(a).normalized();
    let c = a.cross(b);
    Matrix4::new([
        [b.x(), c.x(), a.x(), 0.0],
        [b.y(), c.y(), a.y(), 0.0],
        [b.z(), c.z(), a.z(), 0.0],
        [-(b * cam), -(c * cam), -(a * cam), 1.0],
    ])
}

pub fn transformation(translation: Vector3, rotation: Quaternion, scale: Vector3) -> Matrix4 {
    let translation = Matrix4::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [translation.x(), translation.y(), translation.z(), 1.0],
    ]);
    let rotation = Matrix4::new([
        [
            1.0 - 2.0 * (rotation.y * rotation.y + rotation.z * rotation.z),
            2.0 * (rotation.x * rotation.y - rotation.z * rotation.r),
            2.0 * (rotation.x * rotation.z + rotation.y * rotation.r),
            0.0,
        ],
        [
            2.0 * (rotation.x * rotation.y + rotation.z * rotation.r),
            1.0 - 2.0 * (rotation.x * rotation.x + rotation.z * rotation.z),
            2.0 * (rotation.y * rotation.z - rotation.x * rotation.r),
            0.0,
        ],
        [
            2.0 * (rotation.x * rotation.z - rotation.y * rotation.r),
            2.0 * (rotation.y * rotation.z + rotation.x * rotation.r),
            1.0 - 2.0 * (rotation.x * rotation.x + rotation.y * rotation.y),
            0.0,
        ],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    let scale = Matrix4::new([
        [scale.x(), 0.0, 0.0, 0.0],
        [0.0, scale.y(), 0.0, 0.0],
        [0.0, 0.0, scale.z(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    scale * rotation * translation
}
