use std::{
    fmt::{self, Debug, Display, Formatter},
    iter::zip,
    ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Vector<const N: usize, T> {
    pub values: [T; N],
}

pub type Scalar<T> = Vector<1, T>;

impl<const N: usize, T> Vector<N, T> {
    pub const fn new(values: [T; N]) -> Self {
        Self { values }
    }
}

impl<const N: usize, T: Copy> Vector<N, T> {
    pub const fn fill(value: T) -> Self {
        Self { values: [value; N] }
    }
}

impl<const N: usize, T: Copy + Default> Default for Vector<N, T> {
    fn default() -> Self {
        Self::fill(T::default())
    }
}

impl<const N: usize, T> From<[T; N]> for Vector<N, T> {
    fn from(values: [T; N]) -> Self {
        Self { values }
    }
}

impl<const N: usize, T> Index<usize> for Vector<N, T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.values[index]
    }
}

impl<const N: usize, T> IndexMut<usize> for Vector<N, T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.values[index]
    }
}

impl<const N: usize, T: Copy + Default + Add<Output = T> + Mul<Output = T>> Vector<N, T> {
    pub fn mag2(&self) -> T {
        self.values
            .iter()
            .fold(T::default(), |acc, v| acc + *v * *v)
    }
}

impl<const N: usize> Vector<N, f32> {
    pub fn mag(&self) -> f32 {
        self.mag2().sqrt()
    }
}

impl<const N: usize> Vector<N, f64> {
    pub fn mag(&self) -> f64 {
        self.mag2().sqrt()
    }
}

pub trait X<T> {
    const VALID: ();
    fn x(&self) -> T;
}

pub trait Y<T> {
    const VALID: ();
    fn y(&self) -> T;
}

pub trait Z<T> {
    const VALID: ();
    fn z(&self) -> T;
}

pub trait W<T> {
    const VALID: ();
    fn w(&self) -> T;
}

impl<const N: usize, T: Copy> X<T> for Vector<N, T> {
    const VALID: () = if N < 1 {
        panic!("Vector has too few elements")
    };
    fn x(&self) -> T {
        let _ = <Self as X<T>>::VALID;
        self[0]
    }
}

impl<const N: usize, T: Copy> Y<T> for Vector<N, T> {
    const VALID: () = if N < 2 {
        panic!("Vector has too few elements")
    };
    fn y(&self) -> T {
        let _ = <Self as Y<T>>::VALID;
        self[1]
    }
}

impl<const N: usize, T: Copy> Z<T> for Vector<N, T> {
    const VALID: () = if N < 3 {
        panic!("Vector has too few elements")
    };
    fn z(&self) -> T {
        let _ = <Self as Z<T>>::VALID;
        self[2]
    }
}

impl<const N: usize, T: Copy> W<T> for Vector<N, T> {
    const VALID: () = if N < 4 {
        panic!("Vector has too few elements")
    };
    fn w(&self) -> T {
        let _ = <Self as W<T>>::VALID;
        self[3]
    }
}

impl<T> Scalar<T> {
    pub fn scalar(value: T) -> Self {
        [value].into()
    }
}

impl<const N: usize, T: Display> Display for Vector<N, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("[ ")?;
        for value in &self.values {
            f.write_fmt(format_args!("{} ", value))?;
        }
        f.write_str("]")?;
        Ok(())
    }
}

impl<const N: usize, T: Debug> Debug for Vector<N, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("[ ")?;
        for value in &self.values {
            f.write_fmt(format_args!("{:?} ", value))?;
        }
        f.write_str("]")?;
        Ok(())
    }
}

impl<const N: usize, T: Add<Output = T> + Copy> Add for Vector<N, T> {
    type Output = Vector<N, T>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut n = 0;
        self.values
            .map(|lhs| {
                let result = lhs + rhs[n];
                n += 1;
                result
            })
            .into()
    }
}

impl<const N: usize, T: Add<Output = T> + Copy> AddAssign for Vector<N, T> {
    fn add_assign(&mut self, other: Self) {
        for (left, right) in zip(self.values.iter_mut(), other.values) {
            *left = *left + right;
        }
    }
}

impl<const N: usize, T: Sub<Output = T> + Copy> Sub for Vector<N, T> {
    type Output = Vector<N, T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut n = 0;
        self.values
            .map(|lhs| {
                let result = lhs - rhs[n];
                n += 1;
                result
            })
            .into()
    }
}

impl<const N: usize, T: Sub<Output = T> + Copy> SubAssign for Vector<N, T> {
    fn sub_assign(&mut self, other: Self) {
        for (left, right) in zip(self.values.iter_mut(), other.values) {
            *left = *left - right;
        }
    }
}

impl<const N: usize, T: Add<Output = T> + Mul<Output = T> + Default> Mul for Vector<N, T> {
    type Output = T;

    /// Dot product
    fn mul(self, rhs: Self) -> Self::Output {
        zip(self.values, rhs.values).fold(T::default(), |acc, (l, r)| acc + l * r)
    }
}

impl<T: Sub<Output = T> + Mul<Output = T> + Copy> Vector<3, T> {
    /// Cross product
    pub fn cross(self, rhs: Self) -> Self {
        [
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        ]
        .into()
    }
}

impl<const N: usize, T: Mul<Output = T> + Copy> Mul<T> for Vector<N, T> {
    type Output = Vector<N, T>;

    fn mul(self, rhs: T) -> Self::Output {
        self.values.map(|lhs| lhs * rhs).into()
    }
}

impl<const N: usize, T: Mul<Output = T> + Copy> MulAssign<T> for Vector<N, T> {
    fn mul_assign(&mut self, rhs: T) {
        for value in self.values.iter_mut() {
            *value = *value * rhs;
        }
    }
}

impl<const N: usize, T: Neg<Output = T>> Neg for Vector<N, T> {
    type Output = Vector<N, T>;

    fn neg(self) -> Self::Output {
        self.values.map(|s| -s).into()
    }
}
