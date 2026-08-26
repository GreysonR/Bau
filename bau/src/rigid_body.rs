use bevy::{ecs::query::QueryData, prelude::*};

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

#[derive(QueryData)]
#[query_data(mutable)]
pub struct RigidBodyQuery {
	pub transform: &'static mut Transform,
	pub velocity: &'static mut Velocity,
	pub angular_velocity: &'static mut AngularVelocity,

	pub mass: &'static Mass,
	pub inertia: &'static Inertia,

	pub friction: &'static Friction,
	pub friction_air: &'static FrictionAir,
	pub friction_angular: &'static FrictionAngular,
}

impl<'w, 's> RigidBodyQueryItem<'w, 's> {
	pub fn get_position(&self) -> Vec2 {
		self.transform.translation.xy()
	}
	pub fn set_position(&mut self, position: Vec2) {
		self.transform.translation = position.extend(self.transform.translation.z);
	}

	pub fn get_velocity(&self) -> Vec2 {
		self.velocity.0
	}
	pub fn set_velocity(&mut self, velocity: Vec2) {
		self.velocity.0 = velocity;
	}

	pub fn get_angular_velocity(&self) -> f32 {
		self.angular_velocity.0
	}
	pub fn set_angular_velocity(&mut self, angular_velocity: f32) {
		self.angular_velocity.0 = angular_velocity;
	}
	// Finds the velocity of the given point on the body, taking into account both linear and angular velocity
	pub fn get_velocity_at_point(&self, point: Vec2) -> Vec2 {
		self.get_velocity() + self.get_angular_velocity() * (point - self.get_position()).perp()
	}

	pub fn get_angle(&self) -> Vec2 {
		self.transform.right().xy()
	}
	pub fn set_angle(&mut self, angle: Vec2) {
		*self.transform = self.transform.with_rotation(Quat::from_rotation_arc_2d(Vec2::X, angle));
	}


	// Applies an impulse at a specified position on the body, which changes its angular & translational velocity
	pub fn apply_impulse(&mut self, impulse_position: Vec2, impulse_velocity: Vec2) {
		let radius = impulse_position - self.get_position();
		let cross = radius.perp_dot(impulse_velocity);

		self.velocity.0 += impulse_velocity * self.mass.inverse();
		self.angular_velocity.0 += cross * self.inertia.inverse();
	}
}

impl<'w, 's> RigidBodyQueryReadOnlyItem<'w, 's> {
	pub fn get_position(&self) -> Vec2 {
		self.transform.translation.xy()
	}

	pub fn get_velocity(&self) -> Vec2 {
		self.velocity.0
	}

	pub fn get_angular_velocity(&self) -> f32 {
		self.angular_velocity.0
	}
	// Finds the velocity of the given point on the body, taking into account both linear and angular velocity
	pub fn get_velocity_at_point(&self, point: Vec2) -> Vec2 {
		self.get_velocity() + self.get_angular_velocity() * (point - self.get_position()).perp()
	}

	pub fn get_angle(&self) -> Vec2 {
		self.transform.right().xy()
	}
}