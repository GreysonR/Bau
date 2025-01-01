use crate::Geo;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct BodyOptions {
	pub is_static: bool,
	pub mass: Geo,
	pub restitution: Geo,
	pub friction: Geo,
}
#[wasm_bindgen]
impl BodyOptions {
	#[wasm_bindgen(constructor)]
	pub fn new() -> BodyOptions {
		BodyOptions {
			is_static: false,
			mass: 1.0,
			restitution: 0.3,
			friction: 0.2,
		}
	}
	pub fn is_static(mut self, s: bool) -> Self {
		self.is_static = s;
		self
	}
	pub fn mass(mut self, m: Geo) -> Self {
		self.mass = m;
		self
	}
	pub fn restitution(mut self, r: Geo) -> Self {
		self.restitution = r;
		self
	}
	pub fn friction(mut self, f: Geo) -> Self {
		self.friction = f;
		self
	}
}