use std::hash::Hash;
use std::hash::Hasher;

use wasm_bindgen::prelude::*;
use crate::Vec2;
use crate::Geo;

#[wasm_bindgen]
pub struct Body {
	pub id: u64,
	vertices: Vec<Vec2>,
	position: Vec2,
	angle: Geo,
	velocity: Vec2,
}

#[wasm_bindgen]
impl Body {
	// constructors
	pub fn new(vertices: Vec<Vec2>, position: Vec2) -> Body {
		assert!(vertices.len() >= 3); // There should be at least 3 vertices for a valid body
		
		let mut body = Body {
			id: rand::random(),
			vertices,
			position: Vec2::new(0.0, 0.0),
			velocity: Vec2::new(0.0, 0.0),
			angle: 0.0,
		};

		body.translate_position(position);

		body
	}
	pub fn rectangle(width: Geo, height: Geo, position: Vec2) -> Body {
		let half_width = width / 2.0;
		let half_height = height / 2.0;
		let vertices = vec![
			Vec2::new(-half_width, -half_height), // top left
			Vec2::new( half_width, -half_height), // top right
			Vec2::new( half_width,  half_height), // bottom right
			Vec2::new(-half_width,  half_height), // bottom left
		];
		Body::new(vertices, position)
	}

	// getters
	pub fn get_angle(&self) -> Geo { self.angle }
	pub fn get_position(&self) -> Vec2 { self.position.clone() }
	pub fn get_vertices(&self) -> Vec<Vec2> { self.vertices.clone() }
	pub fn get_velocity(&self) -> Vec2 { self.velocity.clone() }

	// setters
	pub fn translate_position(&mut self, translation: Vec2) {
		self.position += translation;
		
		for vertice in self.vertices.iter_mut() {
			*vertice += translation;
		}
	}
}

impl PartialEq for Body {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
impl Eq for Body {}

impl Hash for Body {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.id.hash(state);
	}
}