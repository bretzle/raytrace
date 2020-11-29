use super::{Dim, Matrix};

impl<T, M, N> Matrix<T, M, N>
where
	M: Dim,
	N: Dim,
{
	/// Iterate over all elements
	pub fn iter(&self) -> impl Iterator<Item = &T> {
		self.data.iter()
	}

	/// Iterate over all elements mutably
	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
		self.data.iter_mut()
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

	/// Iterate over 3-tuples (i, j, self[(i,j)])
	pub fn iter_indexed(&self) -> impl Iterator<Item = (usize, usize, &T)> {
		self.iter_rows()
			.enumerate()
			.map(|(i, iter)| iter.enumerate().map(move |(j, x)| (i, j, x)))
			.flatten()
	}
}
