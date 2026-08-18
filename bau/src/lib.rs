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
				apply_impulses
			).chain()
		);
		
		app.insert_resource(self.clone());
	}
}

// Solves all constraints in the world
fn solve_velocity_constraints(time: Res<Time>, engine: Res<Engine>, mut commands: Commands, constraints: Query<(Entity, &dyn Constraint)>, mut bodies: Query<&mut RigidBody>) {
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
fn solve_position_constraints(time: Res<Time>, engine: Res<Engine>, mut commands: Commands, constraints: Query<(Entity, &dyn Constraint)>, mut bodies: Query<&mut RigidBody>) {
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
fn apply_forces(time: Res<Time>, engine: Res<Engine>, bodies: Query<(&RigidBody, &mut Velocity, &mut AngularVelocity, &FrictionAir, &FrictionAngular, &Mass)>) {
	let gravity = engine.gravity;
	let delta = time.delta_secs();

	if time.elapsed_secs() < 0.5 { // temporarily pause sim at start so everything can load
		return;
	}

	for (_, mut velocity, mut angular_velocity, friction_air, friction_angular, mass) in bodies {
		let velocity = &mut velocity.0;
		let angular_velocity = &mut angular_velocity.0;
		let friction_air = friction_air.0;
		let friction_angular = friction_angular.0;
		let inverse_mass = mass.get_inverse();
		let mass = mass.get();


		// Apply air friction
		let friction_air = (1.0 - friction_air).powf(delta * 1000.0); // 1000.0 is arbitrary, used so friction_air doesn't have to be as absurd (0.99999... just to be damped)
		*velocity *= friction_air;

		let friction_angular = (1.0 - friction_angular).powf(delta * 1000.0);
		*angular_velocity *= friction_angular;

		// Apply gravity
		let force_gravity = gravity * mass;
		*velocity += force_gravity * inverse_mass * delta;
	}
}

// Apply accumulated impulses for this frame to bodies
fn apply_impulses(time: Res<Time>, bodies: Query<(&RigidBody, &Velocity, &AngularVelocity, &mut Transform)>) {
	let delta = time.delta_secs();

	if time.elapsed_secs() < 0.5 { // temporarily pause sim at start so everything can load
		return;
	}
	
	for (_, velocity, angular_velocity, mut position) in bodies {

		let delta_position = delta * velocity.0;
		position.translation += delta_position.extend(0.0);

		let delta_angle = delta * angular_velocity.0;
		position.rotate_z(delta_angle);
	}
}