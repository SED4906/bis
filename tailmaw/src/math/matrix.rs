use std::{
    fmt::{self, Debug, Display, Formatter},
    iter::zip,
    ops::{Add, Mul, Sub},
};

use super::vector::Vector;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Matrix<const M: usize, const N: usize, T> {
    pub values: [Vector<N, T>; M],
}

impl<const M: usize, const N: usize, T: Copy + Default> Matrix<M, N, T> {
    pub fn new() -> Self {
        Self {
            values: [Vector::new(); M],
        }
    }

    pub fn column(self, index: usize) -> Vector<M, T> {
        let mut result = Vector::new();
        for (row, value) in result.values.iter_mut().enumerate() {
            *value = self.values[row].values[index];
        }
        result
    }
}

impl<const M: usize, const N: usize, T: Copy + Default> From<[[T; N]; M]> for Matrix<M, N, T> {
    fn from(values: [[T; N]; M]) -> Self {
        Self {
            values: values.map(|r| r.into()),
        }
    }
}

impl<const M: usize, const N: usize, T: Display> Display for Matrix<M, N, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("[[\n")?;
        for value in &self.values {
            f.write_fmt(format_args!("{}\n", value))?;
        }
        f.write_str("]]")?;
        Ok(())
    }
}

impl<const M: usize, const N: usize, T: Debug> Debug for Matrix<M, N, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("[[\n")?;
        for value in &self.values {
            f.write_fmt(format_args!("{:?}\n", value))?;
        }
        f.write_str("]]")?;
        Ok(())
    }
}

impl<const M: usize, const N: usize, T: Copy + Default + Add<Output = T>> Add<Matrix<M, N, T>>
    for Matrix<M, N, T>
{
    type Output = Matrix<M, N, T>;

    fn add(self, rhs: Matrix<M, N, T>) -> Matrix<M, N, T> {
        let mut result = Matrix::new();
        for (value, (row_l, row_r)) in zip(
            result.values.iter_mut(),
            zip(self.values.iter(), rhs.values.iter()),
        ) {
            *value = *row_l + *row_r;
        }
        result
    }
}

impl<const M: usize, const N: usize, T: Copy + Default + Sub<Output = T>> Sub<Matrix<M, N, T>>
    for Matrix<M, N, T>
{
    type Output = Matrix<M, N, T>;

    fn sub(self, rhs: Matrix<M, N, T>) -> Matrix<M, N, T> {
        let mut result = Matrix::new();
        for (value, (row_l, row_r)) in zip(
            result.values.iter_mut(),
            zip(self.values.iter(), rhs.values.iter()),
        ) {
            *value = *row_l - *row_r;
        }
        result
    }
}

impl<
    const M: usize,
    const N: usize,
    const P: usize,
    T: Copy + Default + Add<Output = T> + Mul<Output = T>,
> Mul<Matrix<N, P, T>> for Matrix<M, N, T>
{
    type Output = Matrix<M, P, T>;

    fn mul(self, rhs: Matrix<N, P, T>) -> Matrix<M, P, T> {
        let mut result = Matrix::new();
        for (value, row) in zip(result.values.iter_mut(), self.values.iter()) {
            *value = *row * rhs;
        }
        result
    }
}

impl<const M: usize, const N: usize, T: Copy + Default + Add<Output = T> + Mul<Output = T>>
    Mul<Vector<N, T>> for Matrix<M, N, T>
{
    type Output = Vector<M, T>;

    fn mul(self, rhs: Vector<N, T>) -> Vector<M, T> {
        let mut result = Vector::new();
        for (value, row) in zip(result.values.iter_mut(), self.values.iter()) {
            *value = *row * rhs;
        }
        result
    }
}

impl<const M: usize, const N: usize, T: Copy + Default + Add<Output = T> + Mul<Output = T>>
    Mul<Matrix<M, N, T>> for Vector<M, T>
{
    type Output = Vector<N, T>;

    fn mul(self, rhs: Matrix<M, N, T>) -> Vector<N, T> {
        let mut result = Vector::new();
        for (index, value) in result.values.iter_mut().enumerate() {
            *value = self * rhs.column(index);
        }
        result
    }
}

impl<const N: usize, T: Copy + Default + Add<Output = T> + Sub<Output = T> + Mul<Output = T>>
    Matrix<N, N, T>
{
    pub fn determinant(self) -> T {
        let mut result = T::default();
        for column in 0..N {
            let mut product_add = self.values[0].values[column];
            let mut product_sub = self.values[0].values[column];
            for row in 1..N {
                product_add = product_add * self.values[row].values[(column + row) % N];
                product_sub = product_sub * self.values[row].values[(N + column - row) % N];
            }
            result = result + product_add - product_sub;
        }
        result
    }
}
