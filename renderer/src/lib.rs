#![allow(dead_code, unused_imports)]
#![feature(clamp)]
use std::panic;
use std::rc::Rc;

mod image;
mod vector;
mod ffi;
mod objects;
mod trace;
mod camera;
mod material;
mod rng;
use material::{ Lambertian, Metalic };
use image::{ Image, Color };
use vector::Vector;
use trace::{ Ray, ray_color };

use wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static mut RENDER_OUTPUT: Option<Box<[u8]>> = None;

#[no_mangle]
extern "C" fn init() {
	panic::set_hook(Box::new(|panic_info| {
		if let Some(payload_str) = panic_info.payload().downcast_ref::<&str>() {
			ffi::log(format!("Panic occurred: {}", payload_str).as_str());
		} else {
			ffi::log("Panic occured, non-str payload.");
		}
		if let Some(loc_info) = panic_info.location() {
			ffi::log(format!("Panic location: {:?}", loc_info).as_str());
		}
	}));
}

#[no_mangle]
extern "C" fn render(aspect: f32, width: usize) -> *const u8 {

	ffi::log(format!("About to render image: {}x{}", width as f32, width as f32 * aspect).as_str());

	let camera = camera::Camera::new(
		// Match the field of view from before:
		126.869897646 * std::f32::consts::PI / 180.0,
		aspect
	);

	let scene = vec![
		objects::Sphere {
			center: Vector::new(0.0, 0.0, -1.0),
			radius: 0.5,
			material: Rc::new(Lambertian {
				albedo: Vector::new(0.7, 0.3, 0.3)
			})
		},
		objects::Sphere {
			center: Vector::new(0.0,-100.5,-1.0), 
			radius: 100.0,
			material: Rc::new(Lambertian {
				albedo: Vector::new(0.8, 0.8, 0.0)
			})
		},
		objects::Sphere {
			center: Vector::new(1.0, 0.0, -1.0), 
			radius: 0.5,
			material: Rc::new(Metalic {
				albedo: Vector::new(0.8, 0.6, 0.2),
				fuzz: 1.0
			})
		},
		objects::Sphere {
			center: Vector::new(-1.0, 0.0, -1.0), 
			radius: 0.5,
			material: Rc::new(Metalic {
				albedo: Vector::new(0.8, 0.8, 0.8),
				fuzz: 0.3
			})
		}
	];

	let output = camera.render(width, 100, |ray| {
		ray_color(&scene, &ray, 50).into()
	}, ffi::progress);
	
	let bytes: Box<[u8]> = output.into();
	let ptr = bytes.as_ptr();
	
	ffi::log(format!("Output located at: {:?}", ptr).as_str());

	unsafe {
		RENDER_OUTPUT.replace(bytes);
	}

	ptr
}