use std::{
    fmt::{self, Debug, Display, Formatter},
    iter::zip,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Vector<const N: usize, T> {
    pub values: [T; N],
}

pub type Scalar<T> = Vector<1, T>;

impl<const N: usize, T: Copy + Default> Vector<N, T> {
    pub const fn new(values: [T; N]) -> Self {
        Vector { values }
    }
}

impl<const N: usize, T: Copy + Default> Default for Vector<N, T> {
    fn default() -> Self {
        Self {
            values: [T::default(); N],
        }
    }
}

impl<const N: usize, T: Copy + Default> From<[T; N]> for Vector<N, T> {
    fn from(values: [T; N]) -> Self {
        Self { values }
    }
}

impl<const N: usize, T: Copy + Add<Output = T> + Mul<Output = T> + Default> Vector<N, T> {
    pub fn mag2(&self) -> T {
        let mut result = T::default();
        for value in &self.values {
            result = result + *value * *value;
        }
        result
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
        self.values[0]
    }
}

impl<const N: usize, T: Copy> Y<T> for Vector<N, T> {
    const VALID: () = if N < 2 {
        panic!("Vector has too few elements")
    };
    fn y(&self) -> T {
        let _ = <Self as Y<T>>::VALID;
        self.values[1]
    }
}

impl<const N: usize, T: Copy> Z<T> for Vector<N, T> {
    const VALID: () = if N < 3 {
        panic!("Vector has too few elements")
    };
    fn z(&self) -> T {
        let _ = <Self as Z<T>>::VALID;
        self.values[2]
    }
}

impl<const N: usize, T: Copy> W<T> for Vector<N, T> {
    const VALID: () = if N < 4 {
        panic!("Vector has too few elements")
    };
    fn w(&self) -> T {
        let _ = <Self as W<T>>::VALID;
        self.values[3]
    }
}

impl<T> Scalar<T> {
    pub fn scalar(value: T) -> Self {
        Vector { values: [value; 1] }
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

impl<const N: usize, T: Add<Output = T> + Copy + Default> Add for Vector<N, T> {
    type Output = Vector<N, T>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::Output::default();
        for (i, (left, right)) in zip(self.values, rhs.values).enumerate() {
            result.values[i] = left + right;
        }
        result
    }
}

impl<const N: usize, T: Add<Output = T> + Copy + Default> AddAssign for Vector<N, T> {
    fn add_assign(&mut self, other: Self) {
        for (left, right) in zip(self.values.iter_mut(), other.values) {
            *left = *left + right;
        }
    }
}

impl<const N: usize, T: Sub<Output = T> + Copy + Default> Sub for Vector<N, T> {
    type Output = Vector<N, T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Self::Output::default();
        for (i, (left, right)) in zip(self.values, rhs.values).enumerate() {
            result.values[i] = left - right;
        }
        result
    }
}

impl<const N: usize, T: Sub<Output = T> + Copy + Default> SubAssign for Vector<N, T> {
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
        let mut result = Self::Output::default();
        for (left, right) in zip(self.values, rhs.values) {
            result = result + left * right;
        }
        result
    }
}

impl<T: Sub<Output = T> + Mul<Output = T> + Copy + Default> Vector<3, T> {
    /// Cross product
    pub fn cross(self, rhs: Self) -> Self {
        let mut result = Self::default();
        result.values[0] = self.values[1] * rhs.values[2] - self.values[2] * rhs.values[1];
        result.values[1] = self.values[2] * rhs.values[0] - self.values[0] * rhs.values[2];
        result.values[2] = self.values[0] * rhs.values[1] - self.values[1] * rhs.values[0];
        result
    }
}

impl<const N: usize, T: Mul<Output = T> + Copy + Default> Mul<T> for Vector<N, T> {
    type Output = Vector<N, T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut result = Self::Output::default();
        for (i, value) in self.values.iter().enumerate() {
            result.values[i] = *value * rhs;
        }
        result
    }
}

impl<const N: usize, T: Mul<Output = T> + Copy + Default> MulAssign<T> for Vector<N, T> {
    fn mul_assign(&mut self, rhs: T) {
        for value in self.values.iter_mut() {
            *value = *value * rhs;
        }
    }
}

impl<const N: usize, T: Neg<Output = T> + Copy + Default> Neg for Vector<N, T> {
    type Output = Vector<N, T>;

    fn neg(self) -> Self::Output {
        Self {
            values: self.values.map(|s| -s),
        }
    }
}
