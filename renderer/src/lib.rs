#![allow(dead_code, unused_imports)]

use wee_alloc;

mod image;
use image::{Image, Color};
mod vector;
use vector::Vector;

#[derive(Debug)]
#[derive(Clone, Copy)]
struct Ray {
	pub origin: Vector,
	pub direction: Vector
}
impl Ray {
	fn at(&self, t: f32) -> Vector {
		self.origin + self.direction * t
	}
}


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern "C" {
	fn progress(prog: f32);
	fn console_log(ptr: *const u8, length: u32);
}
fn log_safe(s: &str) {
	unsafe {
		console_log(s.as_ptr(), s.len() as u32);
	}
}

fn progress_safe(prog: f32) {
	unsafe {
		progress(prog);
	}
}

static mut RENDER_OUTPUT: Option<Box<[u8]>> = None;

struct Scene {
	pub spheres: Vec<Sphere>
}
impl Scene {
	fn ray_color(&self, ray: &Ray) -> Color {
		for sphere in self.spheres.iter() {
			if let Some(intersection) = sphere.hit(ray) {
				return Color::new(255, 0, 0, 255);
			}
		}
		let unit_direction = ray.direction.unit();
		let t = 0.5*(unit_direction.y + 1.0);
		(Vector::new(1.0, 1.0, 1.0)*(1.0-t) + Vector::new(0.5, 0.7, 1.0)*t).into()
	}
}


trait Hit {
	fn hit(&self, ray: &Ray) -> Option<Vector>;
}
struct Sphere {
	center: Vector,
	radius: f32
}
impl Hit for Sphere {
	fn hit(&self, ray: &Ray) -> Option<Vector> {
		let oc = ray.origin - self.center;
		let a = Vector::dot(&ray.direction, &ray.direction);
		let b = 2.0 * Vector::dot(&oc, &ray.direction);
		let c = Vector::dot(&oc, &oc) - self.radius * self.radius;
		let discriminant = b*b - 4.0*a*c;
		if discriminant > 0.0 {
			// TODO: return the point of intersection instead of a default vector.
			Some(Vector::default())
		} else {
			None
		}
	}
}

#[no_mangle]
extern "C" fn render(width: usize, height: usize) -> *const u8 {
	let mut output = Image::new(width, height);

	log_safe(format!("About to render image: {}x{}", width, height).as_str());


	// TODO: Use real camera properties (FOV, etc.)
	let vec_height = height as f32 * 4.0 / width as f32;
	let lower_left_corner = Vector::new(-2.0, -(vec_height / 2.0), -1.0);
	let horizontal = Vector::new(4.0, 0.0, 0.0);
	let vertical = Vector::new(0.0, vec_height, 0.0);
	let origin = Vector::default();

	let scene = Scene {
		spheres: vec![
			Sphere {
				center: Vector::new(0.0, 0.0, -1.0),
				radius: 0.5
			}
		]
	};

	output.pixels(|x, y, color| {
		let u = x as f32 / width as f32;
		let v = y as f32 / height as f32;
		// log_safe(format!("Rendering Pixel: x:{}, y:{}, u:{}, v:{}", x, y, u, v).as_str());
		let ray = Ray {
			origin,
			direction: lower_left_corner + horizontal*u + vertical*v
		};
		*color = scene.ray_color(&ray);
	}, progress_safe);
	
	let bytes: Box<[u8]> = output.into();
	let ptr = bytes.as_ptr();
	
	log_safe(format!("Output located at: {:?}", ptr).as_str());

	unsafe {
		RENDER_OUTPUT.replace(bytes);
	}

	ptr
}