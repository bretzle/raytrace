use raytrace::matrix::*;
use raytrace::{matrix, tuple::Tuple};

#[test]
fn create_matrix4x4() {
	let mat = matrix![ M4, M4 =>
		 1.0,  2.0,  3.0,  4.0;
		 5.5,  6.5,  7.5,  8.5;
		 9.0, 10.0, 11.0, 12.0;
		13.5, 14.5, 15.5, 16.5;
	];

	assert_eq!(mat[(0, 0)], 1.0);
	assert_eq!(mat[(0, 3)], 4.0);
	assert_eq!(mat[(1, 0)], 5.5);
	assert_eq!(mat[(1, 2)], 7.5);
	assert_eq!(mat[(2, 2)], 11.0);
	assert_eq!(mat[(3, 0)], 13.5);
	assert_eq!(mat[(3, 2)], 15.5);
}

#[test]
fn create_matrix3x3() {
	let mat = matrix![ M3, M3 =>
		-3,  5,  0;
		 1, -2, -7;
		 0,  1,  1;
	];

	assert_eq!(mat[(0, 0)], -3);
	assert_eq!(mat[(1, 1)], -2);
	assert_eq!(mat[(2, 2)], 1);
}

#[test]
fn create_matrix2x2() {
	let mat = matrix![ M2, M2 =>
		-3,  5;
		 1, -2;
	];

	assert_eq!(mat[(0, 0)], -3);
	assert_eq!(mat[(0, 1)], 5);
	assert_eq!(mat[(1, 0)], 1);
	assert_eq!(mat[(1, 1)], -2);
}

#[test]
fn matrix_equality() {
	let a = matrix![ M4, M4 =>
		1,2,3,4;
		5,6,7,8;
		9,8,7,6;
		5,4,3,2;
	];
	let b = matrix![ M4, M4 =>
		1,2,3,4;
		5,6,7,8;
		9,8,7,6;
		5,4,3,2;
	];
	let c = matrix![ M4, M4 =>
		2,3,4,5;
		6,7,8,9;
		8,7,6,5;
		4,3,2,1;
	];

	assert_eq!(a, b);
	assert_ne!(a, c);
}

#[test]
fn multiply_matrix_by_matrix() {
	let a = matrix![ M4, M4 =>
		1, 2, 3, 4;
		5, 6, 7, 8;
		9, 8, 7, 6;
		5, 4, 3, 2;
	];
	let b = matrix![ M4, M4 =>
		-2, 1, 2,  3;
		 3, 2, 1, -1;
		 4, 3, 6,  5;
		 1, 2, 7,  8;
	];
	let c = matrix![ M4, M4 =>
		20, 22,  50,  48;
		44, 54, 114, 108;
		40, 58, 110, 102;
		16, 26,  46,  42;
	];

	assert_eq!(a * b, c);
}

#[test]
fn multiply_matrix_by_tuple() {
	let a = matrix![ M4, M4 =>
		1., 2., 3., 4.;
		2., 4., 4., 2.;
		8., 6., 4., 1.;
		0., 0., 0., 1f64;
	];
	let b = Tuple::new(1., 2., 3., 1.).to_matrix();
	let actual = a * b;
	let expected = Tuple::new(18., 24., 33., 1.);

	assert_eq!(expected, actual.to_tuple());
}

#[test]
fn multiply_matrix_by_identity() {
	let a: Matrix4x4<i32> = matrix![ M4, M4 =>
		0, 1, 2, 4;
		1, 2, 4, 8;
		2, 4, 8, 16;
		4, 8, 16, 32;
	];
	let actual = a.clone() * Matrix::identity();

	assert_eq!(a, actual);
}

#[test]
fn multiply_identity_by_tuple() {
	let a = Tuple::new(1., 2., 3., 4.);
	let actual = a.to_matrix() * Matrix::identity();

	assert_eq!(actual, a.to_matrix());
}

#[test]
fn transpose_matrix() {
	let a = matrix![ M4, M4 =>
		0, 9, 3, 0;
		9, 8, 0, 8;
		1, 8, 5, 3;
		0, 0, 5, 8;
	];
	let expected = matrix![ M4, M4 =>
		0, 9, 1, 0;
		9, 8, 8, 0;
		3, 0, 5, 5;
		0, 8, 3, 8;
	];

	assert_eq!(a.transpose(), expected);
}

#[test]
fn transpose_identity() {
	let a: Matrix4x4<i32> = Matrix::identity();
	let expected = a.clone();

	assert_eq!(a.transpose(), expected);
}

#[test]
fn det_2x2() {
	let a = matrix![ M2, M2 =>
		1, 5;
		-3, 2;
	];

	assert_eq!(a.det(), 17);
}

#[test]
fn submatrix() {
	let a = matrix![ M4, M4 =>
		-6, 1,  1, 6;
		-8, 5,  8, 6;
		-1, 0,  8, 2;
		-7, 1, -1, 1;
	];
	let expected = matrix! [ M3, M3 =>
		-6,  1, 6;
		-8,  8, 6;
		-7, -1, 1;
	];

	assert_eq!(a.submatrix(2, 1), expected);
}

#[test]
fn minor() {
	let a = matrix![ M3, M3 =>
		3,  5,  0;
		2, -1, -7;
		6, -1,  5;
	];

	assert_eq!(a.submatrix(1, 0).det(), 25);
	assert_eq!(a.minor(1, 0), 25);
}

#[test]
fn cofactor() {
	let a = matrix![ M3, M3 =>
		3,  5,  0;
		2, -1, -7;
		6, -1,  5;
	];

	assert_eq!(a.minor(0, 0), -12);
	assert_eq!(a.cofactor(0, 0), -12);
	assert_eq!(a.minor(1, 0), 25);
	assert_eq!(a.cofactor(1, 0), -25);
}

#[test]
fn det_3x3() {
	let a = matrix![ M3, M3 =>
		 1, 2,  6;
		-5, 8, -4;
		 2, 6,  4;
	];

	assert_eq!(a.cofactor(0, 0), 56);
	assert_eq!(a.cofactor(0, 1), 12);
	assert_eq!(a.cofactor(0, 2), -46);
	assert_eq!(a.det(), -196);
}

#[test]
fn det_4x4() {
	let a = matrix![ M4, M4 =>
		-2, -8,  3,  5;
		-3,  1,  7,  3;
		 1,  2, -9,  6;
		-6,  7,  7, -9;
	];

	assert_eq!(a.cofactor(0, 0), 690);
	assert_eq!(a.cofactor(0, 1), 447);
	assert_eq!(a.cofactor(0, 2), 210);
	assert_eq!(a.cofactor(0, 3), 51);
	assert_eq!(a.det(), -4071);
}