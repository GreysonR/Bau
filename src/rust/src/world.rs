use std::collections::{HashMap, HashSet};

use crate::{Body, BodyMap, CollisionPair, Frame, Grid, Id, PairId, Time, Vec2, grid::BucketSize};

pub struct World {
	pub gravity: Vec2,
	pub bodies: HashSet<Id>,
	pub frame: Frame,
	pub time: Time,
	pub collision_pairs: HashSet<CollisionPair>,
	pub grid: Grid,
}

impl World {
	pub fn new(bucket_size: BucketSize) -> Self {
		Self {
			gravity: Vec2::new(0.0, 300.0),
			bodies: HashSet::new(),
			frame: 0,
			time: 0.0,
			collision_pairs: HashSet::new(),
			grid: Grid::new(bucket_size),
		}
	}
	pub fn add_body(&mut self, body: &mut Body) {
		if self.bodies.contains(&body.id) { return; }
		self.bodies.insert(body.id);
		self.grid.insert_body(body);
	}
	pub fn remove_body(&mut self, body: &mut Body) {
		self.bodies.remove(&body.id);
		self.grid.remove_body(body);
	}
	pub fn find_pairs(&self, bodies: &BodyMap) -> Vec<(Id, Id)> {
		/*
		- Iterate through all grid buckets
		- Std pairing algo for each bucket
		- Merge all pairs together; Shouldn't be any dupes (can use pair ID with hashmap / set to check)
		*/
		let mut pairs: HashMap<PairId, (Id, Id)> = HashMap::new();

		for (_bucket_id, bucket) in self.grid.get_bucket_iter() {
			self.pair_bucket(bucket, &mut pairs, &bodies);
		}

		pairs.into_iter()
			.map(|pair| pair.1)
			.collect()
	}
	fn pair_bucket(&self, bucket: &Vec<Id>, pairs: &mut HashMap<PairId, (Id, Id)>, bodies: &BodyMap) {
		let len = bucket.len();

		'outer: for i in 0..len - 1 {
			let body_a_id = bucket[i];
			crate::console_log!("body_a: {}", body_a_id);
			let body_a = bodies.get(&body_a_id).expect(&format!("Failed to get body_a {body_a_id} in World::get_pairs"));
			for j in i + 1..len {
				let body_b_id = bucket[j];
				crate::console_log!("body_b: {}", body_b_id);
				if pairs.contains_key(&CollisionPair::pair_id(body_a_id, body_b_id)) { continue 'outer; } // already in pairs

				let body_b = bodies.get(&body_b_id).expect(&format!("Failed to get body_b {body_b_id} in World::get_pairs"));
				if body_a.bounds.overlaps_with(&body_b.bounds) {
					pairs.insert(CollisionPair::pair_id(body_a_id, body_b_id), (body_a_id, body_b_id));
				}
			}
		}
	}

	pub fn update_grid(&mut self, bodies: &mut BodyMap) {
		for (_id, body) in bodies.iter_mut() {
			if body.is_static { continue; }
			self.grid.update_body(body);
		}
	}

	pub fn get_buckets(&self) -> (BucketSize, &crate::grid::GridHashMap) {
		(self.grid.bucket_size, &self.grid.buckets)
	}
}