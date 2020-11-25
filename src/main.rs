use color::write_color;
use vec3::Color;

mod color;
mod ray;
mod vec3;

const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;

fn main() {
	// Render

	println!("P3");
	println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
	println!("255");

	for j in (0..IMAGE_HEIGHT).rev() {
		eprintln!("\rScanlines remaining: {}", j);
		for i in 0..IMAGE_WIDTH {
			let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
			let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
			let b = 0.25;

			let pixel_color = Color::from(r, g, b);
			write_color(pixel_color);
		}
	}

	eprintln!("\nDone.");
}
