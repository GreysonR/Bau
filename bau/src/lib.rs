use bevy::prelude::*;

mod body;
pub use body::{ Body, BodyBuilder };

mod constraint;
pub use constraint::{ Constraint, Spring, ConstraintSolver, FixedDistance };

#[derive(Resource)]
pub struct Gravity(Vec2);

pub struct Engine;
impl Default for Engine {
	fn default() -> Self {
		Self {}
	}
}
impl Plugin for Engine {
	fn build(&self, app: &mut App) {
		app.insert_resource(Gravity(Vec2::new(0.0, -1000.0)));
		app.add_systems(FixedUpdate, (apply_forces, solve_velocity_constraints, solve_position_constraints, apply_impulses).chain()); // TODO: examine FixedUpdate vs Update here
	}
}

// Solves all constraints in the world
fn solve_velocity_constraints(time: Res<Time>, constraints: Query<&Constraint>, mut bodies: Query<&mut Body>) {
	let delta = time.delta_secs();
	let iteration_count = 1; // TODO: make this an engine option
	for _ in 0..iteration_count {
		for constraint in constraints {
			match constraint {
				Constraint::Spring(spring) => spring.solve_velocity(&mut bodies, delta),
				Constraint::FixedDistance(constraint) => constraint.solve_velocity(&mut bodies, delta),
			};
		}
	}
}
fn solve_position_constraints(time: Res<Time>, constraints: Query<&Constraint>, mut bodies: Query<&mut Body>) {
	let delta = time.delta_secs();
	let iteration_count = 1; // TODO: make this an engine option
	for _ in 0..iteration_count {
		for constraint in constraints {
			match constraint {
				Constraint::Spring(spring) => spring.solve_position(&mut bodies, delta),
				Constraint::FixedDistance(constraint) => constraint.solve_position(&mut bodies, delta),
			};
		}
	}
}

// Apply various simple forces to bodies; i.e. air friction, gravity
fn apply_forces(gravity: Res<Gravity>, bodies: Query<&mut Body>) {
	let gravity = gravity.0;

	for mut body in bodies {
		// Apply air friction
		let friction_air = body.friction_air * body.velocity * body.mass;
		body.accumulated_impulse -= friction_air;

		// Apply gravity
		let force_gravity = gravity * body.mass;
		body.accumulated_impulse += force_gravity;
	}
}

// Apply accumulated impulses for this frame to bodies
fn apply_impulses(time: Res<Time>, bodies: Query<&mut Body>) {
	let delta = time.delta_secs();
	let now = time.elapsed_secs();
	
	for mut body in bodies {
		if now < 0.5 { // temporarily pause sim at start so everything can load
			body.accumulated_impulse = Vec2::ZERO;
			continue;
		}

		let delta_velocity = delta * body.accumulated_impulse / body.mass;
		body.velocity += delta_velocity;

		let delta_position = delta * body.velocity;
		body.position += delta_position;
		body.accumulated_impulse = Vec2::ZERO;
	}
}