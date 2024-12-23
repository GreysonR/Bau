use wasm_bindgen::prelude::*;
use crate::Body;
use std::collections::HashMap;

#[wasm_bindgen]
pub struct World {
	bodies: HashMap<u64, Box<Body>>,
}

#[wasm_bindgen]
impl World {
	pub fn add_body(&mut self, body: Body) {
		self.bodies.insert(body.id, Box::new(body));
	}
	pub fn remove_body(&mut self, id: u64) -> bool {
		self.bodies.remove(&id).is_some()
	}
}