#![allow(dead_code, unused_imports)]
// use wasm_bindgen::prelude::*;
use wee_alloc;
// use js_sys;

mod image;
use image::{Image, Color};

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
		color.red = (255.0 * (x as f32 / width as f32)) as u8;
		color.green = (255.0 * (y as f32 / height as f32)) as u8;
		color.blue = (255.0 * 0.2) as u8;
	}, progress_safe);

	let bytes: Box<[u8]> = output.into();
	let ptr = bytes.as_ptr();
	unsafe {
		RENDER_OUTPUT.replace(bytes);
	}

	ptr
}