use std::ops;

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

	pub fn dot(u: &Self, v: &Self) -> f64 {
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

	fn sub(self, rhs: Self) -> Self {
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

impl ops::Div<f64> for Vec3 {
	type Output = Self;

	fn div(self, t: f64) -> Self {
		self * (1. / t)
	}
}

impl std::fmt::Display for Vec3 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{}, {}, {}]", self.x(), self.y(), self.z())
	}
}
