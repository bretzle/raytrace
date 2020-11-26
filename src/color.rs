use crate::{utils::clamp, vec3::Color};

pub fn write_color(color: Color, samples_per_pixel: i32) {
	// let ir = (255.999 * color.x()) as i32;
	// let ig = (255.999 * color.y()) as i32;
	// let ib = (255.999 * color.z()) as i32;

	// println!("{} {} {}", ir, ig, ib);

	let mut r = color.x();
	let mut g = color.y();
	let mut b = color.z();

	// Divide the color by the number of samples
	let scale = 1.0 / samples_per_pixel as f64;
	r *= scale;
	g *= scale;
	b *= scale;

	println!(
		"{} {} {}",
		(256. * clamp(r, 0., 0.999)) as i32,
		(256. * clamp(g, 0., 0.999)) as i32,
		(256. * clamp(b, 0., 0.999)) as i32
	);
}
