use crate::{
    math::{Quaternion, Vector},
    physics::boxes::{OrientedCube, cube_cube},
};

#[test]
fn test_cube_cube_simple_nonintersecting() {
    let c1 = OrientedCube {
        position: Vector::default(),
        rotation: Quaternion {
            r: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        scale: 1.0,
    };
    let c2 = OrientedCube {
        position: Vector::from([0.0, 0.0, 2.0]),
        rotation: Quaternion {
            r: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        scale: 1.0,
    };
    assert!(!cube_cube(c1, c2))
}

#[test]
fn test_cube_cube_simple_intersecting() {
    let c1 = OrientedCube {
        position: Vector::default(),
        rotation: Quaternion {
            r: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        scale: 1.0,
    };
    let c2 = OrientedCube {
        position: Vector::from([0.5, 0.5, 0.5]),
        rotation: Quaternion {
            r: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        scale: 1.0,
    };
    assert!(cube_cube(c1, c2))
}

#[test]
fn test_cube_cube_cross_nonintersecting() {
    let c1 = OrientedCube {
        position: Vector::default(),
        rotation: Quaternion {
            r: 0.9238795,
            x: 0.3826834,
            y: 0.0,
            z: 0.0,
        },
        scale: 1.0,
    };
    let c2 = OrientedCube {
        position: Vector::from([0.0, 0.0, 1.42]),
        rotation: Quaternion {
            r: 0.9238795,
            x: 0.0,
            y: 0.3826834,
            z: 0.0,
        },
        scale: 1.0,
    };
    assert!(!cube_cube(c1, c2))
}
