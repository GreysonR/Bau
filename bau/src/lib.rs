use bevy::prelude::*;

mod rigid_body;
pub use rigid_body::*;

mod constraint;
pub use constraint::*;

#[derive(Resource, Clone)]
pub struct Engine {
	pub velocity_iterations: i32,
	pub position_iterations: i32,
	pub gravity: Vec2,
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
		// Register constraints
		use bevy_trait_query::RegisterExt;
		app
			.register_component_as::<dyn Constraint, Spring>();
			// .register_component_as::<dyn Constraint, FixedDistance>();

		// Engine globals
		app.add_systems(Update, // TODO: examine FixedUpdate vs Update here
			(
				apply_forces,
				solve_velocity_constraints,
				solve_position_constraints,
				integrate_positions
			)
		);

		app.insert_resource(self.clone());
	}
}

// Solves all constraints in the world
fn solve_velocity_constraints(time: Res<Time>, engine: Res<Engine>, mut commands: Commands, constraints: Query<(Entity, &dyn Constraint)>, mut bodies: Query<RigidBodyQuery>) {
	let velocity_iterations = engine.velocity_iterations;
	let delta = time.delta_secs();

	if time.elapsed_secs() < 0.5 { // temporarily pause sim at start so everything can load
		return;
	}

	for _ in 0..velocity_iterations {
		for (entity, constraints) in constraints {
			for constraint in constraints {
				let _ = constraint.solve_velocity(&mut bodies, delta, velocity_iterations).map_err(|_| {
					// despawn constraint if it's broken
					warn!("Constraint {} had error while solving velocity - despawning", entity);
					commands.entity(entity).try_despawn();
				});
			}
		}
	}
}
fn solve_position_constraints(time: Res<Time>, engine: Res<Engine>, mut commands: Commands, constraints: Query<(Entity, &dyn Constraint)>, mut bodies: Query<RigidBodyQuery>) {
	let position_iterations = engine.position_iterations;
	let delta = time.delta_secs();

	if time.elapsed_secs() < 0.5 { // temporarily pause sim at start so everything can load
		return;
	}

	for _ in 0..position_iterations {
		for (entity, constraints) in constraints {
			for constraint in constraints {
				let _ = constraint.solve_position(&mut bodies, delta, position_iterations).map_err(|_| {
					// despawn constraint if it's broken
					warn!("Constraint {} had error while solving position - despawning", entity);
					commands.entity(entity).try_despawn();
				});
			}
		}
	}
}

// Apply various simple forces to bodies; i.e. air friction, gravity; Only applies to dynamic bodies
fn apply_forces(time: Res<Time>, engine: Res<Engine>, bodies: Query<RigidBodyQuery>) {
	let gravity = engine.gravity;
	let delta = time.delta_secs();

	if time.elapsed_secs() < 0.5 { // temporarily pause sim at start so everything can load
		return;
	}

	for mut body in bodies {
		if let RigidBody::Static = body.body_type  { continue; }

		let friction_air = body.friction_air.0;
		let friction_angular = body.friction_angular.0;

		// Apply air friction
		let friction_air = (1.0 - friction_air).powf(delta * 1000.0); // 1000.0 is arbitrary, used so friction_air doesn't have to be as absurd (0.99999... just to be damped)
		body.velocity.0 *= friction_air;

		let friction_angular = (1.0 - friction_angular).powf(delta * 1000.0);
		body.angular_velocity.0 *= friction_angular;

		// Apply gravity
		body.velocity.0 += gravity * delta; // gravity * mass * inverse_mass * delta
	}
}

// Apply accumulated impulses for this frame to bodies
fn integrate_positions(time: Res<Time>, bodies: Query<RigidBodyQuery>) {
	let delta = time.delta_secs();

	if time.elapsed_secs() < 0.5 { // temporarily pause sim at start so everything can load
		return;
	}

	for mut body in bodies {

		let delta_position = delta * body.velocity.0;
		body.transform.translation += delta_position.extend(0.0);

		let delta_angle = delta * body.angular_velocity.0;
		body.transform.rotate_z(delta_angle);
	}
}