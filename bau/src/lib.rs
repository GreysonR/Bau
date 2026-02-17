use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Body {
	pub position: Vec2,
	pub velocity: Vec2,
	pub accumulated_impulse: Vec2,
	pub mass: f32,
	pub friction_air: f32,
}
impl Default for Body {
	fn default() -> Self {
		Self {
			position: Vec2::ZERO,
			velocity: Vec2::ZERO,
			accumulated_impulse: Vec2::ZERO,
			mass: 1.0,
			friction_air: 0.5,
		}
	}
}

#[derive(Component)]
pub enum Constraint {
	Spring(Spring),
}

#[derive(Component, Debug)]
pub struct Spring {
	pub body: Entity,
	pub position: Vec2,
	pub length: f32,
	pub stiffness: f32,
	pub damping: f32,
}
impl Default for Spring {
	fn default() -> Self {
		Self {
			body: Entity::PLACEHOLDER,
			position: Vec2::ZERO,
			length: 100.0,
			stiffness: 100.0,
			damping: 2.0,
		}
	}
}

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
		app.add_systems(Update, ((solve_constraints, apply_body_forces), apply_impulses).chain()); // TODO: examine FixedUpdate vs Update here
	}
}

// Main physics loop; Solves all constraints in the engine
fn solve_constraints(constraints: Query<&Constraint>, mut bodies: Query<&mut Body>) {
	for constraint in constraints {
		match constraint {
			Constraint::Spring(spring) => {
				let mut body = bodies.get_mut(spring.body).expect("body should be in world"); // TODO: handle unwrap
				let ds = spring.position - body.position;
				let spring_dir = ds.normalize_or(Vec2::new(1.0, 0.0));
				let rel_vel = body.velocity.dot(spring_dir);
				let mut impulse = (ds.length() - spring.length) * spring.stiffness;
				impulse -= spring.damping * rel_vel;

				body.accumulated_impulse += impulse * spring_dir;
			},
		};
	}
}

// Apply various forces to bodies; i.e. friction, gravity
fn apply_body_forces(gravity: Res<Gravity>, bodies: Query<&mut Body>) {
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

// Apply accumulated impulses for this frame
fn apply_impulses(time: Res<Time>, bodies: Query<&mut Body>) {
	let delta = time.delta_secs();
	let now = time.elapsed_secs();
	// println!("fps: {}; delta: {}", 1.0 / delta, delta);
	
	for mut body in bodies {
		if now < 0.5 { // temporarily pause sim
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