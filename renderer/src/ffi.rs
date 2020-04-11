extern "C" {
	fn set_progress(prog: f32);
	fn console_log(ptr: *const u8, length: u32);
	fn get_random(ptr: *const u8, length: u32);
}
pub fn get_seed() -> [u8; 16] {
	let seed = [0; 16];
	unsafe {
		get_random(seed.as_ptr(), seed.len() as u32);
	}
	seed
}
pub fn log(s: &str) {
	unsafe {
		console_log(s.as_ptr(), s.len() as u32);
	}
}

pub fn progress(prog: f32) {
	unsafe {
		set_progress(prog);
	}
}