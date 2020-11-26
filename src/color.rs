use crate::{utils::clamp, vec3::Color};

pub fn write_color(color: Color, samples_per_pixel: i32) {
	let mut r = color.x();
	let mut g = color.y();
	let mut b = color.z();

	// Divide the color by the number of samples
	let scale = 1.0 / samples_per_pixel as f64;
	r = (scale * r).sqrt();
	g = (scale * g).sqrt();
	b = (scale * b).sqrt();

	println!(
		"{} {} {}",
		(256. * clamp(r, 0., 0.999)) as i32,
		(256. * clamp(g, 0., 0.999)) as i32,
		(256. * clamp(b, 0., 0.999)) as i32
	);
}
