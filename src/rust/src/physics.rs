use wasm_bindgen::prelude::*;
use crate::World;

#[macro_export]
macro_rules! console_log {
	($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct Physics {

}

impl Physics {
	pub fn new() -> Self {
		Self {}
	}
	pub fn update(&mut self, world: &mut World) {
		// let bodies = world.get_bodies();
		// console_log!("world has {} bodies", bodies.len());
		// println!("world has {} bodies", bodies.len());
	}
}