use std::{ ops::Index, ops::IndexMut};

use crate::{
	color::{Color, BLACK},
	utils::{clamp_and_normalize, split_long_lines},
};

pub struct Canvas {
	pub width: usize,
	pub height: usize,
	pub pixels: Vec<Color>,
}

impl Canvas {
	pub fn new(width: usize, height: usize) -> Self {
		let pixels = vec![BLACK; width * height];
		Self {
			width,
			height,
			pixels,
		}
	}

	fn to_row_major(&self, i: usize, j: usize) -> usize {
		self.width * i + j
	}

	pub fn write_pixel(&mut self, x: usize, y: usize, pixel: Color) -> Result<(), String> {
		if x < self.width {
			if y < self.height {
				let i = self.to_row_major(y, x);
				self.pixels[i] = pixel;
				Ok(())
			} else {
				Err(format!(
					"Tried accessing canvas out of bounds. Max y-index={}, actual index={}.",
					self.height - 1,
					y
				))
			}
		} else {
			Err(format!(
				"Tried accessing canvas out of bounds. Max x-index={}, actual index={}.",
				self.width - 1,
				x
			))
		}
	}

	pub fn as_ppm(&self) -> String {
		let header = format!("P3\n{} {}\n255\n", self.width, self.height);

		let lines = self.iter_rows().fold(vec![], |mut buf, row| {
			let row_buf = row.fold(vec![], |mut row_buf, pixel| {
				row_buf.push(format!(
					"{} {} {}",
					clamp_and_normalize(pixel.r, 255),
					clamp_and_normalize(pixel.g, 255),
					clamp_and_normalize(pixel.b, 255)
				));
				row_buf
			});
			buf.push(row_buf.join(" "));
			buf
		});
		let length_verified_buf = lines
			.iter()
			.map(|s| split_long_lines(70, s))
			.map(|short_lines| short_lines.join("\n"))
			.collect::<Vec<String>>();
		let data = length_verified_buf.join("\n");
		format!("{}{}\n", header, data)
	}

	/// Iterate over all elements
	pub fn iter(&self) -> impl Iterator<Item = &Color> {
		self.pixels.iter()
	}

	/// Iterate over the ith row of the Canvas
	pub fn iter_row(&self, i: usize) -> impl Iterator<Item = &Color> {
		self.iter().skip(i * self.width).take(self.width)
	}

	/// Iterate over the jth coloumn of the Canvas
	pub fn iter_col(&self, j: usize) -> impl Iterator<Item = &Color> {
		self.iter().skip(j).step_by(self.width)
	}

	/// Iterate over all rows of the Canvas
	pub fn iter_rows(&self) -> impl Iterator<Item = impl Iterator<Item = &Color>> {
		(0..self.height).map(move |i| self.iter_row(i))
	}
}

impl Index<(usize, usize)> for Canvas {
	type Output = Color;

	fn index(&self, coords: (usize, usize)) -> &Self::Output {
		let (i, j) = coords;
		&self.pixels[self.to_row_major(i, j)]
	}
}

impl IndexMut<(usize, usize)> for Canvas {
	fn index_mut(&mut self, coords: (usize, usize)) -> &mut Self::Output {
		let (i, j) = coords;
		let idx = self.to_row_major(i, j);
		&mut self.pixels[idx]
	}
}
