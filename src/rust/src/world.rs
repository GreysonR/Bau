use std::collections::HashSet;

use crate::{Id, Vec2, Frame, Time, CollisionPair};

pub struct World {
	pub gravity: Vec2,
	pub bodies: HashSet<Id>, // maybe use a hashset instead?
	pub frame: Frame,
	pub time: Time,
	pub collision_pairs: HashSet<CollisionPair>,
}

impl World {
	pub fn new() -> Self {
		Self {
			gravity: Vec2::new(0.0, 300.0),
			bodies: HashSet::new(),
			frame: 0,
			time: 0.0,
			collision_pairs: HashSet::new(),
		}
	}
	pub fn add_body(&mut self, body: Id) {
		if self.bodies.contains(&body) { return; } // SLOW: O(n)
		self.bodies.insert(body);
	}
	pub fn remove_body(&mut self, id: Id) {
		self.bodies.remove(&id);
	}
	pub fn get_pairs(&self) -> Vec<(Id, Id)> {
		let mut pairs = Vec::new();
		let len = self.bodies.len();
		let bodies_vec: Vec<&Id> = self.bodies.iter().collect();

		for i in 0..len - 1 {
			let body_a = *bodies_vec[i];
			for j in i + 1..len {
				let body_b = *bodies_vec[j];
				// todo: AABB collision test before pushing the pair
				pairs.push((body_a, body_b));
			}
		}

		pairs
	}
}