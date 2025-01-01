use crate::{physics, Body, BodyOptions, Geo, Id, Time, Vec2, World};
use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use serde::Serialize;

#[wasm_bindgen]
#[derive(Serialize)]
pub struct RenderBody {
	vertices: Vec<Vec2>,
	id: Id,
}

#[wasm_bindgen]
pub struct Engine {
	world: World,
	bodies: HashMap<Id, Body>,
}
#[wasm_bindgen]
impl Engine {
	#[wasm_bindgen(constructor)]
	pub fn new() -> Self {
		Self {
			world: World::new(),
			bodies: HashMap::new(),
		}
	}

	// Body methods
	pub fn body_create_rect(&mut self, width: Geo, height: Geo, position: Vec2, options: BodyOptions) -> Id {
		let body = Body::rectangle(width, height, position, options);
		let id = body.id;
		self.bodies.insert(id, body);
		id
	}
	pub fn body_create_circle(&mut self, radius: Geo, position: Vec2, options: BodyOptions) -> Id {
		let body = Body::circle(radius, position, options);
		let id = body.id;
		self.bodies.insert(id, body);
		id
	}
	pub fn body_set_position(&mut self, body_id: Id, position: Vec2) {
		if !self.bodies.contains_key(&body_id) { return } // Body doesn't exist
		self.bodies.get_mut(&body_id).unwrap().set_position(position);
	}
	pub fn body_translate_position(&mut self, body_id: Id, translation: Vec2) {
		if !self.bodies.contains_key(&body_id) { return } // Body doesn't exist
		self.bodies.get_mut(&body_id).unwrap().translate_position(translation);
	}
	pub fn body_set_velocity(&mut self, body_id: Id, velocity: Vec2) {
		if !self.bodies.contains_key(&body_id) { return } // Body doesn't exist
		self.bodies.get_mut(&body_id).unwrap().set_velocity(velocity);
	}
	pub fn body_apply_velocity(&mut self, body_id: Id, velocity: Vec2) {
		if !self.bodies.contains_key(&body_id) { return } // Body doesn't exist
		self.bodies.get_mut(&body_id).unwrap().apply_velocity(&velocity);
	}
	pub fn body_apply_angular_velocity(&mut self, body_id: Id, velocity: Geo) {
		if !self.bodies.contains_key(&body_id) { return } // Body doesn't exist
		self.bodies.get_mut(&body_id).unwrap().apply_angular_velocity(velocity);
	}
	pub fn body_translate_angle(&mut self, body_id: Id, angle: Geo) {
		if !self.bodies.contains_key(&body_id) { return } // Body doesn't exist
		self.bodies.get_mut(&body_id).unwrap().translate_angle(angle);
	}
	pub fn body_set_angle(&mut self, body_id: Id, angle: Geo) {
		if !self.bodies.contains_key(&body_id) { return } // Body doesn't exist
		self.bodies.get_mut(&body_id).unwrap().set_angle(angle);
	}
	pub fn body_get_position(&self, body_id: Id) -> Vec2 {
		self.bodies.get(&body_id).unwrap().get_position().clone()
	}
	pub fn body_get_vertices(&self, body_id: Id) -> Vec<Vec2> {
		self.bodies.get(&body_id).unwrap().get_vertices().clone()
	}

	// World methods
	pub fn world_add_body(&mut self, body_id: Id) {
		if !self.bodies.contains_key(&body_id) { return } // Body doesn't exist
		self.world.add_body(body_id);
	}
	pub fn world_remove_body(&mut self, body_id: Id) {
		if !self.bodies.contains_key(&body_id) { return } // Body doesn't exist
		self.world.remove_body(body_id);
	}
	pub fn world_get_bodies(&self) -> JsValue {
		let mut bodies: Vec<Id> = Vec::new();
		for body_id in self.world.bodies.iter() {
			bodies.push(*body_id);
		}
		serde_wasm_bindgen::to_value(&bodies).unwrap()
	}
	pub fn world_get_collision_pairs(&self) -> JsValue {
		// world.collision_pairs = HashMap<Id, CollisionPair>
		let mut pairs = Vec::new();
		for pair in self.world.collision_pairs.iter() {
			pairs.push(pair.clone());
		}
		serde_wasm_bindgen::to_value(&pairs).unwrap()
	}

	// Physics methods
	pub fn physics_update(&mut self) {
		let delta: Time = 1.0 / 144.0; // delta time. todo: compute this
		physics::update(&mut self.world, &mut self.bodies, delta);
	}
}