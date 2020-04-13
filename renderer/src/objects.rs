use crate::vector::Vector;
use crate::trace::{ Ray, Hittable, Intersection };
use crate::material::Material;
use std::rc::Rc;

pub struct Sphere {
	pub center: Vector,
	pub radius: f32,
	pub material: Rc<dyn Material>
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
		return Some(ray.make_intersection(position, normal, t, self.material.clone()));
	}
}