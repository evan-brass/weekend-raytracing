use std::convert::Into;

// Color:
#[repr(packed)]
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug)]
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
#[derive(Debug)]
pub struct Image {
	width: usize,
	height: usize,
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
	pub fn set(&mut self, x: usize, y: usize, color: Color) {
		if x >= self.width || y >= self.height {
			panic!("Set called with invalid location: x: {}, y: {} but image is {}x{}", x, y, self.width, self.height);
		}
		self.data[y * self.width + x] = color;
	}
	pub fn pixels<F: FnMut(usize, usize, &mut Color)>(&mut self, mut f: F) {
		let width = self.width;
		let height = self.height;
		for (ind, item) in self.data.iter_mut().enumerate() {
			let x = ind % width;
			let y = height - ind / width;

			f(x, y, item);
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