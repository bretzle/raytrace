use crate::{
	objects::HitRecord,
	ray::Ray,
	vec3::{Color, Vec3},
};

pub trait Material {
	fn scatter(
		&self,
		r_in: Ray,
		rec: HitRecord,
		attenuation: &mut Color,
		scattered: &mut Ray,
	) -> bool;
}

pub struct Lambertian {
	albedo: Color,
}

impl Lambertian {
	pub fn new(color: Color) -> Self {
		Self { albedo: color }
	}
}

impl Material for Lambertian {
	fn scatter(
		&self,
		r_in: Ray,
		rec: HitRecord,
		attenuation: &mut Color,
		scattered: &mut Ray,
	) -> bool {
		let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

		if scatter_direction.near_zero() {
			scatter_direction = rec.normal;
		}

		*scattered = Ray::new(rec.p, scatter_direction);
		*attenuation = self.albedo;
		true
	}
}

pub struct Metal {
	albedo: Color,
	fuzz: f64,
}

impl Metal {
	pub fn new(color: Color, f: f64) -> Self {
		Self {
			albedo: color,
			fuzz: if f < 1.0 { f } else { 1.0 },
		}
	}
}

impl Material for Metal {
	fn scatter(
		&self,
		r_in: Ray,
		rec: HitRecord,
		attenuation: &mut Color,
		scattered: &mut Ray,
	) -> bool {
		let reflected = Vec3::reflect(Vec3::unit(r_in.direction()), rec.normal);
		*scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
		*attenuation = self.albedo;
		Vec3::dot(scattered.direction(), rec.normal) > 0.0
	}
}
