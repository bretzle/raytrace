use std::{fmt, iter::Sum, marker::PhantomData, ops::Index, ops::IndexMut, ops::Mul};

use fmt::Debug;
use num_traits::{One, Zero};

use crate::tuple::Tuple;

pub trait Dim {
	const SIZE: usize;
}
#[derive(Debug, Copy, Clone, Default)]
pub struct M1;
#[derive(Debug, Copy, Clone, Default)]
pub struct M2;
#[derive(Debug, Copy, Clone, Default)]
pub struct M3;
#[derive(Debug, Copy, Clone, Default)]
pub struct M4;
impl Dim for M1 {
	const SIZE: usize = 1;
}

impl Dim for M2 {
	const SIZE: usize = 2;
}
impl Dim for M3 {
	const SIZE: usize = 3;
}
impl Dim for M4 {
	const SIZE: usize = 4;
}

pub type Matrix2x2<T> = Matrix<T, M2, M2>;
pub type Matrix3x3<T> = Matrix<T, M3, M3>;
pub type Matrix4x4<T> = Matrix<T, M4, M4>;

#[derive(Clone, PartialOrd)]
pub struct Matrix<T, M: Dim, N: Dim> {
	_m: PhantomData<M>,
	_n: PhantomData<N>,
	pub data: Vec<T>,
}

impl<T, M, N> Matrix<T, M, N>
where
	T: Default + Clone,
	M: Dim,
	N: Dim,
{
	pub fn new() -> Self {
		let data = vec![T::default(); M::SIZE * N::SIZE];
		Self {
			_m: PhantomData,
			_n: PhantomData,
			data,
		}
	}
}

impl<T, M, N> Matrix<T, M, N>
where
	M: Dim,
	N: Dim,
{
	pub fn new_uninitialized() -> Self {
		let data = Vec::with_capacity(M::SIZE * N::SIZE);
		Self {
			_m: PhantomData,
			_n: PhantomData,
			data,
		}
	}

	pub fn width(&self) -> usize {
		M::SIZE
	}

	pub fn height(&self) -> usize {
		N::SIZE
	}

	pub fn to_row_major(i: usize, j: usize) -> usize {
		N::SIZE * i + j
	}

	/// Iterate over all elements
	pub fn iter(&self) -> impl Iterator<Item = &T> {
		self.data.iter()
	}

	/// Iterate over the ith row of the matrix
	pub fn iter_row(&self, i: usize) -> impl Iterator<Item = &T> {
		self.iter().skip(i * N::SIZE).take(N::SIZE)
	}

	/// Iterate over the jth coloumn of the matrix
	pub fn iter_col(&self, j: usize) -> impl Iterator<Item = &T> {
		self.iter().skip(j).step_by(N::SIZE)
	}

	/// Iterate over all rows of the matrix
	pub fn iter_rows(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
		(0..M::SIZE).map(move |i| self.iter_row(i))
	}

	/// Iterate over all rows of the matrix
	pub fn iter_cols(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
		(0..N::SIZE).map(move |j| self.iter_col(j))
	}
}

impl<T, M, N> Matrix<T, M, N>
where
	T: Copy,
	M: Dim,
	N: Dim,
{
	pub fn transpose(&self) -> Matrix<T, N, M> {
		let mut v = Vec::with_capacity(M::SIZE * N::SIZE);
        for i in 0..M::SIZE {
            for j in 0..N::SIZE {
                v.push(self[(j, i)]);
            }
        }
        Matrix::from(v)
	}
}

impl<T, M> Matrix<T, M, M>
where
	T: One + Zero + Copy,
	M: Dim,
{
	pub fn identity() -> Self {
		let mut m = Matrix::new_uninitialized();
		m.data = vec![T::zero(); M::SIZE * M::SIZE];
		for i in 0..M::SIZE {
			m[(i, i)] = T::one();
		}
		m
	}
}

impl<T: Into<f64> + Copy> Matrix<T, M4, M1> {
	pub fn to_tuple(self) -> Tuple {
		Tuple::new(
			self.data[0].into(),
			self.data[1].into(),
			self.data[2].into(),
			self.data[3].into(),
		)
	}
}

impl<T, M, N> Debug for Matrix<T, M, N>
where
	T: Debug,
	M: Dim,
	N: Dim,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "Matrix {}x{}", M::SIZE, N::SIZE)?;

		for x in 0..M::SIZE {
			for y in 0..N::SIZE {
				write!(f, "{:?} ", self.data[x * N::SIZE + y])?;
			}
			writeln!(f, "")?;
		}

		Ok(())
	}
}

impl<T, M, N> From<Vec<T>> for Matrix<T, M, N>
where
	M: Dim,
	N: Dim,
{
	fn from(v: Vec<T>) -> Self {
		assert_eq!(v.len(), M::SIZE * N::SIZE);
		Matrix {
			data: v,
			_m: PhantomData,
			_n: PhantomData,
		}
	}
}

impl<T, M, N> Index<(usize, usize)> for Matrix<T, M, N>
where
	M: Dim,
	N: Dim,
{
	type Output = T;

	fn index(&self, coords: (usize, usize)) -> &Self::Output {
		let (i, j) = coords;
		&self.data[Self::to_row_major(i, j)]
	}
}

impl<T, M, N> IndexMut<(usize, usize)> for Matrix<T, M, N>
where
	M: Dim,
	N: Dim,
{
	fn index_mut(&mut self, coords: (usize, usize)) -> &mut Self::Output {
		let (i, j) = coords;
		&mut self.data[Self::to_row_major(i, j)]
	}
}

/// Create a new matrix of specified size
/// Verifies that the number of values matches the size
/// # Example
/// ```
/// use raytrace::matrix;
/// use raytrace::matrix::M3;
/// let a = matrix![ M3, M3 =>
///     1,  2, 3;
///     4, -5, 6;
///     7,  8, 9;
/// ];
/// ```
/// Creates a new 3x3 matrix
#[macro_export]
macro_rules! matrix {
	( $m:ty, $n:ty => $( $( $val:expr ),+ );* ; ) => {
        {
            type M = $m;
            type N = $n;
            use $crate::matrix::{Matrix, Dim};
            let data = vec![ $( vec![$($val),+] ),* ];
            assert_eq!(data.len(), M::SIZE);
            assert_eq!(data[0].len(), N::SIZE);
            let flattened = data.into_iter().map(|arr| arr.into_iter()).flatten().collect::<Vec<_>>();
            assert_eq!(flattened.len(), M::SIZE * N::SIZE);
            Matrix::<_, M, N>::from(flattened)
        }
	};
}

impl<T, M, N> PartialEq for Matrix<T, M, N>
where
	T: PartialEq,
	M: Dim,
	N: Dim,
{
	fn eq(&self, other: &Self) -> bool {
		self.data == other.data
	}
}

impl<T, MA, N, NB> Mul<Matrix<T, N, NB>> for Matrix<T, MA, N>
where
	T: Mul + Sum<<T as Mul>::Output> + Copy,
	MA: Dim,
	N: Dim,
	NB: Dim,
{
	type Output = Matrix<T, MA, NB>;

	fn mul(self, rhs: Matrix<T, N, NB>) -> Self::Output {
		let mut new = Matrix::new_uninitialized();
		for row in self.iter_rows() {
			let row = row.collect::<Vec<_>>();
			for col in rhs.iter_cols() {
				let e = row.iter().zip(col).map(|(r, c)| **r * *c).sum();
				new.data.push(e);
			}
		}
		new
	}
}
