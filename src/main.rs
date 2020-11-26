use std::rc::Rc;

use camera::Camera;
use color::write_color;
use material::{Dielectric, Lambertian, Metal};
use objects::{list::HittableList, sphere::Sphere, HitRecord, Hittable};
use ray::Ray;
use utils::*;
use vec3::{Color, Point3, Vec3};

mod camera;
mod color;
mod material;
mod objects;
mod ray;
mod utils;
mod vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

fn main() {
	// World
	let mut world = HittableList::new();

	let material_ground = Rc::new(Lambertian::new(Color::from(0.8, 0.8, 0.0)));
	let material_center = Rc::new(Lambertian::new(Color::from(0.1, 0.2, 0.5)));
	let material_left = Rc::new(Dielectric::new(1.5));
	let material_right = Rc::new(Metal::new(Color::from(0.8, 0.6, 0.2), 1.0));

	world.add(Rc::new(Sphere::new(
		Point3::from(0.0, -100.5, -1.0),
		100.0,
		material_ground,
	)));
	world.add(Rc::new(Sphere::new(
		Point3::from(0.0, 0.0, -1.0),
		0.5,
		material_center,
	)));
	world.add(Rc::new(Sphere::new(
		Point3::from(-1.0, 0.0, -1.0),
		0.5,
		material_left.clone(),
	)));
	world.add(Rc::new(Sphere::new(
		Point3::from(-1.0, 0.0, -1.0),
		-0.4,
		material_left,
	)));
	world.add(Rc::new(Sphere::new(
		Point3::from(1.0, 0.0, -1.0),
		0.5,
		material_right,
	)));

	// Camera
	let camera = Camera::new();

	// Render
	println!("P3");
	println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
	println!("255");

	for j in (0..IMAGE_HEIGHT).rev() {
		eprintln!("\rScanlines remaining: {}", j);
		for i in 0..IMAGE_WIDTH {
			let mut pixel_color = Color::default();
			for _ in 0..SAMPLES_PER_PIXEL {
				let u = (i as f64 + random(0.0, 1.0)) / (IMAGE_WIDTH as f64 - 1.0);
				let v = (j as f64 + random(0.0, 1.0)) / (IMAGE_HEIGHT as f64 - 1.0);
				let ray = camera.get_ray(u, v);
				pixel_color += ray_color(ray, &world, MAX_DEPTH);
			}
			write_color(pixel_color, SAMPLES_PER_PIXEL)
		}
	}

	eprintln!("\nDone.");
}

fn ray_color(r: Ray, world: &HittableList, depth: i32) -> Color {
	let mut rec = HitRecord::default();

	if depth <= 0 {
		return Color::default();
	}

	if world.hit(r, 0.001, INFINITY, &mut rec) {
		let mut scattered = Ray::new(Point3::default(), Vec3::default());
		let mut attenuation = Color::default();
		if rec
			.mat_ptr
			.scatter(r, rec.clone(), &mut attenuation, &mut scattered)
		{
			return attenuation * ray_color(scattered, world, depth - 1);
		}
		return Color::from(0.0, 0.0, 0.0);
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
