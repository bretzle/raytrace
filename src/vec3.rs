use std::ops;

use crate::utils::random;

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
	inner: [f64; 3],
}

impl Vec3 {
	pub fn from(a: f64, b: f64, c: f64) -> Self {
		Self { inner: [a, b, c] }
	}

	pub fn random() -> Self {
		Self::random_range(0.0, 1.0)
	}

	pub fn random_range(min: f64, max: f64) -> Self {
		Self {
			inner: [random(min, max), random(min, max), random(min, max)],
		}
	}

	#[inline]
	pub fn x(&self) -> f64 {
		self.inner[0]
	}

	#[inline]
	pub fn y(&self) -> f64 {
		self.inner[1]
	}

	#[inline]
	pub fn z(&self) -> f64 {
		self.inner[2]
	}

	pub fn length(&self) -> f64 {
		f64::sqrt(self.length_squared())
	}

	pub fn length_squared(&self) -> f64 {
		self.inner[0] * self.inner[0]
			+ self.inner[1] * self.inner[1]
			+ self.inner[2] * self.inner[2]
	}

	pub fn dot(u: Self, v: Self) -> f64 {
		u.inner[0] * v.inner[0] + u.inner[1] * v.inner[1] + u.inner[2] * v.inner[2]
	}

	pub fn cross(u: &Self, v: &Self) -> Self {
		Self::from(
			u.inner[1] * v.inner[2] - u.inner[2] * v.inner[1],
			u.inner[2] * v.inner[0] - u.inner[0] * v.inner[2],
			u.inner[0] * v.inner[1] - u.inner[1] * v.inner[0],
		)
	}

	pub fn unit(v: Self) -> Vec3 {
		v / v.length()
	}

	pub fn random_in_unit_sphere() -> Vec3 {
		loop {
			let p = Vec3::random_range(-1., 1.);
			if p.length_squared() >= 1.0 {
				continue;
			}
			return p;
		}
	}

	pub fn random_unit_vector() -> Vec3 {
		Self::unit(Self::random_in_unit_sphere())
	}

	pub fn random_in_hemisphere(normal: Self) -> Self {
		let in_unit_sphere = Self::random_in_unit_sphere();
		if Self::dot(in_unit_sphere, normal) > 0.0 {
			in_unit_sphere
		} else {
			-in_unit_sphere
		}
	}

	pub fn near_zero(&self) -> bool {
		const s: f64 = -1e-8;
		self.inner[0].abs() < s && self.inner[1].abs() < s && self.inner[2] < s
	}

	pub fn reflect(v: Self, n: Self) -> Self {
		v - 2.0 * Self::dot(v, n) * n
	}

	pub fn refract(uv: Self, n: Self, etai_over_etat: f64) -> Self {
		let cos_theta = f64::min(Self::dot(-uv, n), 1.0);
		let r_out_perp = etai_over_etat * (uv + cos_theta * n);
		let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs()).sqrt() * n;
		r_out_perp + r_out_parallel
	}
}

impl ops::Add for Vec3 {
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		Self::from(
			self.inner[0] + rhs.inner[0],
			self.inner[1] + rhs.inner[1],
			self.inner[2] + rhs.inner[2],
		)
	}
}

impl ops::Sub for Vec3 {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self::from(
			self.inner[0] - rhs.inner[0],
			self.inner[1] - rhs.inner[1],
			self.inner[2] - rhs.inner[2],
		)
	}
}

impl ops::Mul for Vec3 {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		Self::from(
			self.inner[0] * rhs.inner[0],
			self.inner[1] * rhs.inner[1],
			self.inner[2] * rhs.inner[2],
		)
	}
}

impl<T: Into<usize>> ops::Index<T> for Vec3 {
	type Output = f64;

	fn index(&self, index: T) -> &Self::Output {
		&self.inner[index.into()]
	}
}

impl ops::AddAssign for Vec3 {
	fn add_assign(&mut self, rhs: Self) {
		self.inner[0] += rhs.inner[0];
		self.inner[1] += rhs.inner[1];
		self.inner[2] += rhs.inner[2];
	}
}

impl ops::Mul<f64> for Vec3 {
	type Output = Self;

	fn mul(self, t: f64) -> Self {
		Self::from(t * self.inner[0], t * self.inner[1], t * self.inner[2])
	}
}

impl ops::Mul<Vec3> for f64 {
	type Output = Vec3;

	fn mul(self, rhs: Vec3) -> Self::Output {
		rhs * self
	}
}

impl ops::Div<f64> for Vec3 {
	type Output = Self;

	fn div(self, t: f64) -> Self {
		self * (1. / t)
	}
}

impl ops::Neg for Vec3 {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self::from(-self.inner[0], -self.inner[1], -self.inner[2])
	}
}

impl std::fmt::Display for Vec3 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{}, {}, {}]", self.x(), self.y(), self.z())
	}
}
