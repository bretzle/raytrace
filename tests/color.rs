use raytrace::color::Color;

#[test]
fn colors_are_rgb_tuples() {
	let c = Color::new(-0.5, 0.4, 1.7);

	assert_eq!(c.r, -0.5);
	assert_eq!(c.g, 0.4);
	assert_eq!(c.b, 1.7);
}

#[test]
fn adding_colors() {
	let c1 = Color::new(0.9, 0.6, 0.75);
	let c2 = Color::new(0.7, 0.1, 0.25);

	let actual = c1 + c2;
	let expected = Color::new(1.6, 0.7, 1.);

	assert_eq!(expected, actual);
}

#[test]
fn subtracting_colors() {
	let c1 = Color::new(0.9, 0.6, 0.75);
	let c2 = Color::new(0.7, 0.1, 0.25);

	let actual = c1 - c2;
	let expected = Color::new(0.2, 0.5, 0.5);

	assert_eq!(expected, actual);
}

#[test]
fn multiplying_color_by_scalar() {
	let c = Color::new(0.2, 0.3, 0.4);

	let actual = c * 2.0;
	let expected = Color::new(0.4, 0.6, 0.8);

	assert_eq!(expected, actual);
}

#[test]
fn multiplying_colors() {
	let c1 = Color::new(1., 0.2, 0.4);
	let c2 = Color::new(0.9, 1., 0.1);

	let actual = c1 * c2;
	let expected = Color::new(0.9, 0.2, 0.04);

	assert_eq!(expected, actual);
}
