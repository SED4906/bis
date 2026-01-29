use crate::math::{Quaternion, Vector, Vector3};

pub struct OrientedCube {
    // Center of the cube
    pub position: Vector3,
    pub rotation: Quaternion,
    // Length of edges
    pub scale: f32,
}

pub fn cube_cube(c1: OrientedCube, c2: OrientedCube) -> bool {
    // First, check if the cubes are even close together.
    if (c1.position - c2.position).mag() > (c1.scale + c2.scale) * 1.5 {
        return false;
    }
    // If so, use the Separating Axis Theorem to check intersection.
    let mut axes = [Vector::default(); 15];
    axes[0] = Vector::new([1.0, 0.0, 0.0]).rotate(c1.rotation);
    axes[1] = Vector::new([0.0, 1.0, 0.0]).rotate(c1.rotation);
    axes[2] = Vector::new([0.0, 0.0, 1.0]).rotate(c1.rotation);
    axes[3] = Vector::new([1.0, 0.0, 0.0]).rotate(c2.rotation);
    axes[4] = Vector::new([0.0, 1.0, 0.0]).rotate(c2.rotation);
    axes[5] = Vector::new([0.0, 0.0, 1.0]).rotate(c2.rotation);
    axes[6] = axes[0].cross(axes[3]);
    axes[7] = axes[0].cross(axes[4]);
    axes[8] = axes[0].cross(axes[5]);
    axes[9] = axes[1].cross(axes[3]);
    axes[10] = axes[1].cross(axes[4]);
    axes[11] = axes[1].cross(axes[5]);
    axes[12] = axes[2].cross(axes[3]);
    axes[13] = axes[2].cross(axes[4]);
    axes[14] = axes[2].cross(axes[5]);
    let verts1 = [
        c1.position - (axes[0] + axes[1] + axes[2]) * c1.scale * 0.5, // - - -
        c1.position + (axes[2] - axes[0] - axes[1]) * c1.scale * 0.5, // - - +
        c1.position + (axes[1] - axes[2] - axes[0]) * c1.scale * 0.5, // - + -
        c1.position + (axes[1] + axes[2] - axes[0]) * c1.scale * 0.5, // - + +
        c1.position + (axes[0] - axes[1] - axes[2]) * c1.scale * 0.5, // + - -
        c1.position + (axes[0] + axes[2] - axes[1]) * c1.scale * 0.5, // + - +
        c1.position + (axes[0] + axes[1] - axes[2]) * c1.scale * 0.5, // + + -
        c1.position + (axes[0] + axes[1] + axes[2]) * c1.scale * 0.5, // + + +
    ];
    let verts2 = [
        c2.position - (axes[3] + axes[4] + axes[5]) * c2.scale * 0.5, // - - -
        c2.position + (axes[5] - axes[3] - axes[4]) * c2.scale * 0.5, // - - +
        c2.position + (axes[4] - axes[5] - axes[3]) * c2.scale * 0.5, // - + -
        c2.position + (axes[4] + axes[5] - axes[3]) * c2.scale * 0.5, // - + +
        c2.position + (axes[3] - axes[4] - axes[5]) * c2.scale * 0.5, // + - -
        c2.position + (axes[3] + axes[5] - axes[4]) * c2.scale * 0.5, // + - +
        c2.position + (axes[3] + axes[4] - axes[5]) * c2.scale * 0.5, // + + -
        c2.position + (axes[3] + axes[4] + axes[5]) * c2.scale * 0.5, // + + +
    ];
    for axis in axes {
        let proj1 = verts1.map(|v| v * axis);
        let proj2 = verts2.map(|v| v * axis);
        let min1 = proj1.into_iter().reduce(f32::min).unwrap();
        let max1 = proj1.into_iter().reduce(f32::max).unwrap();
        let min2 = proj2.into_iter().reduce(f32::min).unwrap();
        let max2 = proj2.into_iter().reduce(f32::max).unwrap();
        if (min1 < min2 && max1 < min2) || (min2 < min1 && max2 < min1) {
            return false;
        }
    }
    true
}
