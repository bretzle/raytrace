use std::ops;

pub const BLACK: Color = Color {
	r: 0.,
	g: 0.,
	b: 0.,
};
pub const WHITE: Color = Color {
	r: 1.,
	g: 1.,
	b: 1.,
};
pub const RED: Color = Color {
	r: 1.,
	g: 0.,
	b: 0.,
};
pub const GREEN: Color = Color {
	r: 0.,
	g: 1.,
	b: 0.,
};

#[derive(Debug, Copy, Clone)]
pub struct Color {
	pub r: f64,
	pub g: f64,
	pub b: f64,
}

impl Color {
	pub fn new(r: f64, g: f64, b: f64) -> Color {
		Color { r, g, b }
	}
}

impl PartialEq for Color {
	fn eq(&self, other: &Self) -> bool {
		super::approx_eq(self.r, other.r)
			&& super::approx_eq(self.g, other.g)
			&& super::approx_eq(self.b, other.b)
	}
}

impl ops::Add<Color> for Color {
	type Output = Color;
	fn add(self, rhs: Color) -> Color {
		Color {
			r: self.r + rhs.r,
			g: self.g + rhs.g,
			b: self.b + rhs.b,
		}
	}
}

impl ops::Sub<Color> for Color {
	type Output = Color;
	fn sub(self, rhs: Color) -> Color {
		Color {
			r: self.r - rhs.r,
			g: self.g - rhs.g,
			b: self.b - rhs.b,
		}
	}
}

impl ops::Mul<f64> for Color {
	type Output = Color;
	fn mul(self, rhs: f64) -> Color {
		Color {
			r: self.r * rhs,
			g: self.g * rhs,
			b: self.b * rhs,
		}
	}
}

impl ops::Mul<Color> for Color {
	type Output = Color;
	fn mul(self, rhs: Color) -> Color {
		Color {
			r: self.r * rhs.r,
			g: self.g * rhs.g,
			b: self.b * rhs.b,
		}
	}
}
