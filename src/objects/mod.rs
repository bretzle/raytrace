pub mod list;
pub mod sphere;

use crate::{
	ray::Ray,
	vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub struct HitRecord {
	pub p: Point3,
	pub normal: Vec3,
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
			t: 0.,
			front_face: true,
		}
	}
}

pub trait Hittable {
	fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
