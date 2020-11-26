use rand::Rng;

pub use std::f64::consts::PI;
pub use std::f64::INFINITY;

pub fn degrees_to_radians(degrees: f64) -> f64 {
	degrees * PI / 180.0
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
	if x < min {
		return min;
	}
	if x > max {
		return max;
	}
	x
}

pub fn random(min: f64, max: f64) -> f64 {
	rand::thread_rng().gen_range(min, max)
}
