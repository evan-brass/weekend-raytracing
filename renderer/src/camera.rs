use crate::vector::Vector;
use crate::image::{ Image, Color };
use crate::trace::Ray;
use crate::ffi;
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
	pub fn render<R: Rng, F: Fn(Ray) -> Color, P: Fn(f32)>(&self, width: usize, samples: usize, rng: &mut R, cast: F, progress: P) -> Image {
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
			// Accumulate the color from the samples:
			let mut acc_r: usize = 0;
			let mut acc_g: usize = 0;
			let mut acc_b: usize = 0;
			let mut acc_a: usize = 0;

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

				let sample = cast(ray);
				acc_r += sample.red as usize;
				acc_g += sample.green as usize;
				acc_b += sample.blue as usize;
				acc_a += sample.alpha as usize;

				// Supply progress information:
				casts_done += 1;
				progress(casts_done as f32 / casts_total as f32);
			}

			// Then return the average:
			*color = Color::new(
				(acc_r / samples) as u8,
				(acc_g / samples) as u8,
				(acc_b / samples) as u8,
				(acc_a / samples) as u8
			);
		});

		output
	}
}