use bevy::prelude::*;

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

/*
// STIFF CONSTRAINT SOLVER
impl ConstraintSolver for FixedDistance {
	fn solve_velocity(&self, bodies: &mut Query<&mut Body>, delta_time: f32, iterations: i32) {
		let mut body = bodies.get_mut(self.body).expect("body should be in world"); // TODO: handle unwrap
		
		let radius = self.position_offset.rotate(Vec2::from_angle(body.angle));
		let body_position = body.position + radius;

		let ds = body_position - self.position;
		let dir = ds.normalize_or(Vec2::new(1.0, 0.0));
		
		let point_velocity = body.velocity + body.angular_velocity * radius.perp();
		let rel_vel = point_velocity.dot(dir);

		let impulse = -rel_vel; // velocity in dir should go to 0; i.e. impulse + rel_vel = 0; so impulse = -rel_vel
		let stiffness: f32 = 0.01;

		let p = impulse / iterations as f32 * dir;
		body.apply_impulse(p * stiffness.powf(delta_time * 10.0), body_position);
	}
	fn solve_position(&self, bodies: &mut Query<&mut Body>, delta_time: f32, iterations: i32) {
		// return;
		let mut body = bodies.get_mut(self.body).expect("body should be in world"); // TODO: handle unwrap
		
		let radius = self.position_offset.rotate(Vec2::from_angle(body.angle));
		let body_position = body.position + radius;

		let ds = body_position - self.position;
		let dir = ds.normalize_or(Vec2::new(1.0, 0.0));
		
		/*
			C = 0			definition of general constraint
			C = |s| - L,	definition of fixed distance constraint
			|s| = L; 		plugging in C = 0

			dir = s / |s|	definition of dir
			|s| = s / dir;	solve for |s|
			
			s / dir = L;	plug in |s| = s / dir
			[ s = L * dir ]	soln
		*/
		let position_stiffness: f32 = 0.01;
		let diff_len = (self.length - ds.length()) * position_stiffness.powf(delta_time * 10.0);
		let diff = diff_len * dir;
		body.translate_position(diff / (iterations as f32));
	}
}
*/
impl ConstraintSolver for FixedDistance {
	fn solve_velocity(&self, bodies: &mut Query<&mut Body>, h: f32, iterations: i32) {
		let mut body = bodies.get_mut(self.body).expect("body should be in world"); // TODO: handle unwrap

		let radius = self.position_offset.rotate(Vec2::from_angle(body.angle));
		let position = body.position + radius;
		
		let ds = position - self.position;
		let dir = ds.normalize_or(Vec2::new(1.0, 0.0));
		let x1 = ds.length() - self.length;

		let point_velocity = body.velocity + body.angular_velocity * radius.perp();
		let rel_vel = point_velocity.dot(dir);


		let zeta: f32 = 1.0; // damping ratio, zeta
		let omega: f32 = 40.0; // oscillation frequency, omega

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