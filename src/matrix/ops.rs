use super::{Dim, Matrix};
use std::{iter::Sum, ops::{Index, IndexMut, Mul}};

// Index

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

// Multiply

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

impl<T, MA, N, NB> Mul<&Matrix<T, N, NB>> for Matrix<T, MA, N>
where
	T: Mul + Sum<<T as Mul>::Output> + Copy,
	MA: Dim,
	N: Dim,
	NB: Dim,
{
	type Output = Matrix<T, MA, NB>;

	fn mul(self, rhs: &Matrix<T, N, NB>) -> Self::Output {
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

impl<T, MA, N, NB> Mul<Matrix<T, N, NB>> for &Matrix<T, MA, N>
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

impl<T, MA, N, NB> Mul<&Matrix<T, N, NB>> for &Matrix<T, MA, N>
where
	T: Mul + Sum<<T as Mul>::Output> + Copy,
	MA: Dim,
	N: Dim,
	NB: Dim,
{
	type Output = Matrix<T, MA, NB>;

	fn mul(self, rhs: &Matrix<T, N, NB>) -> Self::Output {
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
