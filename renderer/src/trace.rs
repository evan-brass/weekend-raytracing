use crate::vector::Vector;
use crate::image::Color;
use crate::material::Material;
use std::rc::Rc;

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
	pub fn make_intersection(&self, position: Vector, normal: Vector, t: f32, material: Rc<dyn Material>) -> Intersection {
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
			hit_side,
			material
		}
	}
}

pub fn ray_color<T: Hittable>(object: &T, ray: &Ray, depth_left: usize) -> Vector {
	// Return black when we run out of depth
	if depth_left == 0 {
		return Vector::new(0.0, 0.0, 0.0);
	}

	// Use the material to figure out the color:
	if let Some(intersection) = object.hit(ray, 0.001, std::f32::INFINITY) {
		if let Some((scatter_ray, attenuation)) = intersection.material.scatter(ray, &intersection) {
			return Vector::mult_parts(attenuation, ray_color(object, &scatter_ray, depth_left - 1));
		} else {
			return Vector::new(0.0, 0.0, 0.0);
		}
	}

	// Background-color:
	let unit_direction = ray.direction.unit();
	let t = 0.5*(unit_direction.y + 1.0);
	return Vector::new(1.0, 1.0, 1.0) * (1.0-t) + Vector::new(0.5, 0.7, 1.0) * t;
}

pub enum GeometrySide {
	Inside,
	Outside
}
pub struct Intersection {
	pub t: f32,
	pub position: Vector,
	pub normal: Vector,
	pub hit_side: GeometrySide,
	pub material: Rc<dyn Material>
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