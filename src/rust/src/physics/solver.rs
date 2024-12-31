use crate::{Body, Id, World, Time};
use std::collections::HashMap;

// todo: actually solve velocity constraints
pub fn solve_velocity(world: &mut World, bodies: &mut HashMap<Id, Body>, _delta: Time) {
	// Clear old collision pairs
	world.collision_pairs.retain(|pair| pair.is_valid(world.frame)); // todo: move this to main loop to make more efficient

	/*
		For all collision pairs
			For all collision contacts
				Get relative velocity at that point
				Solve contraint rel vel >= 0 by applying velocity * impulse
				Update final vel / angular vel for 
	*/

	for pair in world.collision_pairs.iter() {
		let body_a = bodies.get_mut(&pair.body_a).expect("failed to unwrap body_a in solve_velocity");
		let body_b = bodies.get_mut(&pair.body_b).expect("failed to unwrap body_b in solve_velocity");
		let normal = &pair.normal;
		let contacts = &pair.contacts;
		
		for contact in contacts.iter() {

		}
	}
}