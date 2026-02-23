use bevy::prelude::*;

mod body;
pub use body::{ Body, BodyBuilder };

mod constraint;
pub use constraint::{ Constraint, Spring, ConstraintSolver, FixedDistance };

#[derive(Resource)]
pub struct Gravity(Vec2);

#[derive(Resource)]
pub struct VelocityIterations(i32);

#[derive(Resource)]
pub struct PositionIterations(i32);

pub struct Engine {
	velocity_iterations: i32,
	position_iterations: i32,
	gravity: Vec2,
}
impl Default for Engine {
	fn default() -> Self {
		Self {
			velocity_iterations: 1,
			position_iterations: 1,
			gravity: Vec2::new(0.0, -1000.0),
		}
	}
}
impl Plugin for Engine {
	fn build(&self, app: &mut App) {
		// Engine globals
		app.insert_resource(VelocityIterations(self.velocity_iterations));
		app.insert_resource(PositionIterations(self.position_iterations));
		app.insert_resource(Gravity(self.gravity));

		app.add_systems(FixedUpdate, (apply_forces, solve_velocity_constraints, solve_position_constraints, apply_impulses).chain()); // TODO: examine FixedUpdate vs Update here
	}
}

// Solves all constraints in the world
fn solve_velocity_constraints(time: Res<Time>, velocity_iterations: Res<VelocityIterations>, constraints: Query<&Constraint>, mut bodies: Query<&mut Body>) {
	let velocity_iterations = velocity_iterations.0;
	let delta = time.delta_secs();
	for _ in 0..velocity_iterations {
		for constraint in constraints {
			match constraint {
				Constraint::Spring(spring) => spring.solve_velocity(&mut bodies, delta),
				Constraint::FixedDistance(constraint) => constraint.solve_velocity(&mut bodies, delta),
			};
		}
	}
}
fn solve_position_constraints(time: Res<Time>, position_iterations: Res<PositionIterations>, constraints: Query<&Constraint>, mut bodies: Query<&mut Body>) {
	let position_iterations = position_iterations.0;
	let delta = time.delta_secs();
	for _ in 0..position_iterations {
		for constraint in constraints {
			match constraint {
				Constraint::Spring(spring) => spring.solve_position(&mut bodies, delta),
				Constraint::FixedDistance(constraint) => constraint.solve_position(&mut bodies, delta),
			};
		}
	}
}

// Apply various simple forces to bodies; i.e. air friction, gravity
fn apply_forces(time: Res<Time>, gravity: Res<Gravity>, bodies: Query<&mut Body>) {
	let delta = time.delta_secs();
	let gravity = gravity.0;

	for mut body in bodies {
		let inverse_mass = body.inverse_mass;
		// Apply air friction
		let friction_air = body.friction_air * body.velocity * body.mass;
		body.velocity -= friction_air * inverse_mass * delta;

		// Apply gravity
		let force_gravity = gravity * body.mass;
		body.velocity += force_gravity * inverse_mass * delta;
	}
}

// Apply accumulated impulses for this frame to bodies
fn apply_impulses(time: Res<Time>, bodies: Query<&mut Body>) {
	let delta = time.delta_secs();
	let now = time.elapsed_secs();
	
	for mut body in bodies {
		if now < 0.5 { // temporarily pause sim at start so everything can load
			body.velocity = Vec2::ZERO; // TODO: this messes up initial velocities
			body.angular_velocity = 0.0;
			continue;
		}

		let delta_position = delta * body.velocity;
		body.translate_position(delta_position);

		let delta_angle = delta * body.angular_velocity;
		body.translate_angle(delta_angle);
	}
}