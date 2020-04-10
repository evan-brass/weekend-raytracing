use std::convert::Into;
// use wasm_bindgen::prelude::*;
// use js_sys;

// Color:
#[repr(packed)]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
	pub alpha: u8
}
impl Color {
	pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
		Color { red, green, blue, alpha }
	}
}
impl Default for Color {
	fn default() -> Self {
		// Debug color (CSS hotpink)
		Self::new(255, 105, 180, 255)
	}
}

// Image:
pub struct Image {
	pub width: usize,
	pub height: usize,
	data: Box<[Color]>
}
impl Image {
	pub fn new(width: usize, height: usize) -> Self {
		let mut vec = Vec::new();
		vec.resize(width * height, Color::default());
		let data = vec.into_boxed_slice();
		Image {
			width, height,
			data
		}
	}
	pub fn pixels<F: Fn(usize, usize, &mut Color), P: Fn(f32)>(&mut self, f: F, p: P) {
		let width = self.width;
		let height = self.height;
		let length = self.data.len();
		for (ind, item) in self.data.iter_mut().enumerate() {
			let x = ind % width;
			let y = height - ind / height;

			f(x, y, item);

			// progress.call1(&JsValue::NULL, &JsValue::from(ind as f32 / length as f32));
			p(ind as f32 / length as f32);
		};
	}
}
impl Into<Box<[u8]>> for Image {
	fn into(self) -> Box<[u8]> {
		let length = self.data.len() * std::mem::size_of::<Color>();

		let mut vec_old: Vec<Color> = self.data.into_vec();
		let vec_new: Vec<u8> = unsafe {
			Vec::from_raw_parts(vec_old.as_mut_ptr() as *mut u8, length, length)
		};
		std::mem::forget(vec_old);

		vec_new.into_boxed_slice()
	}
}