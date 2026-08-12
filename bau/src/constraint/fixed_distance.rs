use bevy::prelude::*;
use std::f32::consts::PI;

use super::Body;
use super::ConstraintSolver;

#[derive(Component, Debug)]
pub struct FixedDistance {
	pub body_a: Option<Entity>,
	pub body_a_offset: Vec2,

	pub body_b: Option<Entity>,
	pub body_b_offset: Vec2,

	pub length: f32,
}
impl Default for FixedDistance {
	fn default() -> Self {
		Self {
			body_a: None,
			body_a_offset: Vec2::ZERO,

			body_b: None,
			body_b_offset: Vec2::ZERO,

			length: 100.0,
		}
	}
}

impl ConstraintSolver for FixedDistance {
	fn solve_velocity(&self, bodies: &mut Query<&mut Body>, h: f32, iterations: i32) -> Result<(), BevyError> {
		if self.body_a.is_none() || self.body_b.is_none() {
			return Ok(()); // Ok() for now; todo: maybe throw error
		}
		let mut body_a;
		let mut body_b;
		if let Ok([a, b]) = bodies.get_many_mut([self.body_a.unwrap(), self.body_b.unwrap()]) {
			body_a = a;
			body_b = b;
		}
		else { return Ok(()); }
		
		if body_a.is_static && body_b.is_static {
			return Ok(()); // don't do anything if both are static
		}

		let radius_a = self.body_a_offset.rotate(Vec2::from_angle(body_a.angle));
		let position_a = body_a.position + radius_a;

		let radius_b = self.body_b_offset.rotate(Vec2::from_angle(body_b.angle));
		let position_b = body_b.position + radius_b;
		
		let ds = position_b - position_a;
		let dir = ds.normalize_or(Vec2::X);
		let position_error = ds.length() - self.length; // constraint-space position

		let point_a_velocity = body_a.velocity + body_a.angular_velocity * radius_a.perp();
		let point_b_velocity = body_b.velocity + body_b.angular_velocity * radius_b.perp();
		let rel_vel = (point_b_velocity - point_a_velocity).dot(dir);
		
		let inverse_effective_mass = body_a.inverse_mass + body_b.inverse_mass + radius_a.perp_dot(dir) * body_a.inverse_inertia + radius_b.perp_dot(dir) * body_b.inverse_inertia;
		let effective_mass = 1.0 / inverse_effective_mass;


		// Calculate soft constraint parameters
		let zeta: f32 = 0.2; // damping ratio, zeta; value is arbitrary, chosen to behave nicely
		let omega: f32 = 2.0 * PI * 6.0; // oscillation frequency, omega; value is arbitrary, chosen to behave nicely

		let k = effective_mass * omega.powf(2.0);
		let c = 2.0 * effective_mass * omega * zeta;

		let gamma = 1.0 / (c + h*k);
		let beta = h*k * gamma;

		// Soft constraint impulse equation
		let mut impulse = -(rel_vel + beta / h * position_error) / (inverse_effective_mass + gamma / h);
		impulse /= iterations as f32;


		// Apply impulses
		let p = -impulse * dir;
		body_a.apply_impulse(p, position_a);
		body_b.apply_impulse(-p, position_b);

		Ok(())
	}
}