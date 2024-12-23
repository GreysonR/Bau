extern crate web_sys;
// use wasm_bindgen::prelude::*;

type Geo = f32;

mod world;
pub use world::World;

mod bodies;
pub use bodies::Body;

mod vec2;
pub use vec2::Vec2;

mod console_log;
// use console_log::*;