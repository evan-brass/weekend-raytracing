use std::ops::{Add, Sub, Mul, Div};
use std::convert::Into;

use crate::image::Color;

#[derive(Clone, Copy)]
pub struct Vector {
	pub x: f32,
	pub y: f32,
	pub z: f32
}
// Default is the zero vector:
impl Default for Vector {
	fn default() -> Self {
		Self::new(0.0, 0.0, 0.0)
	}
}
// Vector functions:
impl Vector {
	pub fn squared_length(&self) -> f32 {
		Self::dot(self, self)
	}
	pub fn length(&self) -> f32 {
		self.squared_length().sqrt()
	}
	pub fn dot(u: &Self, v: &Self) -> f32 {
		u.x * v.x +
		u.y * v.y +
		u.z * v.z
	}
	pub fn cross(u: &Self, v: &Self) -> Self {
		Vector::new(
			u.y * v.z - u.z * v.y,
			u.z * v.x - u.x * v.z,
			u.x * v.y - u.y * v.x
		)
	}
	pub fn unit(self) -> Self {
		let length = self.length();
		self / length
	}
}
// Operators:
impl Vector {
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		Self {
			x, y, z
		}
	}
}
impl Add for Vector {
	type Output = Self;
	fn add(mut self, other: Self) -> Self::Output {
		self.x += other.x;
		self.y += other.y;
		self.z += other.z;
		self
	}
}
impl Sub for Vector {
	type Output = Self;
	fn sub(self, other: Self) -> Self::Output {
		self + (other * -1.0)
	}
}
impl Mul<f32> for Vector {
	type Output = Self;
	fn mul(mut self, other: f32) -> Self::Output {
		self.x *= other;
		self.y *= other;
		self.z *= other;
		self
	}
}
impl Div<f32> for Vector {
	type Output = Self;
	fn div(mut self, other: f32) -> Self::Output {
		self.x /= other;
		self.y /= other;
		self.z /= other;
		self
	}
}
// Color conversion:
impl Into<Color> for Vector {
	fn into(self) -> Color {
		Color::new(
			(self.x * 255.0) as u8,
			(self.y * 255.0) as u8,
			(self.z * 255.0) as u8,
			255
		)
	}
}