use crate::{Body, Id, World, Time};
use std::collections::HashMap;

// todo: actually solve velocity constraints
pub fn solve_velocity(world: &mut World, _bodies: &mut HashMap<Id, Body>, _delta: Time) {
	world.collision_pairs.retain(|pair| pair.is_valid(world.frame));

	// for (pair_id, pair) in world.collision_pairs.iter() {
	// }
}