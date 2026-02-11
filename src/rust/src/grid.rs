use std::collections::{HashMap, hash_map::Iter};

use crate::{Body, Geo, Id, Vec2};

pub type GridPosition = i32; // grid positions
pub type GridPairId = u64; // paired grid positions; should be u128, but that doesn't work with hasher :(
pub type BucketSize = Geo;
pub type GridHashMap = HashMap<GridPairId, Vec<Id>>;


// watered-down implementations with correct types
struct GridVec {
	x: GridPosition,
	y: GridPosition
}
impl GridVec {
	pub fn new(x: GridPosition, y: GridPosition) -> Self {
		Self { x, y }
	}
}
struct GridBounds {
	min: GridVec,
	max: GridVec,
}
impl GridBounds {
	pub fn new(min: GridVec, max: GridVec) -> Self {
		Self { min, max }
	}
}

pub struct Grid {
	pub bucket_size: BucketSize,
	pub buckets: GridHashMap,
}

impl Grid {
	pub fn new(bucket_size: BucketSize) -> Self {
		Self {
			bucket_size,
			buckets: GridHashMap::default()
		}
	}

	pub fn get_bucket_iter(&self) -> Iter<'_, u64, Vec<u16>> {
		self.buckets.iter()
	}

	fn pair(point: &GridVec) -> GridPairId { // elegant pairing, modified to work with negative numbers
		let x: GridPairId = if point.x >= 0 { point.x * 2 } else { point.x * -2 - 1 } as GridPairId; // safety: result is always positive, and GridPairId is larger type
		let y: GridPairId = if point.y >= 0 { point.y * 2 } else { point.y * -2 - 1 } as GridPairId;
		if x >= y {
			x * x + x + y // safety: bc we upcasted from i32, we will always have enough space in u128 for operation
		}
		else {
			y * y + x
		}
	}
	#[allow(unused)]
	fn unpair(id: GridPairId) -> GridVec { // elegant unpairing, modified to work with negative numbers. DO NOT LOOK INSIDE UNDER ANY CIRCUMSTANCES. This method is only for debugging anyways
		// safety: pray with all your might it doesn't lose precision bc I'm not dealing with ts
		// Unless otherwise noted, the above safety note applies to every line in this function, and you must say a unique, increasingly passionate prayer for each line.
		let sqrtz = f64::sqrt(id as f64) as GridPairId;
		let sqz = sqrtz * sqrtz;
		let result1 = if (id - sqz) >= sqrtz {
			GridVec::new(sqrtz as GridPosition, (id - sqz - sqrtz) as GridPosition) // good lord
		} else {
			GridVec::new((id - sqz) as GridPosition, sqrtz as GridPosition)
		};
		let x = if result1.x % 2 == 0 { result1.x / 2 } else { (result1.x + 1) / -2 };
		let y = if result1.y % 2 == 0 { result1.y / 2 } else { (result1.y + 1) / -2 };
		GridVec::new(x, y)
	}

	fn to_grid_space(&self, point: &Vec2) -> GridVec {
		GridVec::new((point.x / self.bucket_size) as GridPosition, (point.y / self.bucket_size) as GridPosition)
	}
	pub fn get_bucket_from_world_pos(&mut self, position: Vec2) -> &mut Vec<Id> {
		let grid_space = self.to_grid_space(&position);
		let id = Grid::pair(&grid_space);
		self.get_bucket(id)
	}
	fn get_bucket(&mut self, bucket_id: GridPairId) -> &mut Vec<Id> {
		if !self.buckets.contains_key(&bucket_id) {
			self.buckets.insert(bucket_id, Vec::new());
		}
		self.buckets.get_mut(&bucket_id).unwrap() // already know it contains key; unwrap is fine
	}
	pub fn insert_body(&mut self, body: &mut Body) {
		let grid_bounds = GridBounds::new(
			self.to_grid_space(&body.bounds.min),
			self.to_grid_space(&body.bounds.max)
		);

		if body.grid_spaces.len() > 0 {
			panic!("body shouldn't be already in the grid before inserting"); // todo: not panic
		}
		// crate::console_log!("-- Inserting body {} --", body.id);
		// crate::console_log!("grid bounds: {{ min: {}, {}; max: {}, {} }}", grid_bounds.min.x, grid_bounds.min.y, grid_bounds.max.x, grid_bounds.max.y);

		for y in grid_bounds.min.y..=grid_bounds.max.y {
			for x in grid_bounds.min.x..=grid_bounds.max.x {
				let bucket_id = Grid::pair(&GridVec::new(x, y));
				let bucket = self.get_bucket(bucket_id);
				bucket.push(body.id);
				body.grid_spaces.push(bucket_id);
			}
		}
	}
	pub fn remove_body(&mut self, body: &mut Body) {
		// crate::console_log!("-- Removing body {} --", body.id);
		// crate::console_log!("grid cells: {:?}", body.grid_spaces.iter().map(|x| Grid::unpair(*x)));

		let body_id = body.id;
		for bucket_id in body.grid_spaces.iter() {
			let bucket = self.get_bucket(*bucket_id);
			let index = bucket.iter().position(|x| *x == body_id).unwrap();
			bucket.remove(index);

			if bucket.len() == 0 {
				self.buckets.remove(bucket_id);
			}
		}
		body.grid_spaces.clear();
	}
	pub fn update_body(&mut self, body: &mut Body) {
		// todo: optimize this
		self.remove_body(body);
		self.insert_body(body);
	}
}