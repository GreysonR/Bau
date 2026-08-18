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

/*
impl RigidBody {
	// Finds the velocity of the given point on the body, taking into account both linear and angular velocity
	pub fn get_velocity_at_point(&self, point: Vec2) -> Vec2 {
		self.get_velocity() + self.get_angular_velocity() * (point - self.position).perp()
	}
}
*/
