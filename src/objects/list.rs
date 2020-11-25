use std::rc::Rc;

use crate::ray::Ray;

use super::{HitRecord, Hittable};

pub struct HittableList {
	pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
	pub fn new() -> Self {
		Self { objects: vec![] }
	}

	pub fn add(&mut self, object: Rc<dyn Hittable>) {
		self.objects.push(object);
	}

	pub fn clear(&mut self) {
		while self.objects.len() > 0 {
			self.objects.pop();
		}
	}
}

impl Hittable for HittableList {
	fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
		let mut temp_rec = HitRecord::default();
		let mut hit_anything = false;
		let mut closest_so_far = t_max;

		for object in &self.objects {
			if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
				hit_anything = true;
				closest_so_far = temp_rec.clone().t;
				*rec = temp_rec.clone();
			}
		}

		hit_anything
	}
}
