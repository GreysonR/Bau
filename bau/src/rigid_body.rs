use bevy::{ecs::query::QueryData, prelude::*};

mod vertices;
pub use vertices::*;

mod mass_components;
pub use mass_components::*;

mod friction_components;
pub use friction_components::*;

mod velocity_components;
pub use velocity_components::*;

mod collider;
pub use collider::*;

#[derive(Component, Debug)]
#[require(Transform, Velocity, AngularVelocity, Friction, FrictionAir, FrictionAngular, Angle, Collider)] // Mass and Inertia are handled in setup fn
pub enum RigidBody {
	Static,
	Dynamic,
}
pub fn setup_rigid_body_mass(mut commands: Commands, rigid_body_query: Query<(Entity, &RigidBody, &Collider), (Added<RigidBody>, Without<Mass>)>) {

	for (entity, rigid_body, collider) in &rigid_body_query {
		let mass = match &rigid_body {
			RigidBody::Dynamic => Mass::new(collider.get_area() * 0.01), // 1 unit mass per 10x10 area
			RigidBody::Static => Mass::infinite(),
		};
		commands.entity(entity)
			.insert(mass);
	}
}
pub fn setup_rigid_body_inertia(mut commands: Commands, rigid_body_query: Query<(Entity, &RigidBody, &Collider, &Mass), (Added<RigidBody>, Without<Inertia>)>) {

	for (entity, rigid_body, collider, mass) in &rigid_body_query {
		let inertia = match &rigid_body {
			RigidBody::Dynamic => Inertia::new(collider.get_inertia(mass.value())),
			RigidBody::Static => Inertia::infinite(),
		};
		commands.entity(entity)
			.insert(inertia);
	}
}


#[derive(QueryData)]
#[query_data(mutable)]
pub struct RigidBodyQuery {
	pub body_type: &'static RigidBody,

	pub transform: &'static mut Transform,
	pub angle: &'static mut Angle,
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
		*self.transform = self.transform.with_translation(position.extend(self.transform.translation.z))
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
	pub fn get_velocity_at_radius(&self, radius: Vec2) -> Vec2 {
		self.get_velocity() + self.get_angular_velocity() * radius.perp()
	}

	pub fn get_angle(&self) -> f32 {
		// self.transform.rotation.to_axis_angle().1 // Vec3 returned should be the z axis
		self.angle.0
	}
	pub fn get_angle_dir(&self) -> Vec2 {
		Vec2::from_angle(self.get_angle())
	}
	pub fn set_angle_dir(&mut self, angle: Vec2) {
		// *self.transform = self.transform.with_rotation(Quat::from_rotation_arc_2d(Vec2::X, angle));
		self.set_angle(angle.to_angle());
	}
	pub fn set_angle(&mut self, angle: f32) {
		self.angle.0 = angle;
	}


	// Applies an impulse at a specified position on the body, which changes its angular & translational velocity
	pub fn apply_impulse(&mut self, impulse_radius: Vec2, impulse_velocity: Vec2) {
		// let radius = impulse_radius - self.get_position();
		let cross = impulse_radius.perp_dot(impulse_velocity);

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

	pub fn get_angle(&self) -> f32 {
		// self.transform.rotation.to_axis_angle().1 // Vec3 returned should be the z axis
		self.angle.0
	}
	pub fn get_angle_dir(&self) -> Vec2 {
		Vec2::from_angle(self.get_angle())
	}
}

pub fn post_update_rigid_body(rigid_bodies: Query<RigidBodyQuery>) {
	for mut rigid_body in rigid_bodies {
		rigid_body.transform.rotation = Quat::from_rotation_arc_2d(Vec2::X, Vec2::from_angle(rigid_body.angle.0));
	}
}