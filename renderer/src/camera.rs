use crate::vector::Vector;
use crate::image::{ Image, Color };
use crate::trace::Ray;
use crate::ffi;
use crate::rng::get_rng;
use rand::Rng;

pub struct Camera {
	fov: f32,
	aspect: f32
	// TODO: Add transformation matrix that moves / points the camera.
}
impl Camera {
	pub fn new(fov: f32, aspect: f32) -> Self {
		Camera { fov, aspect }
	}
	pub fn render<F: FnMut(Ray) -> Vector, P: Fn(f32)>(&self, width: usize, samples: usize, mut cast: F, progress: P) -> Image {
		let height = (self.aspect * width as f32) as usize;
		let origin = Vector::new(0.0, 0.0, 0.0);
		
		let mut output = Image::new(width, height);

		// Used for progress:
		let mut casts_done: usize = 0;
		let casts_total: usize = samples * width * height;

		let z = -1.0 / (self.fov / 2.0).tan();

		ffi::log(format!(
			"Casts will range from {:?} to {:?}",
			Vector::new(-1.0, -self.aspect, z),
			Vector::new(1.0, self.aspect, z)
		).as_str());

		output.pixels(|x, y, color| {
			let rng = get_rng();

			// Accumulate the color from the samples:
			let mut accum = Vector::default();

			for _ in 0..samples {
				// TODO: Simplify
				let u = (x as f32 + rng.gen_range(0.0, 1.0)) / width as f32;
				let v = (y as f32 + rng.gen_range(0.0, 1.0)) / height as f32;
				let ray = Ray {
					origin,
					direction: Vector::new(
						2.0 * u - 1.0,
						2.0 * self.aspect * v - self.aspect,
						z
					)
				};

				accum = accum + cast(ray);

				// Supply progress information:
				casts_done += 1;
				progress(casts_done as f32 / casts_total as f32);
			}

			// Average...
			accum = accum / samples as f32;
			// ...and gamma correct:
			accum.x = accum.x.sqrt().clamp(0.0, 0.999);
			accum.y = accum.y.sqrt().clamp(0.0, 0.999);
			accum.z = accum.z.sqrt().clamp(0.0, 0.999);
			*color = accum.into();
		});

		output
	}
}