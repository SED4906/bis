use std::{
    fmt::{self, Debug, Display, Formatter},
    ops::{Add, Index, IndexMut, Mul, Sub},
};

use super::vector::Vector;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Matrix<const M: usize, const N: usize, T> {
    pub values: [[T; N]; M],
}

impl<const M: usize, const N: usize, T> Matrix<M, N, T> {
    pub const fn new(values: [[T; N]; M]) -> Self {
        Self { values }
    }
}

impl<const M: usize, const N: usize, T: Copy> Matrix<M, N, T> {
    pub const fn fill(value: T) -> Self {
        Self {
            values: [[value; N]; M],
        }
    }
}

impl<const M: usize, const N: usize, T: Copy> Matrix<M, N, T> {
    pub fn column(self, index: usize) -> Vector<M, T> {
        self.values.map(|row| row[index]).into()
    }
}

impl<const M: usize, const N: usize, T> Index<(usize, usize)> for Matrix<M, N, T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &T {
        &self.values[index.0][index.1]
    }
}

impl<const M: usize, const N: usize, T> Index<usize> for Matrix<M, N, T> {
    type Output = [T; N];

    fn index(&self, index: usize) -> &[T; N] {
        &self.values[index]
    }
}

impl<const M: usize, const N: usize, T> IndexMut<(usize, usize)> for Matrix<M, N, T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        &mut self.values[index.0][index.1]
    }
}

impl<const M: usize, const N: usize, T> IndexMut<usize> for Matrix<M, N, T> {
    fn index_mut(&mut self, index: usize) -> &mut [T; N] {
        &mut self.values[index]
    }
}

impl<const M: usize, const N: usize, T: Copy + Default> Default for Matrix<M, N, T> {
    fn default() -> Self {
        Self::fill(T::default())
    }
}

impl<const M: usize, const N: usize, T> From<[[T; N]; M]> for Matrix<M, N, T> {
    fn from(values: [[T; N]; M]) -> Self {
        Self { values }
    }
}

impl<const M: usize, const N: usize, T: Display> Display for Matrix<M, N, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("[[\n")?;
        for value in &self.values {
            f.write_str("[ ")?;
            for value in value {
                f.write_fmt(format_args!("{} ", value))?;
            }
            f.write_str("]\n")?;
        }
        f.write_str("]]")?;
        Ok(())
    }
}

impl<const M: usize, const N: usize, T: Debug> Debug for Matrix<M, N, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("[[\n")?;
        for value in &self.values {
            f.write_str("[ ")?;
            for value in value {
                f.write_fmt(format_args!("{:?} ", value))?;
            }
            f.write_str("]\n")?;
        }
        f.write_str("]]")?;
        Ok(())
    }
}

impl<const M: usize, const N: usize, T: Copy + Add<Output = T>> Add<Matrix<M, N, T>>
    for Matrix<M, N, T>
{
    type Output = Matrix<M, N, T>;

    fn add(self, rhs: Matrix<M, N, T>) -> Matrix<M, N, T> {
        let mut m = 0;
        self.values
            .map(|row| {
                let mut n = 0;
                let result = row.map(|lhs| {
                    let result = lhs + rhs[(m, n)];
                    n += 1;
                    result
                });
                m += 1;
                result
            })
            .into()
    }
}

impl<const M: usize, const N: usize, T: Copy + Sub<Output = T>> Sub<Matrix<M, N, T>>
    for Matrix<M, N, T>
{
    type Output = Matrix<M, N, T>;

    fn sub(self, rhs: Matrix<M, N, T>) -> Matrix<M, N, T> {
        let mut m = 0;
        self.values
            .map(|row| {
                let mut n = 0;
                let result = row.map(|lhs| {
                    let result = lhs - rhs[(m, n)];
                    n += 1;
                    result
                });
                m += 1;
                result
            })
            .into()
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
        self.values
            .map(|row| (Vector::from(row) * rhs).values)
            .into()
    }
}

impl<const M: usize, const N: usize, T: Copy + Default + Add<Output = T> + Mul<Output = T>>
    Mul<Vector<N, T>> for Matrix<M, N, T>
{
    type Output = Vector<M, T>;

    fn mul(self, rhs: Vector<N, T>) -> Vector<M, T> {
        self.values.map(|row| Vector::from(row) * rhs).into()
    }
}

impl<const M: usize, const N: usize, T: Copy + Default + Add<Output = T> + Mul<Output = T>>
    Mul<Matrix<M, N, T>> for Vector<M, T>
{
    type Output = Vector<N, T>;

    fn mul(self, rhs: Matrix<M, N, T>) -> Vector<N, T> {
        let mut index = 0;
        [T::default(); N]
            .map(|_| {
                let result = self * rhs.column(index);
                index += 1;
                result
            })
            .into()
    }
}

impl<const N: usize, T: Copy + Default + Add<Output = T> + Sub<Output = T> + Mul<Output = T>>
    Matrix<N, N, T>
{
    pub fn determinant(self) -> T {
        let mut result = T::default();
        for column in 0..N {
            let mut product_add = self[(0, column)];
            let mut product_sub = self[(0, column)];
            for row in 1..N {
                product_add = product_add * self[(row, (column + row) % N)];
                product_sub = product_sub * self[(row, (N + column - row) % N)];
            }
            result = result + product_add - product_sub;
        }
        result
    }
}
