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
