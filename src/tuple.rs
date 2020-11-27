use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
	pub x: f64,
	pub y: f64,
	pub z: f64,
	pub w: f64,
}

impl Tuple {
	pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
		Self { x, y, z, w }
	}

	pub fn point(x: f64, y: f64, z: f64) -> Self {
		Tuple::new(x, y, z, 1.)
	}

	pub fn vector(x: f64, y: f64, z: f64) -> Self {
		Tuple::new(x, y, z, 0.)
	}

	pub fn is_point(&self) -> bool {
		self.w == 1.
	}

	pub fn is_vector(&self) -> bool {
		self.w == 0.
	}

	pub fn magnitude(&self) -> f64 {
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}

	pub fn normalize(&self) -> Tuple {
		let m = self.magnitude();
		Tuple::vector(self.x / m, self.y / m, self.z / m)
	}

	pub fn dot(&self, t: &Tuple) -> f64 {
		self.x * t.x + self.y * t.y + self.z * t.z + self.w * t.w
	}

	pub fn cross(&self, t: &Tuple) -> Self {
		Tuple::vector(
			self.y * t.z - self.z * t.y,
			self.z * t.x - self.x * t.z,
			self.x * t.y - self.y * t.x,
		)
	}

	pub fn reflect(&self, normal: Tuple) -> Self {
		*self - normal * 2. * self.dot(&normal)
	}
}

impl PartialEq for Tuple {
	fn eq(&self, other: &Self) -> bool {
		super::approx_eq(self.x, other.x)
			&& super::approx_eq(self.y, other.y)
			&& super::approx_eq(self.z, other.z)
			&& self.w == other.w
	}
}

impl ops::Add<Tuple> for Tuple {
	type Output = Tuple;
	fn add(self, rhs: Tuple) -> Tuple {
		Tuple {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
			w: self.w + rhs.w,
		}
	}
}

impl ops::Sub<Tuple> for Tuple {
	type Output = Tuple;
	fn sub(self, rhs: Tuple) -> Tuple {
		Tuple {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
			w: self.w - rhs.w,
		}
	}
}

impl ops::Neg for Tuple {
	type Output = Tuple;
	fn neg(self) -> Tuple {
		Tuple {
			x: -self.x,
			y: -self.y,
			z: -self.z,
			w: -self.w,
		}
	}
}

impl ops::Mul<f64> for Tuple {
	type Output = Tuple;
	fn mul(self, rhs: f64) -> Tuple {
		Tuple {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
			w: self.w * rhs,
		}
	}
}

impl ops::Div<f64> for Tuple {
	type Output = Tuple;
	fn div(self, rhs: f64) -> Tuple {
		Tuple {
			x: self.x / rhs,
			y: self.y / rhs,
			z: self.z / rhs,
			w: self.w / rhs,
		}
	}
}

impl From<[f64; 4]> for Tuple {
	fn from(item: [f64; 4]) -> Self {
		Tuple {
			x: item[0],
			y: item[1],
			z: item[2],
			w: item[3],
		}
	}
}

