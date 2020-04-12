use crate::vector::Vector;
use crate::image::Color;
use rand::Rng;

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Ray {
	pub origin: Vector,
	pub direction: Vector
}
impl Ray {
	pub fn at(&self, t: f32) -> Vector {
		self.origin + self.direction * t
	}
	pub fn make_intersection(&self, position: Vector, normal: Vector, t: f32) -> Intersection {
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

fn random_unit_vector<T: Rng>(rng: &mut T) -> Vector {
	let a = rng.gen_range(0.0, 2.0 * std::f32::consts::PI);
    let z: f32 = rng.gen_range(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    return Vector::new(r * a.cos(), r * a.sin(), z);
	// loop {
	// 	let test = Vector::new(
	// 		rng.gen_range(-1.0, 1.0),
	// 		rng.gen_range(-1.0, 1.0),
	// 		rng.gen_range(-1.0, 1.0)
	// 	);
	// 	if test.length() < 1.0 {
	// 		break test;
	// 	}
	// }
}

pub fn ray_color<T: Hittable, R: Rng>(object: &T, rng: &mut R, ray: &Ray, depth_left: usize) -> Vector {
	if depth_left == 0 {
		return Vector::new(0.0, 0.0, 0.0);
	}

	if let Some(intersection) = object.hit(ray, 0.001, std::f32::INFINITY) {
		let unit = random_unit_vector(rng);
		return ray_color(object, rng, &Ray {
			origin: intersection.position,
			direction: intersection.normal + unit
		}, depth_left - 1) * 0.5;
	}
	let unit_direction = ray.direction.unit();
	let t = 0.5*(unit_direction.y + 1.0);
	Vector::new(1.0, 1.0, 1.0) * (1.0-t) + Vector::new(0.5, 0.7, 1.0) * t
}

pub enum GeometrySide {
	Inside,
	Outside
}
pub struct Intersection {
	pub t: f32,
	pub position: Vector,
	pub normal: Vector,
	pub hit_side: GeometrySide
}
pub trait Hittable {
	fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<Intersection>;
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