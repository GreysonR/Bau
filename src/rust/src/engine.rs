use crate::{physics, Body, Geo, Id, Time, Vec2, World};
use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use serde::*;

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

	// Render methods
	pub fn get_bodies_vertices(&self) -> JsValue {
		let mut bodies: Vec<RenderBody> = Vec::new();
		for body_id in self.world.bodies.iter() {
			let body = self.bodies.get(body_id).unwrap();
			bodies.push(RenderBody {
				vertices: body.get_vertices().clone(),
				id: body.id,
			});
		}
		serde_wasm_bindgen::to_value(&bodies).unwrap()
	}
	pub fn get_collision_pairs(&self) -> JsValue {
		// world.collision_pairs = HashMap<Id, CollisionPair>
		let mut pairs: Vec<Vec<Id>> = Vec::new();
		for pair in self.world.collision_pairs.iter() {
			pairs.push(vec![pair.body_a, pair.body_b]);
		}
		serde_wasm_bindgen::to_value(&pairs).unwrap()
	}

	// Body methods
	pub fn body_create_rect(&mut self, width: Geo, height: Geo, position: Vec2, is_static: bool) -> Id {
		let body = Body::rectangle(width, height, position, is_static);
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

	// World methods
	pub fn world_add_body(&mut self, body_id: Id) {
		if !self.bodies.contains_key(&body_id) { return } // Body doesn't exist
		self.world.add_body(body_id);
	}
	pub fn world_remove_body(&mut self, body_id: Id) {
		if !self.bodies.contains_key(&body_id) { return } // Body doesn't exist
		self.world.remove_body(body_id);
	}

	// Physics methods
	pub fn physics_update(&mut self) {
		let delta: Time = 1.0 / 144.0; // delta time. todo: compute this
		physics::update(&mut self.world, &mut self.bodies, delta);
	}
}