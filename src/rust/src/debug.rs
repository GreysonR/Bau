
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	pub fn log(s: &str);
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! console_log {
	($($t:tt)*) => {
		$crate::log(&format!($($t)*))
	};
}