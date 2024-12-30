use crate::{Body, CollisionPair, Geo, Id, Vec2, World};
use std::collections::HashMap;

pub fn find(world: &mut World, bodies: &mut HashMap<Id, Body>) {
	let pairs = world.get_pairs();
	for pair in pairs {
		let body_a = bodies.get(&pair.0).unwrap();
		let body_b = bodies.get(&pair.1).unwrap();
		if !collides(body_a, body_b) { continue; }

		// Create manifold (collision_pair) if they collide and add to world pairs (if it doesn't already exist)
		// let pair_id = CollisionPair::pair_id(body_a.id, body_b.id);
		world.collision_pairs.replace(CollisionPair {
			body_a: body_a.id,
			body_b: body_b.id,
			frame: world.frame,
		});
	}
}
fn collides(body_a: &Body, body_b: &Body) -> bool {
	// SAT: check if 1d projection has a collision for every axis of each body
	let find_supports = |body: &Body, direction: &Vec2| {
		let mut min = Geo::MAX;
		let mut max = Geo::MIN;
		for vertex in body.get_vertices().iter() {
			let proj = vertex.dot(*direction);
			if proj < min { min = proj };
			if proj > max { max = proj };
		}
		(min, max)
	};
	let collides_against = |body_a: &Body, body_b: &Body| { // Checks body_a axes against body_b
		for axis in body_a.axes.iter() {
			let support_dir = axis.clone().normal();
			let a = find_supports(body_a, &support_dir);
			let b = find_supports(body_b, &support_dir);
			if a.1 < b.0 || a.0 > b.1 { // bounds aren't overlapping
				return false;
			}
		}
		true
	};
	collides_against(body_a, body_b) && collides_against(body_b, body_a)
}