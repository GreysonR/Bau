use wasm_bindgen::prelude::*;
use std::{collections::HashMap, ops::Mul};
use crate::{Engine, World, Body, Id};
use web_sys::console;

#[wasm_bindgen]
pub struct Physics {}

impl Physics {
	pub fn new() -> Self {
		Self {}
	}
	pub fn update(&mut self, world: &mut World, bodies: &mut HashMap<Id, Body>) {
		// console::log_1(&format!("world has {} bodies", world.num_bodies()).into());

		// apply forces
		self.apply_forces(world, bodies);


		// apply velocities
		self.apply_velocity(world, bodies);
	}
	fn apply_forces(&mut self, world: &mut World, bodies: &mut HashMap<Id, Body>) {
		let gravity = &world.gravity;
		for body_id in world.bodies.iter() {
			let body = bodies.get_mut(body_id).unwrap();
			body.apply_velocity(gravity);
		}
	}
	fn apply_velocity(&mut self, world: &mut World, bodies: &mut HashMap<Id, Body>) {
		// todo: use actual delta time
		let delta_t = 1.0 / 144.0f32; // delta time, 144 is the assumed framerate
		for body_id in world.bodies.iter() {
			let body = bodies.get_mut(body_id).unwrap();
			body.translate_position(*body.get_velocity() * &delta_t); // todo: average cur velocity with last velocity for trapezoidal approx
		}
	}
}