use wasm_bindgen::prelude::*;
use crate::Color;

#[wasm_bindgen]
pub struct BodyRender {
	pub fill: Color,
}
#[wasm_bindgen]
impl BodyRender {
	pub fn new(fill: Color) -> Self {
		Self {
			fill,
		} 
	}
}