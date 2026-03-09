use bevy::prelude::*;
use std::f32::consts::PI;

use super::Body;
use super::ConstraintSolver;

#[derive(Component, Debug)]
pub struct FixedDistance {
	pub body: Entity,
	pub position: Vec2,
	pub length: f32,
	pub position_offset: Vec2,
}
impl Default for FixedDistance {
	fn default() -> Self {
		Self {
			body: Entity::PLACEHOLDER,
			position: Vec2::ZERO,
			length: 100.0,
			position_offset: Vec2::ZERO,
		}
	}
}

impl ConstraintSolver for FixedDistance {
	fn solve_velocity(&self, bodies: &mut Query<&mut Body>, h: f32, iterations: i32) -> Result<(), BevyError> {
		let mut body = bodies.get_mut(self.body)?;

		let radius = self.position_offset.rotate(Vec2::from_angle(body.angle));
		let position = body.position + radius;
		
		let ds = position - self.position;
		let dir = ds.normalize_or(Vec2::new(1.0, 0.0));
		let x1 = ds.length() - self.length;

		let point_velocity = body.velocity + body.angular_velocity * radius.perp();
		let rel_vel = point_velocity.dot(dir);


		let zeta: f32 = 1.0; // damping ratio, zeta
		let omega: f32 = 2.0 * PI * 6.0; // oscillation frequency, omega

		let k = body.mass * omega.powf(2.0);
		let c = 2.0 * body.mass * omega * zeta;

		let gamma = 1.0 / (c + h*k);
		let beta = h*k * gamma;


		let mut impulse = -(rel_vel + beta / h * x1) / (body.inverse_mass + gamma / h);

		impulse /= iterations as f32;

		let p = impulse * dir;
		body.apply_impulse(p, position);

		Ok(())
	}
}