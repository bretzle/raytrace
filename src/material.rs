use crate::{
	objects::HitRecord,
	ray::Ray,
	utils::random,
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

pub struct Dielectric {
	ir: f64,
}

impl Dielectric {
	pub fn new(ir: f64) -> Self {
		Self { ir }
	}

	fn reflectance(cos: f64, ref_idx: f64) -> f64 {
		// Use Schlick's approximation for reflectance.
		let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
		r0 = r0 * r0;
		r0 + (1.0 - r0) * (1.0 - cos).powi(5)
	}
}

impl Material for Dielectric {
	fn scatter(
		&self,
		r_in: Ray,
		rec: HitRecord,
		attenuation: &mut Color,
		scattered: &mut Ray,
	) -> bool {
		*attenuation = Color::from(1.0, 1.0, 1.0);
		let refraction_ratio = if rec.front_face {
			1.0 / self.ir
		} else {
			self.ir
		};

		let unit_direction = Vec3::unit(r_in.direction());
		let cos_theta = f64::min(Vec3::dot(-unit_direction, rec.normal), 1.0);
		let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

		let cannot_refract = refraction_ratio * sin_theta > 1.0;
		let direction = if cannot_refract
			|| Self::reflectance(cos_theta, refraction_ratio) > random(0.0, 1.0)
		{
			Vec3::reflect(unit_direction, rec.normal)
		} else {
			Vec3::refract(unit_direction, rec.normal, refraction_ratio)
		};

		*scattered = Ray::new(rec.p, direction);

		true
	}
}
