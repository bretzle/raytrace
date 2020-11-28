use raytrace::canvas::Canvas;
use raytrace::color::*;

#[test]
fn read_and_write() {
	let mut c = Canvas::new(2, 2);
	c[(0, 0)] = WHITE;
	c[(0, 1)] = GREEN;
	c[(1, 1)] = RED;
	assert_eq!(c.pixels[0], WHITE);
	assert_eq!(c.pixels[1], GREEN);
	assert_eq!(c.pixels[2], BLACK);
	assert_eq!(c.pixels[3], RED);
}

#[test]
fn ppm_header() {
	let c = Canvas::new(20, 10);
	assert_eq!(
		&c.as_ppm().lines().take(3).collect::<Vec<_>>().join("\n"),
		"P3\n20 10\n255"
	);
}

#[test]
fn iter_rows() {
	let mut c = Canvas::new(2, 2);
	c[(0, 0)] = WHITE;
	c[(0, 1)] = GREEN;
	c[(1, 1)] = RED;
	let mut i = c.iter_rows();
	for p in c.iter_rows() {
		for p in p {
			dbg!(p);
		}
	}
	let mut row1 = i.next().unwrap();
	let mut row2 = i.next().unwrap();
	assert!(i.next().is_none());
	assert_eq!(row1.next().unwrap(), &WHITE);
	assert_eq!(row1.next().unwrap(), &GREEN);
	assert_eq!(row2.next().unwrap(), &BLACK);
	assert_eq!(row2.next().unwrap(), &RED);
}

#[test]
fn ppm_data() {
	let mut c = Canvas::new(2, 2);
	c[(0, 0)] = WHITE * 0.5;
	c[(1, 1)] = RED;
	c[(0, 1)] = WHITE * 10.0;
	let s = c.as_ppm();
	assert_eq!(&s, "P3\n2 2\n255\n128 128 128 255 255 255\n0 0 0 255 0 0\n");
}

#[test]
fn end_in_newline() {
	let c = Canvas::new(5, 3);
	assert_eq!(c.as_ppm().chars().last(), Some('\n'));
}
