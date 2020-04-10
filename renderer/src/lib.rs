#![allow(dead_code, unused_imports)]

use wee_alloc;

mod image;
use image::{Image, Color};
mod vector;
use vector::Vector;


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern "C" {
	fn progress(prog: f32);
}
fn progress_safe(prog: f32) {
	unsafe {
		progress(prog);
	}
}

static mut RENDER_OUTPUT: Option<Box<[u8]>> = None;

#[no_mangle]
extern "C" fn render(width: usize, height: usize) -> *const u8 {
	let mut output = Image::new(width, height);

	output.pixels(|x, y, color| {
		*color = Vector::new(
			x as f32 / width as f32,
			y as f32 / height as f32,
			0.2
		).into()
	}, progress_safe);

	let bytes: Box<[u8]> = output.into();
	let ptr = bytes.as_ptr();
	unsafe {
		RENDER_OUTPUT.replace(bytes);
	}

	ptr
}