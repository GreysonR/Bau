use std::collections::HashMap;
use nohash_hasher::BuildNoHashHasher;

use crate::{Body, Id, Time, World};
mod collisions;
mod solver;

// todo: add physics options struct

/*
physics update steps:
	Find collisions
		Get all pairs of bodies
		Check if they collide
			Create manifold (collision_pair) if they do
	Apply forces
		Gravity
	Solve velocity constraints
	Solve position constraints
	Update positions / angles
*/
pub fn update(world: &mut World, bodies: &mut HashMap<Id, Body, BuildNoHashHasher<Id>>, delta: Time) {
	collisions::find(world, bodies);
	apply_forces(world, bodies, delta); // applies gravity (and other forces)
	
	// Solve velocities
	let velocity_iterations = 15;
	for _ in 0..velocity_iterations {
		solver::solve_velocity(world, bodies, delta / velocity_iterations as f32);
	}

	// todo: solve position constraints
	apply_velocities(world, bodies, delta);

	// Increment world time
	world.frame += 1;
	world.time += delta;
}

// Applying forces/velocities
fn apply_forces(world: &mut World, bodies: &mut HashMap<Id, Body, BuildNoHashHasher<Id>>, delta: Time) {
	let gravity = &world.gravity * &delta;
	for body_id in world.bodies.iter() {
		let body = bodies.get_mut(body_id).unwrap();
		if body.is_static { continue; } // Don't apply forces to static bodies
		body.apply_velocity(&gravity);
	}
}
fn apply_velocities(world: &mut World, bodies: &mut HashMap<Id, Body, BuildNoHashHasher<Id>>, delta: Time) {
	// todo: use actual delta time
	for body_id in world.bodies.iter() {
		let body = bodies.get_mut(body_id).unwrap();
		if body.is_static { continue; } // Don't move static bodies
		body.translate_position(body.get_velocity() * delta); // todo: average cur velocity with last velocity for trapezoidal approx
		body.translate_angle(body.angular_velocity * delta);
	}
}