use bevy::prelude::*;

use super::Body;
use super::ConstraintSolver;

#[derive(Component, Debug)]
pub struct Spring {
	pub body: Entity,
	pub position: Vec2,
	
	pub length: f32,
	pub frequency: f32,
	pub damping: f32,
	pub stiffness: f32,

	pub position_offset: Vec2,
	pub constrain_compression: bool,
}
impl Default for Spring {
	fn default() -> Self {
		Self {
			body: Entity::PLACEHOLDER,
			position: Vec2::ZERO,

			length: 100.0,
			frequency: 5.0,
			damping: 0.1,
			stiffness: 50.0,

			position_offset: Vec2::ZERO,
			constrain_compression: true,
		}
	}
}

#[allow(unused)]
impl Spring {
	fn solve_stiff(&self, bodies: &mut Query<&mut Body>, _delta_time: f32, iterations: i32) {
		let mut body = bodies.get_mut(self.body).expect("body should be in world"); // TODO: handle unwrap
		
		let radius = self.position_offset.rotate(Vec2::from_angle(body.angle));
		let body_position = body.position + radius;
		let ds = body_position - self.position;
		let dir = ds.normalize_or(Vec2::new(1.0, 0.0));

		let stiffness: f32 = self.stiffness;
		
		let point_velocity = body.velocity + body.angular_velocity * radius.perp();
		let rel_vel = point_velocity.dot(dir);
		let x1 = ds.length() - self.length;
		if !self.constrain_compression && x1 < 0.0 { return; } // don't eval constraint if less than max length

		let mut impulse = -x1 * stiffness;
		impulse -= self.damping * rel_vel;
		impulse /= iterations as f32;

		let p = impulse * dir;
		body.apply_impulse(p, body_position);
	}
	fn solve_soft(&self, bodies: &mut Query<&mut Body>, h: f32, iterations: i32) {
		let mut body = bodies.get_mut(self.body).expect("body should be in world"); // TODO: handle unwrap

		let radius = self.position_offset.rotate(Vec2::from_angle(body.angle));
		let position = body.position + radius;
		
		let ds = position - self.position;
		let dir = ds.normalize_or(Vec2::new(1.0, 0.0));
		let x1 = ds.length() - self.length;
		if !self.constrain_compression && x1 < 0.0 { return; } // don't eval constraint if less than max length

		let point_velocity = body.velocity + body.angular_velocity * radius.perp();
		let rel_vel = point_velocity.dot(dir);


		let zeta: f32 = self.damping; // damping ratio, zeta
		let omega: f32 = self.frequency; // oscillation frequency, omega

		let k = body.mass * omega.powf(2.0);
		let c = 2.0 * body.mass * omega * zeta;

		let gamma = 1.0 / (c + h*k);
		let beta = h*k * gamma;


		let mut impulse = -(rel_vel + beta / h * x1) / (body.inverse_mass + gamma / h);

		impulse /= iterations as f32;

		let p = impulse * dir;
		body.apply_impulse(p, position);
	}
}

impl ConstraintSolver for Spring {
	fn solve_velocity(&self, bodies: &mut Query<&mut Body>, delta_time: f32, iterations: i32) {
		self.solve_soft(bodies, delta_time, iterations);
	}
}