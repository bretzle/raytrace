use super::{Dim, Matrix, M1, M4};
use crate::tuple::Tuple;
use std::marker::PhantomData;

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
