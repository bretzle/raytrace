use raytrace::matrix;
use raytrace::matrix::*;

fn main() {
	// let mat: Matrix<f64, M4, M2> = Matrix::new();
	let mat = matrix![ M3, M3 =>
		1,  2, 3;
		4, -5, 6;
		7,  8, 9;
	];

	println!("{:?}", mat);
}
