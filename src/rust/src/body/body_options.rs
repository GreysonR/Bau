use crate::Geo;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct BodyOptions {
	pub is_static: bool,
	pub mass: Geo,
}
#[wasm_bindgen]
impl BodyOptions {
	#[wasm_bindgen(constructor)]
	pub fn new() -> BodyOptions {
		BodyOptions {
			is_static: false,
			mass: 1.0,
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
}