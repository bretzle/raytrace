use std::rc::Rc;

use color::write_color;
use objects::{list::HittableList, sphere::Sphere, HitRecord, Hittable};
use ray::Ray;
use utils::*;
use vec3::{Color, Point3, Vec3};

mod color;
mod objects;
mod ray;
mod utils;
mod vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

fn main() {
	// World
	let mut world = HittableList::new();
	world.add(Rc::new(Sphere::new(Point3::from(0., 0., -1.), 0.5)));
	world.add(Rc::new(Sphere::new(Point3::from(0., -100.5, -1.), 100.)));

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
			let pixel_color = ray_color(ray, &world);

			write_color(pixel_color);
		}
	}

	eprintln!("\nDone.");
}

fn ray_color(r: Ray, world: &HittableList) -> Color {
	let mut rec = HitRecord::default();
	if world.hit(r, 0., INFINITY, &mut rec) {
		return 0.5 * (rec.normal + Color::from(1., 1., 1.));
	}
	let unit_direction = Vec3::unit(r.direction());
	let t = 0.5 * (unit_direction.y() + 1.0);
	return (1.0 - t) * Color::from(1.0, 1.0, 1.0) + t * Color::from(0.5, 0.7, 1.0);
}

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
	let oc = r.origin() - center;
	let a = r.direction().length_squared();
	let half_b = Vec3::dot(oc, r.direction());
	let c = oc.length_squared() - (radius * radius);
	let discriminant = half_b * half_b - a * c;

	if discriminant < 0.0 {
		-1.
	} else {
		(-half_b - discriminant.sqrt()) / a
	}
}
