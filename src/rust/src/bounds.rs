use wasm_bindgen::prelude::*;

use crate::{Vec2};

#[wasm_bindgen]
#[derive(Clone)]
pub struct Bounds {
	pub min: Vec2,
	pub max: Vec2
}

impl Bounds {
	// Create a new Bounds with a given min and max
	pub fn new(min: Vec2, max: Vec2) -> Self {
		Self {
			min: min,
			max: max,
		}
	}

	// Create a new Bounds with zeroed out min and max
	pub fn empty() -> Self {
		Self {
			min: Vec2::zero(),
			max: Vec2::zero(),
		}
	}

	pub fn from_vertices(vertices: &Vec<Vec2>) -> Self {
		let mut bounds = Bounds::empty();
		for vertex in vertices {
			bounds.expand_to(vertex);
		}

		bounds
	}
	
	// Sets the bounds to the given values
	pub fn set(&mut self, min: &Vec2, max: &Vec2) {
		self.min.x = min.x;
		self.min.y = min.y;
		self.max.x = max.x;
		self.max.y = max.y;
	}

	// Expands the bounds to include point. Doesn't do anything if the point is already within the bounds
	pub fn expand_to(&mut self, point: &Vec2) {
		self.min.x = self.min.x.min(point.x);
		self.min.y = self.min.y.min(point.y);
		self.max.x = self.max.x.max(point.x);
		self.max.y = self.max.y.max(point.y);
	}

	// Updates the current bounds to cover the given vertices
	pub fn update_from_vertices(&mut self, vertices: &Vec<Vec2>) {
		if vertices.len() <= 0 {
			panic!("Bounds::update_from_vertices: vertices.len() must be > 0")
		}

		// Reset bounds
		self.min.x = vertices[0].x;
		self.min.y = vertices[0].y;
		self.max.x = vertices[0].x;
		self.max.y = vertices[0].y;

		// Expand to vertices
		for vertex in vertices {
			self.expand_to(vertex);
		}
	}

	// Determines if the bounds overlap with other_bounds
	pub fn overlaps_with(&self, other_bounds: &Bounds) -> bool {
		   self.min.x <= other_bounds.max.x
		&& self.max.x >= other_bounds.min.x
		&& self.min.y <= other_bounds.max.y
		&& self.max.y >= other_bounds.min.y
	}
}