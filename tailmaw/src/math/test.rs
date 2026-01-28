#[test]
fn test_matmul_identity() {
    use super::Matrix;
    let identity4x4: Matrix<4,4,usize> = Matrix::from([
        [1, 0, 0, 0],
        [0, 1, 0, 0],
        [0, 0, 1, 0],
        [0, 0, 0, 1],
    ]);
    assert_eq!(identity4x4*identity4x4, identity4x4)
}

#[test]
fn test_matmul_more_complicated() {
    use super::Matrix;
    let l_mat: Matrix<2,3,usize> = Matrix::from([
        [1, 2, 3],
        [4, 5, 6],
    ]);
    let r_mat: Matrix<3,4,usize> = Matrix::from([
        [1,  2,  3,  4],
        [5,  6,  7,  8],
        [9, 10, 11, 12],
    ]);
    let e_mat: Matrix<2,4,usize> = Matrix::from([
        [1*1 + 2*5 + 3*9, 1*2 + 2*6 + 3*10, 1*3 + 2*7 + 3*11, 1*4 + 2*8 + 3*12],
        [4*1 + 5*5 + 6*9, 4*2 + 5*6 + 6*10, 4*3 + 5*7 + 6*11, 4*4 + 5*8 + 6*12]
    ]);

    assert_eq!(l_mat*r_mat, e_mat)
}

#[test]
fn test_cross_product() {
    use super::{Cross,Vector};
    let x: Vector<3,isize> = Vector::from([1,0,0]);
    let y: Vector<3,isize> = Vector::from([0,1,0]);
    let z: Vector<3,isize> = Vector::from([0,0,1]);
    assert_eq!(x.cross(y), z);
    assert_eq!(y.cross(x), -z)
}
