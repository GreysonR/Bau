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
#[require(Transform, Velocity, AngularVelocity, Friction, FrictionAir, FrictionAngular)] // Mass and Inertia are handled in setup fn
pub enum RigidBody {
	Static,
	Dynamic,
}
pub fn setup_rigid_body_mass(mut commands: Commands, rigid_body_query: Query<(Entity, &RigidBody), (Added<RigidBody>, Without<Mass>)>) {

	for rigid_body in &rigid_body_query {
		let mass = match &rigid_body.1 {
			RigidBody::Dynamic => Mass::default(),
			RigidBody::Static => Mass::infinite(),
		};
		commands.entity(rigid_body.0)
			.insert(mass);
	}
}
pub fn setup_rigid_body_inertia(mut commands: Commands, rigid_body_query: Query<(Entity, &RigidBody), (Added<RigidBody>, Without<Inertia>)>) {

	for rigid_body in &rigid_body_query {
		let inertia = match &rigid_body.1 {
			RigidBody::Dynamic => Inertia::default(),
			RigidBody::Static => Inertia::infinite(),
		};
		commands.entity(rigid_body.0)
			.insert(inertia);
	}
}


#[derive(QueryData)]
#[query_data(mutable)]
pub struct RigidBodyQuery {
	pub body_type: &'static RigidBody,

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

	pub fn get_angle_dir(&self) -> Vec2 {
		self.transform.right().xy()
	}
	pub fn set_angle_dir(&mut self, angle: Vec2) {
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