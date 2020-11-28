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
