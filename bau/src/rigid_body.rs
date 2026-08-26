use bevy::prelude::*;

mod vertices;
pub use vertices::*;

mod mass_components;
pub use mass_components::*;

mod friction_components;
pub use friction_components::*;

mod velocity_components;
pub use velocity_components::*;


#[derive(Component, Debug)]
#[require(Transform, Velocity, AngularVelocity, Mass, Inertia, Friction, FrictionAir, FrictionAngular)]
pub enum RigidBody {
	Static,
	Dynamic,
}


impl RigidBody {
	// Finds the velocity of the given point on the body, taking into account both linear and angular velocity
	pub fn get_velocity_at_point(
		position: &Transform, velocity: &Velocity, angular_velocity: &AngularVelocity,
		point: Vec2
	) -> Vec2 {
		velocity.0 + angular_velocity.0 * (point - position.translation.xy()).perp()
	}
	pub fn apply_impulse(
		body_position: &Transform, velocity: &mut Velocity, angular_velocity: &mut AngularVelocity, mass: &Mass, inertia: &Inertia,
		impulse_position: Vec2, impulse_velocity: Vec2
	) {
		let radius = impulse_position - body_position.translation.xy();
		let cross = radius.perp_dot(impulse_velocity);

		velocity.0 += impulse_velocity * mass.inverse();
		angular_velocity.0 += cross * inertia.inverse();
	}
}
