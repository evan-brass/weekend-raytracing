use wasm_bindgen::prelude::*;

extern "C" {
	fn progress(percent: u32);
}

fn safe_progress(percent: u32) {
	unsafe {
		progress(percent);
	}
}

#[repr(packed)]
#[repr(C)]
#[derive(Clone)]
struct Color {
	red: u8,
	green: u8,
	blue: u8,
	alpha: u8
}
impl Default for Color {
	fn default() -> Self {
		Color {
			// Debug color  (CSS hotpink)
			red: 255,
			green: 105,
			blue: 180,
			alpha: 255
		}
	}
}

fn colors_to_bytes(input: Box<[Color]>) -> Box<[u8]> {
	let length = input.len() * std::mem::size_of::<Color>();

	let mut vec_old: Vec<Color> = input.into_vec();
	let vec_new: Vec<u8> = unsafe {
		Vec::from_raw_parts(vec_old.as_mut_ptr() as *mut u8, length, length)
	};
	std::mem::forget(vec_old);

	vec_new.into_boxed_slice()
}

#[wasm_bindgen]
pub fn render(width: usize, height: usize) -> Box<[u8]> {
	let mut output = {
		// Build a slice that's the right size:
		let mut vec = Vec::new();
		vec.resize(width * height, Color::default());
		vec.into_boxed_slice()
	};

	colors_to_bytes(output)
}