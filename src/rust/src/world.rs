use crate::{Id, Vec2};

pub struct World {
	pub gravity: Vec2,
	pub bodies: Vec<Id>, // maybe use a hashset instead?
}

impl World {
	pub fn new() -> Self {
		Self {
			gravity: Vec2::new(0.0, 2.0),
			bodies: Vec::new(),
		}
	}
	pub fn add_body(&mut self, body: Id) {
		if self.bodies.contains(&body) { return; } // SLOW: O(n)
		self.bodies.push(body);
	}
	pub fn remove_body(&mut self, id: Id) {
		self.bodies.retain(|body| *body != id); // SLOW: O(n)
	}
}