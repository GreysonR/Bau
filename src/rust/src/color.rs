use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

#[wasm_bindgen]
impl Color {
	#[wasm_bindgen(constructor)]
	pub fn new(r: u8, g: u8, b: u8) -> Self {
		Self(r, g, b)
	}
	#[wasm_bindgen(js_name = fromHex)]
	pub fn from_hex(hexcode: &str) -> Self {
		let trimmed = hexcode.trim_start_matches("#").trim_start_matches("0x");
		let color = i64::from_str_radix(trimmed, 16).expect("Could not parse color");

		let r = ((color >> 16) % 256) as u8;
		let g = ((color >> 8) % 256) as u8;
		let b = (color % 256) as u8;

		Self::new(r, g, b)
	}
}