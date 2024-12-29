use crate::{Vec2, Geo, Id, Body, Physics, World};
use wasm_bindgen::prelude::*;
use std::collections::HashMap;

#[wasm_bindgen]
pub struct Engine {
	world: World,
	bodies: HashMap<Id, Body>,
	physics: Physics,
}
#[wasm_bindgen]
impl Engine {
	#[wasm_bindgen(constructor)]
	pub fn new() -> Self {
		Self {
			world: World::new(),
			bodies: HashMap::new(),
			physics: Physics::new(),
		}
	}

	// Render methods
	pub fn get_bodies_vertices(&self) -> JsValue {
		let mut vertices: Vec<Vec<Vec2>> = Vec::new();
		for body_id in self.world.bodies.iter() {
			let body = self.bodies.get(body_id).unwrap();
			vertices.push(body.get_vertices().clone())
		}
		serde_wasm_bindgen::to_value(&vertices).unwrap()
	}

	// Body methods
	pub fn body_create_rect(&mut self, width: Geo, height: Geo, position: Vec2) -> Id {
		let body = Body::rectangle(width, height, position);
		let id = body.id;
		self.bodies.insert(id, body);
		id
	}
	pub fn body_translate_position(&mut self, body_id: Id, translation: Vec2) {
		if !self.bodies.contains_key(&body_id) { return } // Body doesn't exist
		self.bodies.get_mut(&body_id).unwrap().translate_position(translation);
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
		self.physics.update(&mut self.world);
	}
}