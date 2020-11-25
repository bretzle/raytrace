use color::write_color;
use ray::Ray;
use vec3::{Color, Point3, Vec3};

mod color;
mod ray;
mod vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

fn main() {
	// Camera

	let viewport_height = 2.0;
	let viewport_width = ASPECT_RATIO * viewport_height;
	let focal_length = 1.0;

	let origin = Point3::default();
	let horizontal = Vec3::from(viewport_width, 0.0, 0.0);
	let vertical = Vec3::from(0.0, viewport_height, 0.0);
	let lower_left_corner =
		origin - (horizontal / 2.) - (vertical / 2.) - Vec3::from(0.0, 0.0, focal_length);

	// Render

	println!("P3");
	println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
	println!("255");

	for j in (0..IMAGE_HEIGHT).rev() {
		eprintln!("\rScanlines remaining: {}", j);
		for i in 0..IMAGE_WIDTH {
			let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
			let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;

			let ray = Ray::new(
				origin,
				lower_left_corner + (u * horizontal) + (v * vertical) - origin,
			);
			let pixel_color = ray_color(&ray);

			write_color(pixel_color);
		}
	}

	eprintln!("\nDone.");
}

fn ray_color(r: &Ray) -> Color {
	let unit_direction = Vec3::unit(r.direction());
	let t = 0.5 * (unit_direction.y() + 1.0);
	(Color::from(1.0, 1.0, 1.0) * (1.0 - t)) + (Color::from(0.5, 0.7, 1.0) * t)
}
