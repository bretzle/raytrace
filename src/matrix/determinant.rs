use super::{Dim, Matrix, Sub, M2, M3, M4};
use num_traits::{Num, Signed};
use std::{iter::Sum, ops::DivAssign};

pub trait Determinant<T> {
	fn minor(&self, i: usize, j: usize) -> T;
	fn cofactor(&self, i: usize, j: usize) -> T;
	fn det(&self) -> T;
}

impl<T: Num + Copy> Determinant<T> for Matrix<T, M2, M2> {
	fn det(&self) -> T {
		match self.data[..] {
			[a, b, c, d] => a * d - b * c,
			_ => unreachable!(),
		}
	}

	fn minor(&self, _: usize, _: usize) -> T {
		unimplemented!()
	}

	fn cofactor(&self, _: usize, _: usize) -> T {
		unimplemented!()
	}
}

impl<T: Num + Copy + Signed + Sum> Determinant<T> for Matrix<T, M3, M3> {
	fn det(&self) -> T {
		(0..3).map(|j| self[(0, j)] * self.cofactor(0, j)).sum()
	}

	fn minor(&self, i: usize, j: usize) -> T {
		self.submatrix(i, j).det()
	}

	fn cofactor(&self, i: usize, j: usize) -> T {
		if (i + j) % 2 == 0 {
			self.minor(i, j)
		} else {
			-T::one() * self.minor(i, j)
		}
	}
}

impl<T: Num + Copy + Signed + Sum> Determinant<T> for Matrix<T, M4, M4> {
	fn minor(&self, i: usize, j: usize) -> T {
		self.submatrix(i, j).det()
	}

	fn cofactor(&self, i: usize, j: usize) -> T {
		if (i + j) % 2 == 0 {
			self.minor(i, j)
		} else {
			-T::one() * self.minor(i, j)
		}
	}

	fn det(&self) -> T {
		(0..4).map(|j| self[(0, j)] * self.cofactor(0, j)).sum()
	}
}

impl<T, M, N> Matrix<T, M, N>
where
	T: Copy,
	M: Dim + Sub,
	N: Dim + Sub,
	<M as Sub>::SUB: Dim,
	<N as Sub>::SUB: Dim,
{
	pub fn submatrix(&self, i: usize, j: usize) -> Matrix<T, M::SUB, N::SUB> {
		self.remove_row(i).remove_col(j)
	}
}

impl<T, M, N> Matrix<T, M, N>
where
	T: Copy,
	M: Dim + Sub,
	N: Dim,
	<M as Sub>::SUB: Dim,
{
	/// Remove the i'th row
	pub fn remove_row(&self, i: usize) -> Matrix<T, M::SUB, N> {
		let mut m = Matrix::new_uninitialized();
		m.data = self
			.iter_rows()
			.enumerate()
			.filter_map(|(x, row)| if x != i { Some(row) } else { None })
			.flatten()
			.copied()
			.collect::<Vec<T>>();
		m
	}
}

impl<T, M, N> Matrix<T, M, N>
where
	T: Copy,
	M: Dim,
	N: Dim + Sub,
	<N as Sub>::SUB: Dim,
{
	/// Remove the j'th col
	pub fn remove_col(&self, j: usize) -> Matrix<T, M, N::SUB> {
		let mut m = Matrix::new_uninitialized();
		m.data = self
			.iter_cols()
			.enumerate()
			.filter_map(|(x, col)| if x != j { Some(col) } else { None })
			.fold(None, |old: Option<Vec<Vec<_>>>, new| match old {
				// zip all the iterators
				// TODO Maybe test another datastructure here - VecDeque or LinkedList or smth
				// Could also transpose -> remove row -> transpose, if that is more efficient
				Some(it) => Some(
					it.into_iter()
						.zip(new)
						.map(|(mut v, elem)| {
							v.push(elem);
							v
						})
						.collect::<Vec<Vec<_>>>(),
				),
				None => Some(new.map(|x| vec![x]).collect::<Vec<_>>()),
			})
			.unwrap()
			.into_iter()
			.flatten()
			.copied()
			.collect::<Vec<T>>();
		m
	}
}

impl<T, M> Matrix<T, M, M>
where
	T: Num + Copy + DivAssign,
	M: Dim,
	Self: Determinant<T>,
{
	pub fn invert(&self) -> Option<Self> {
		let det = self.det();
		if det == T::zero() {
			None
		} else {
			let mut m = Self::new_uninitialized();
			m.data = self
				.iter_indexed()
				.map(|(i, j, _)| self.cofactor(i, j))
				.collect::<Vec<_>>();
			let mut m2 = m.transpose();
			let _ = m2.iter_mut().map(|x| *x /= det).collect::<Vec<_>>();
			Some(m2)
		}
	}
}
