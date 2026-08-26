use bevy::prelude::*;
use std::f32::consts::PI;

use super::{ Constraint, constraint_options::* };
use crate::rigid_body::*;

mod spring_options;
use spring_options::*;


#[derive(Component, Debug)]
#[require(BodyA, BodyB, UnstretchedLength, Frequency, Damping, AllowCompression)]
pub struct Spring {
	pub body_a: Option<Entity>,
	pub body_a_offset: Vec2,

	pub body_b: Option<Entity>,
	pub body_b_offset: Vec2,
	
	pub unstretched_length: f32,
	pub frequency: f32,
	pub damping: f32,

	pub allow_compression: bool,
}
impl Default for Spring {
	fn default() -> Self {
		Self {
			body_a: None, // todo: use options instead
			body_a_offset: Vec2::ZERO,

			body_b: None,
			body_b_offset: Vec2::ZERO,

			unstretched_length: 100.0,
			frequency: 5.0,
			damping: 0.1,

			allow_compression: true,
		}
	}
}

impl Constraint for Spring {
	fn solve_velocity(&self, bodies: &mut Query<(&RigidBody, &Transform, &mut Velocity, &mut AngularVelocity, &Mass, &Inertia)>, h: f32, iterations: i32) -> Result<(), BevyError> {
		if self.body_a.is_none() || self.body_b.is_none() {
			return Ok(()); // return Ok() for now, todo: maybe return error
		}
		let body_a;
		let body_b;
		if let Ok([a, b]) = bodies.get_many_mut([self.body_a.unwrap(), self.body_b.unwrap()]) {
			body_a = a;
			body_b = b;
		}
		else { return Ok(()); }

		let (_, transform_a, mut velocity_a, mut angular_velocity_a, mass_a, inertia_a) = body_a;
		let position_body_a = transform_a.translation.xy();
		let rotation_a = transform_a.right().xy();
		let (_, transform_b, mut velocity_b, mut angular_velocity_b, mass_b, inertia_b) = body_b;
		let position_body_b = transform_b.translation.xy();
		let rotation_b = transform_b.right().xy();

		let radius_a = self.body_a_offset.rotate(rotation_a);
		let position_a = position_body_a + radius_a;

		let radius_b = self.body_b_offset.rotate(rotation_b);
		let position_b = position_body_b + radius_b;
		
		let ds = position_b - position_a;
		let dir = ds.normalize_or(Vec2::X);
		let position_error = ds.length() - self.unstretched_length; // constraint-space position
		if self.allow_compression && position_error < 0.0 { return Ok(()); } // don't eval constraint if in compression

		let point_a_velocity = RigidBody::get_velocity_at_point(transform_a, &velocity_a, &angular_velocity_a, position_a);
		let point_b_velocity = RigidBody::get_velocity_at_point(transform_b, &velocity_b, &angular_velocity_b, position_b);
		let rel_vel = (point_b_velocity - point_a_velocity).dot(dir);
		
		let inverse_effective_mass = mass_a.inverse() + mass_b.inverse() + radius_a.perp_dot(dir) * inertia_a.inverse() + radius_b.perp_dot(dir) * inertia_b.inverse();
		let effective_mass = 1.0 / inverse_effective_mass;


		// Calculate soft constraint parameters
		let zeta: f32 = self.damping; // damping ratio, zeta
		let omega: f32 = 2.0 * PI * self.frequency; // oscillation frequency, omega

		let k = effective_mass * omega.powf(2.0);
		let c = 2.0 * effective_mass * omega * zeta;

		let gamma = 1.0 / (c + h*k);
		let beta = h*k * gamma;

		// Soft constraint impulse equation
		let mut impulse = -(rel_vel + beta / h * position_error) / (inverse_effective_mass + gamma / h);
		impulse /= iterations as f32;


		// Apply impulses
		let p = -impulse * dir;
		RigidBody::apply_impulse(transform_a, &mut velocity_a, &mut angular_velocity_a, mass_a, inertia_a, position_a, p);
		RigidBody::apply_impulse(transform_b, &mut velocity_b, &mut angular_velocity_b, mass_b, inertia_b, position_b, -p);

		Ok(())
	}
}