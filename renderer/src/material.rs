use crate::trace::{ Ray, Intersection };
use crate::vector::Vector;
use crate::rng::get_rng;
use rand::Rng;

fn random_unit_vector() -> Vector {
	let rng = get_rng();
	let a = rng.gen_range(0.0, 2.0 * std::f32::consts::PI);
    let z: f32 = rng.gen_range(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    return Vector::new(r * a.cos(), r * a.sin(), z);
}

pub trait Material {
	fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<(Ray, Vector)>;
}

pub struct Lambertian {
	pub albedo: Vector
}
impl Material for Lambertian {
	fn scatter(&self, _ray: &Ray, intersection: &Intersection) -> Option<(Ray, Vector)> {
		let unit = random_unit_vector();
		Some((
			Ray {
				origin: intersection.position,
				direction: intersection.normal + unit
			},
			self.albedo
		))
	}
}

pub struct Metalic {
	pub albedo: Vector,
	pub fuzz: f32
}
impl Material for Metalic {
	fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<(Ray, Vector)> {
		let reflection = Vector::reflect(ray.direction, intersection.normal);
		Some((
			Ray {
				origin: intersection.position,
				direction: reflection + random_unit_vector() * self.fuzz
			},
			self.albedo
		))
	}
}