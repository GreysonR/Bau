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
	fn solve_velocity(&self, bodies: &mut Query<RigidBodyQuery>, h: f32, iterations: i32) -> Result<(), BevyError> {
		if self.body_a.is_none() || self.body_b.is_none() {
			return Ok(()); // return Ok() for now, todo: maybe return error
		}
		let mut body_a;
		let mut body_b;
		if let Ok([a, b]) = bodies.get_many_mut([self.body_a.unwrap(), self.body_b.unwrap()]) {
			body_a = a;
			body_b = b;
		}
		else { return Ok(()); }

		let radius_a = self.body_a_offset.rotate(body_a.get_angle_dir());
		let position_a = body_a.get_position() + radius_a;

		let radius_b = self.body_b_offset.rotate(body_b.get_angle_dir());
		let position_b = body_b.get_position() + radius_b;
		
		let ds = position_b - position_a;
		let dir = ds.normalize_or(Vec2::X);
		let position_error = ds.length() - self.unstretched_length; // constraint-space position
		if self.allow_compression && position_error < 0.0 { return Ok(()); } // don't eval constraint if in compression

		let point_a_velocity = body_a.get_velocity_at_point(position_a);
		let point_b_velocity = body_b.get_velocity_at_point(position_b);
		let rel_vel = (point_b_velocity - point_a_velocity).dot(dir);
		
		let inverse_effective_mass = body_a.mass.inverse() + body_b.mass.inverse() + radius_a.perp_dot(dir) * body_a.inertia.inverse() + radius_b.perp_dot(dir) * body_b.inertia.inverse();
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
		body_a.apply_impulse(position_a, p);
		body_b.apply_impulse(position_a, -p);

		Ok(())
	}
}