use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use crate::ffi;

static mut GLOBAL_RNG: Option<SmallRng> = None;

pub fn get_rng() -> &'static mut SmallRng {
	unsafe {
		// we have multipl mutable references to global_rng except since we're single threaded (any other thread would be running in a separate worker) then I think this is fine.
		GLOBAL_RNG.get_or_insert_with(|| {
			SmallRng::from_seed(ffi::get_seed())
		})
	}
}