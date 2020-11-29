use crate::utils::ApproxEq;
use fmt::Debug;
use num_traits::{One, Zero};
use std::{fmt, marker::PhantomData};

mod convert;
mod determinant;
mod iter;
mod macros;
mod ops;
mod typestate;

pub use determinant::*;
pub use macros::*;
pub use typestate::*;

pub type Matrix2x2<T> = Matrix<T, M2, M2>;
pub type Matrix3x3<T> = Matrix<T, M3, M3>;
pub type Matrix4x4<T> = Matrix<T, M4, M4>;

#[derive(Clone, PartialEq)]
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

impl<T, M, N> ApproxEq for &Matrix<T, M, N>
where
	T: ApproxEq + Copy,
	M: Dim,
	N: Dim,
{
	fn approx_eq(self, other: Self) -> bool {
		self.iter().zip(other.iter()).all(|(l, r)| l.approx_eq(*r))
	}
}
