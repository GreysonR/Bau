use crate::{Frame, Geo, Id, PairId, Vec2};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Contact {
	pub vertex: Vec2,

	pub incident: Id, // body that owns vertex
	pub reference: Id, // body that is overlapping with verex
	pub anchor_a: Vec2,
	pub anchor_b: Vec2,
	pub mass_coefficient: Geo,
}

#[derive(Serialize, Clone)]
pub struct CollisionPair {
	pub body_a: Id,
	pub body_b: Id,
	pub frame: Frame,

	pub contacts: Vec<Contact>,

	pub depth: Geo,
	pub normal: Vec2,
	pub tangent: Vec2,
	pub normal_point: Vec2, // only used for debugging

	pub friction: Geo,
	pub restitution: Geo,
}

impl CollisionPair {
	pub fn pair_id(x: Id, y: Id) -> PairId { // returned type must be MORE THAN 2x the bytes of the ID type
		// [elegant pairing](http://szudzik.com/ElegantPairing.pdf)
		let x = x as PairId;
		let y = y as PairId;
		if x > y {
			x * x + x + y
		}
		else {
			y * y + x
		}
	}
	pub fn unpair_id(id: PairId) -> (Id, Id) {
		let z = (id as f64).sqrt() as PairId; // we want id floor'd, so precision loss is correct
		let l = id - z * z; // safety: z is sqrt'd, so squaring cannot overflow
		if l < z { (l as Id, z as Id) } else { (z as Id, (l - z) as Id) }
	}
	pub fn is_valid(&self, frame: Frame) -> bool {
		self.frame >= frame
	}
}
impl std::hash::Hash for CollisionPair {
	fn hash<H>(&self, state: &mut H) where H: std::hash::Hasher {
		CollisionPair::pair_id(self.body_a, self.body_b).hash(state);
	}
}
impl std::cmp::PartialEq for CollisionPair {
	fn eq(&self, other: &CollisionPair) -> bool {
		CollisionPair::pair_id(self.body_a, self.body_b) == CollisionPair::pair_id(other.body_a, other.body_b)
	}
}
impl std::cmp::Eq for CollisionPair {}