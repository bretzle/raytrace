pub mod list;
pub mod sphere;

use std::rc::Rc;

use crate::{
	material::Lambertian,
	material::Material,
	ray::Ray,
	vec3::Color,
	vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub struct HitRecord {
	pub p: Point3,
	pub normal: Vec3,
	pub mat_ptr: Rc<dyn Material>,
	pub t: f64,
	pub front_face: bool,
}

impl HitRecord {
	fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
		self.front_face = Vec3::dot(r.direction(), outward_normal) < 0.0;
		self.normal = if self.front_face {
			outward_normal
		} else {
			-outward_normal
		};
	}
}

impl Default for HitRecord {
	fn default() -> Self {
		Self {
			p: Point3::default(),
			normal: Vec3::default(),
			mat_ptr: Rc::new(Lambertian::new(Color::default())),
			t: 0.,
			front_face: true,
		}
	}
}

pub trait Hittable {
	fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
