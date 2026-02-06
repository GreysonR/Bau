use std::collections::HashSet;

use crate::{BodyMap, CollisionPair, Frame, Id, Time, Vec2};

pub struct World {
	pub gravity: Vec2,
	pub bodies: HashSet<Id>,
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
	pub fn get_pairs(&self, bodies: &BodyMap) -> Vec<(Id, Id)> {
		let mut pairs = Vec::new();
		let len = self.bodies.len();
		let bodies_vec: Vec<&Id> = self.bodies.iter().collect();

		for i in 0..len - 1 {
			let body_a_id = *bodies_vec[i];
			let body_a = bodies.get(&body_a_id).expect(&format!("Failed to get body_a {body_a_id} in World::get_pairs"));
			for j in i + 1..len {
				let body_b_id = *bodies_vec[j];
				let body_b = bodies.get(&body_b_id).expect(&format!("Failed to get body_b {body_b_id} in World::get_pairs"));
				if body_a.bounds.overlaps_with(&body_b.bounds) {
					pairs.push((body_a_id, body_b_id));
				}
			}
		}

		pairs
	}
}