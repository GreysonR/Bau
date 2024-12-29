use std::collections::HashMap;
use crate::{World, Body, Id, Time};

// todo: add physics options struct

pub fn update(world: &mut World, bodies: &mut HashMap<Id, Body>, delta: Time) {
	// apply forces
	apply_forces(world, bodies, delta);

	// apply velocities
	apply_velocities(world, bodies, delta);
}
fn apply_forces(world: &mut World, bodies: &mut HashMap<Id, Body>, delta: Time) {
	let gravity = world.gravity * &delta;
	for body_id in world.bodies.iter() {
		let body = bodies.get_mut(body_id).unwrap();
		if body.is_static { continue; } // Don't apply forces to static bodies
		body.apply_velocity(&gravity);
	}
}
fn apply_velocities(world: &mut World, bodies: &mut HashMap<Id, Body>, delta: Time) {
	// todo: use actual delta time
	for body_id in world.bodies.iter() {
		let body = bodies.get_mut(body_id).unwrap();
		if body.is_static { continue; } // Don't move static bodies
		body.translate_position(*body.get_velocity() * &delta); // todo: average cur velocity with last velocity for trapezoidal approx
	}

	world.frame += 1;
	world.time += delta;
}