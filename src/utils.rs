use std::convert::TryInto;

/// Clamp function, see https://github.com/rust-lang/rust/issues/44095
/// Unstable as of writing this (23.08.19)
pub fn clamp<N: PartialOrd>(a: N, min: N, max: N) -> N {
	assert!(min <= max);
	let mut x = a;
	if x < min {
		x = min;
	} else if x > max {
		x = max;
	}
	x
}

/// Split lines that are over `max_length' long into multiple lines.
/// Breakes only at whitespace.
pub fn split_long_lines(max_length: usize, s: &str) -> Vec<String> {
	let mut length = 0;
	let mut line_buf = vec![];
	let mut buf = s.split_whitespace().fold(vec![], |mut buf, segment| {
		if length + segment.len() < max_length {
			length += segment.len() + 1;
			line_buf.push(segment);
		} else {
			buf.push(line_buf.join(" "));
			length = 0;
			line_buf.clear();
			length += segment.len();
			line_buf.push(segment);
		}
		buf
	});
	buf.push(line_buf.join(" "));
	buf
}

/// Clamp the value to the range from 0.0 to 1.0 and then map that range onto 0 to max
pub fn clamp_and_normalize(num: f64, max: usize) -> usize {
	((clamp(num, 0.0, 1.0) * max as f64).round() as i64)
		.try_into()
		.unwrap()
}

/// Trait that provides equivalence for floating-point based types
pub trait ApproxEq<Rhs = Self> {
	/// Maximum allowed error such that two instances are regarded as being equal.
	fn approx_eq(self, other: Rhs) -> bool;
}

pub const EPSILON_F64: f64 = 0.1e-4;
impl ApproxEq for f64 {
	fn approx_eq(self, other: Self) -> bool {
		(self - other).abs() < EPSILON_F64
	}
}

pub const EPSILON_F32: f32 = 0.1e-4;
impl ApproxEq for f32 {
	fn approx_eq(self, other: Self) -> bool {
		(self - other).abs() < EPSILON_F32
	}
}

/// Adaption of assert_eq from the stdlib to work with assert_eq rather than std::ops::Eq::eq
#[macro_export]
macro_rules! assert_approx_eq {
    ($left:expr, $right:expr) => ({
        use $crate::utils::ApproxEq;
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !((left_val).approx_eq(*right_val)) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left ≈ right)`
  left: `{:?}`,
 right: `{:?}`"#, &*left_val, &*right_val)
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        use $crate::utils::ApproxEq;
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !((left_val).approx_eq(*right_val)) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left ≈ right)`
  left: `{:?}`,
 right: `{:?}`: {}"#, &*left_val, &*right_val,
                           format_args!($($arg)+))
                }
            }
        }
    });
}
