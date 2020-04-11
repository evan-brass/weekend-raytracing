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
	fn make_intersection(&self, position: Vector, normal: Vector, t: f32) -> Intersection {
		let hit_side = if Vector::dot(&self.direction, &normal) > 0.0 {
			GeometrySide::Inside
		} else {
			GeometrySide::Outside
		};
		let normal = match hit_side {
			GeometrySide::Inside => normal * -1.0,
			GeometrySide::Outside => normal
		};
		Intersection {
			t,
			position,
			normal,
			hit_side
		}
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

fn ray_color<T: Hittable>(object: T, ray: &Ray) -> Color {
	if let Some(intersection) = object.hit(ray, 0.0, std::f32::INFINITY) {
		return ((intersection.normal + Vector::new(1.0, 1.0, 1.0)) * 0.5).into();
	}
	let unit_direction = ray.direction.unit();
	let t = 0.5*(unit_direction.y + 1.0);
	(Vector::new(1.0, 1.0, 1.0)*(1.0-t) + Vector::new(0.5, 0.7, 1.0)*t).into()
}


enum GeometrySide {
	Inside,
	Outside
}
struct Intersection {
	t: f32,
	position: Vector,
	normal: Vector,
	hit_side: GeometrySide
}
trait Hittable {
	fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<Intersection>;
}
struct Sphere {
	center: Vector,
	radius: f32
}
impl Hittable for Sphere {
	fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<Intersection> {
		let oc = ray.origin - self.center;
		let a = ray.direction.length_squared();
		let half_b = Vector::dot(&oc, &ray.direction);
		let c = oc.length_squared() - self.radius * self.radius;
		let discriminant = half_b * half_b - a * c;
		
		if discriminant <= 0.0 {
			// No intersection solutions for this ray.
			return None;
		}

		let root = discriminant.sqrt();
		let mut t = (-half_b - root) / a;
		if t < tmin || t > tmax {
			t = (-half_b + root) / a;
			if t < tmin || t > tmax {
				// Neither intersection solution is in range.
				return None;
			}
		}
		let position = ray.at(t);
		// let normal = (position - self.center) / self.radius;
		let normal = (position - self.center).unit();
		return Some(ray.make_intersection(position, normal, t));
	}
}
impl<T: Hittable> Hittable for Vec<T> {
	fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<Intersection> {
		let mut closest: Option<Intersection> = None;
		for item in self.iter() {
			if let Some(intersection) = item.hit(ray, tmin, tmax) {
				match closest {
					Some(Intersection { t, .. }) if intersection.t < t => {
						closest = Some(intersection)
					},
					None => closest = Some(intersection),
					_ => ()
				}
			}
		}
		closest
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


	output.pixels(|x, y, color| {
		let u = x as f32 / width as f32;
		let v = y as f32 / height as f32;
		// log_safe(format!("Rendering Pixel: x:{}, y:{}, u:{}, v:{}", x, y, u, v).as_str());
		let ray = Ray {
			origin,
			direction: lower_left_corner + horizontal*u + vertical*v
		};
		*color = ray_color(
			vec![
				Sphere {
					center: Vector::new(0.0, 0.0, -1.0),
					radius: 0.5
				},
				Sphere {
					center: Vector::new(0.0,-100.5,-1.0), 
					radius: 100.0
				}
			],
			&ray
		);
	}, progress_safe);
	
	let bytes: Box<[u8]> = output.into();
	let ptr = bytes.as_ptr();
	
	log_safe(format!("Output located at: {:?}", ptr).as_str());

	unsafe {
		RENDER_OUTPUT.replace(bytes);
	}

	ptr
}