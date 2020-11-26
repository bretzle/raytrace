use std::rc::Rc;

use camera::Camera;
use color::write_color;
use material::{Dielectric, Lambertian, Material, Metal};
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

const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: i32 = 1200;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 50;
const MAX_DEPTH: i32 = 50;

fn main() {
	// World
	let world = random_scene();

	// Camera
	let lookfrom = Point3::from(13.0, 2.0, 3.0);
	let lookat = Point3::from(0.0, 0.0, 0.0);
	let vup = Vec3::from(0.0, 1.0, 0.0);
	let vfov = 20.0;
	let aperature = 0.1;

	let camera = Camera::new(
		lookfrom,
		lookat,
		vup,
		vfov,
		ASPECT_RATIO,
		aperature,
		(lookfrom - lookat).length(),
	);

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

fn random_scene() -> HittableList {
	let mut world = HittableList::new();

	let ground_material = Rc::new(Lambertian::new(Color::from(0.5, 0.5, 0.5)));
	world.add(Rc::new(Sphere::new(
		Point3::from(0.0, -1000.0, 0.0),
		1000.0,
		ground_material,
	)));

	for a in -11..11 {
		for b in -11..11 {
			let choose_mat = random(0., 1.);
			let center = Point3::from(
				a as f64 + 0.9 * random(0., 1.),
				0.2,
				b as f64 + 0.9 * random(0., 1.),
			);

			if (center - Point3::from(4., 0.2, 0.)).length() > 0.9 {
				let mut sphere_material: Rc<dyn Material> =
					Rc::new(Metal::new(Color::default(), 0.0));

				if choose_mat < 0.8 {
					// diffuse
					let albedo = Color::random() * Color::random();
					sphere_material = Rc::new(Lambertian::new(albedo));
					world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
				} else if choose_mat < 0.95 {
					// metal
					let albedo = Color::random_range(0.5, 1.);
					let fuzz = random(0., 0.5);
					sphere_material = Rc::new(Metal::new(albedo, fuzz));
					world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
				} else {
					// glass
					sphere_material = Rc::new(Dielectric::new(1.5));
					world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)))
				}
			}
		}
	}
	let material1 = Rc::new(Dielectric::new(1.5));
	world.add(Rc::new(Sphere::new(
		Point3::from(0., 1., 0.),
		1.,
		material1,
	)));

	let material2 = Rc::new(Lambertian::new(Color::from(0.4, 0.2, 0.1)));
	world.add(Rc::new(Sphere::new(
		Point3::from(-4., 1., 0.),
		1.,
		material2,
	)));

	let material3 = Rc::new(Metal::new(Color::from(0.7, 0.6, 0.5), 0.0));
	world.add(Rc::new(Sphere::new(
		Point3::from(4., 1., 0.),
		1.,
		material3,
	)));

	world
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
