use crate::Geo;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct BodyOptions {
	pub is_static: bool,
	pub mass: Geo,
	pub restitution: Geo,
	pub friction: Geo,
}

impl BodyOptions {
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

impl Default for BodyOptions {
	fn default() -> Self {
		Self {
			is_static: false,
			mass: 1.0,
			restitution: 0.2,
			friction: 0.2,
		}
	}
}

impl From<JsValue> for BodyOptions {
	fn from(value: JsValue) -> BodyOptions {
		if value.is_null_or_undefined() {
			return BodyOptions::default();
		}

		serde_wasm_bindgen::from_value(value)
			.unwrap_or_else(|error| {
				web_sys::console::warn_1(&format!("Failed to parse BodyOptions: {}. Using defaults.", error).into());
				BodyOptions::default()
			})
	}
}