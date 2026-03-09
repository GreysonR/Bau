use bevy::prelude::*;

mod body;
pub use body::{ Body, BodyBuilder };

mod constraint;
pub use constraint::{ Constraint, Spring, ConstraintSolver, FixedDistance };

#[derive(Resource, Clone)]
pub struct Engine {
	pub velocity_iterations: i32,
	pub position_iterations: i32,
	pub gravity: Vec2,
}
impl Default for Engine {
	fn default() -> Self {
		Self {
			velocity_iterations: 10,
			position_iterations: 1,
			gravity: Vec2::new(0.0, -1000.0),
		}
	}
}
impl Plugin for Engine {
	fn build(&self, app: &mut App) {
		// Engine globals
		app.add_systems(Update, (apply_forces, solve_velocity_constraints, solve_position_constraints, apply_impulses).chain()); // TODO: examine FixedUpdate vs Update here
		
		app.insert_resource(self.clone());
	}
}

// Solves all constraints in the world
fn solve_velocity_constraints(time: Res<Time>, engine: Res<Engine>, mut commands: Commands, constraints: Query<(Entity, &Constraint)>, mut bodies: Query<&mut Body>) {
	let velocity_iterations = engine.velocity_iterations;
	let delta = time.delta_secs();

	if time.elapsed_secs() < 0.5 { // temporarily pause sim at start so everything can load
		return;
	}

	for _ in 0..velocity_iterations {
		for (entity, constraint) in constraints {
			let _ = constraint.solve_velocity(&mut bodies, delta, velocity_iterations).map_err(|_| {
				// despawn constraint if it's broken
				warn!("Constraint {} had error while solving - despawning", entity);
				commands.entity(entity).try_despawn();
			});
		}
	}
}
fn solve_position_constraints(time: Res<Time>, engine: Res<Engine>, constraints: Query<&Constraint>, mut bodies: Query<&mut Body>) {
	let position_iterations = engine.position_iterations;
	let delta = time.delta_secs();
	
	if time.elapsed_secs() < 0.5 { // temporarily pause sim at start so everything can load
		return;
	}

	for _ in 0..position_iterations {
		for constraint in constraints {
			constraint.solve_position(&mut bodies, delta, position_iterations);
		}
	}
}

// Apply various simple forces to bodies; i.e. air friction, gravity
fn apply_forces(time: Res<Time>, engine: Res<Engine>, bodies: Query<&mut Body>) {
	let gravity = engine.gravity;
	let delta = time.delta_secs();

	if time.elapsed_secs() < 0.5 { // temporarily pause sim at start so everything can load
		return;
	}

	for mut body in bodies {
		let inverse_mass = body.inverse_mass;
		// Apply air friction
		let friction_air = (1.0 - body.friction_air).powf(delta * 10.0); // * 10.0 so friction_air doesn't have to be as absurd (0.99999... just to be damped)
		body.velocity *= friction_air;

		let friction_angular = (1.0 - body.friction_angular).powf(delta * 10.0);
		body.angular_velocity *= friction_angular;

		// Apply gravity
		let force_gravity = gravity * body.mass;
		body.velocity += force_gravity * inverse_mass * delta;
	}
}

// Apply accumulated impulses for this frame to bodies
fn apply_impulses(time: Res<Time>, bodies: Query<&mut Body>) {
	let delta = time.delta_secs();

	if time.elapsed_secs() < 0.5 { // temporarily pause sim at start so everything can load
		return;
	}
	
	for mut body in bodies {

		let delta_position = delta * body.velocity;
		body.translate_position(delta_position);

		let delta_angle = delta * body.angular_velocity;
		body.translate_angle(delta_angle);
	}
}